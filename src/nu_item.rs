use nu_plugin::EvaluatedCall;
use nu_protocol::{IntoSpanned, PipelineData, ShellError, Span, Value};
use skim::prelude::*;

use crate::command_context::CommandContext;

pub struct NuItem {
    pub context: Arc<CommandContext>,
    pub value: Value,
}

impl SkimItem for NuItem {
    fn text(&self) -> Cow<str> {
        self.context
            .format
            .map(self)
            .to_expanded_string(", ", &self.context.nu_config)
            .into()
    }

    fn preview(&self, context: PreviewContext) -> ItemPreview {
        let preview_result = self.context.preview.map(self);
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
