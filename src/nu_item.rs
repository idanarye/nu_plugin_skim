use nu_protocol::Value;
use skim::prelude::*;

use crate::command_context::CommandContext;

pub struct NuItem {
    pub context: Arc<CommandContext>,
    pub value: Value
}

impl SkimItem for NuItem {
    fn text(&self) -> Cow<str> {
        self.value.to_expanded_string(", ", &self.context.as_ref().engine.get_config().unwrap()).into()
    }

    //fn preview(&self, _context: PreviewContext) -> ItemPreview {
        //ItemPreview::Global
    //}
}
