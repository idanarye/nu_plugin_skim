use nu_plugin::EvaluatedCall;
use nu_protocol::{IntoSpanned, PipelineData, ShellError, Span, Value};
use skim::prelude::*;

use crate::command_context::CommandContext;

pub struct NuItem {
    pub context: Arc<CommandContext>,
    pub value: Value,
    pub display: AnsiString<'static>,
}

impl NuItem {
    pub fn new(context: Arc<CommandContext>, value: Value) -> Self {
        let display = AnsiString::parse(
            &context
                .format
                .map(&context, &value)
                .to_expanded_string(", ", &context.nu_config),
        );
        Self {
            context,
            value,
            display,
        }
    }
}

impl SkimItem for NuItem {
    fn text(&self) -> Cow<str> {
        self.display.stripped().to_owned().into()
    }

    fn display(&self, _context: DisplayContext) -> AnsiString {
        self.display.clone()
    }

    fn preview(&self, context: PreviewContext) -> ItemPreview {
        let preview_result = self.context.preview.map(&self.context, &self.value);
        if let Ok(preview_result) = preview_result.coerce_string() {
            return ItemPreview::AnsiText(preview_result);
        }
        let result = self
            .context
            .engine
            .find_decl("table")
            .and_then(|table_decl| {
                let table_decl = table_decl.ok_or_else(|| ShellError::GenericError {
                    error: "`table` decl is empty".to_owned(),
                    msg: "`table` decl is empty".to_owned(),
                    span: None,
                    help: None,
                    inner: vec![],
                })?;
                let as_table = self.context.engine.call_decl(
                    table_decl,
                    // TODO: get the actual span
                    EvaluatedCall::new(Span::unknown()).with_named(
                        "width".into_spanned(Span::unknown()),
                        Value::int(context.width as i64, Span::unknown()),
                    ),
                    PipelineData::Value((*preview_result).clone(), None),
                    true,
                    false,
                )?;
                let as_table_text = as_table.collect_string("\n", &self.context.nu_config)?;
                Ok(as_table_text)
            });
        match result {
            Ok(text) => ItemPreview::AnsiText(text),
            Err(err) => ItemPreview::AnsiText(err.to_string()),
        }
    }
}
