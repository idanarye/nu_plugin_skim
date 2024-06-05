use nu_protocol::Value;
use skim::prelude::*;

use crate::command_context::{CommandContext, FormatFlag};

pub struct NuItem {
    pub context: Arc<CommandContext>,
    pub value: Value
}

impl SkimItem for NuItem {
    fn text(&self) -> Cow<str> {
        let storage: Value;
        let value = match &self.context.format {
            FormatFlag::None => &self.value,
            FormatFlag::Path(cell_path) => {
                storage = self.value.clone().follow_cell_path(&cell_path.members, true).unwrap();
                &storage
            }
        };
        value.to_expanded_string(", ", &self.context.nu_config).into()
    }

    //fn preview(&self, _context: PreviewContext) -> ItemPreview {
        //ItemPreview::Global
    //}
}
