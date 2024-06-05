use nu_protocol::Value;
use skim::prelude::*;

use crate::command_context::{CommandContext, FormatFlag};

pub struct NuItem {
    pub context: Arc<CommandContext>,
    pub value: Value,
}

impl SkimItem for NuItem {
    fn text(&self) -> Cow<str> {
        let storage: Value;
        let value = match &self.context.format {
            FormatFlag::None => &self.value,
            FormatFlag::Closure(closure) => {
                storage = match self.context.engine.eval_closure(
                    closure,
                    vec![],
                    Some(self.value.clone()),
                ) {
                    Ok(ok) => ok,
                    Err(err) => return err.to_string().into(),
                };
                &storage
            }
        };
        value
            .to_expanded_string(", ", &self.context.nu_config)
            .into()
    }

    //fn preview(&self, _context: PreviewContext) -> ItemPreview {
    //ItemPreview::Global
    //}
}
