use nu_protocol::Value;
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

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(
            self.context.preview.map(self).to_expanded_string(", ", &self.context.nu_config)
        )
    }
}
