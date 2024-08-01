use nu_color_config::{StyleComputer, StyleMapping};
use nu_protocol::{
    engine::{EngineState, Stack},
    Signals, Span, Value,
};
use nu_table::{ExpandedTable, TableOpts};
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
        let fake_engine_state = EngineState::default();
        let fake_stack = Stack::default();
        let style_computer =
            StyleComputer::new(&fake_engine_state, &fake_stack, StyleMapping::default());
        let table_opts = TableOpts::new(
            &self.context.nu_config,
            &style_computer,
            &Signals::EMPTY, // TODO: actually send a signal when switching item? Is this necessary?
            Span::new(0, 0), // TODO: figure the correct span?
            context.width,
            (
                self.context.nu_config.table_indent.left,
                self.context.nu_config.table_indent.right,
            ),
            self.context.nu_config.table_mode,
            0,
            true,
        );
        let (string_result, _) =
            ExpandedTable::new(None, false, "".to_owned()).build_value(&preview_result, table_opts);
        ItemPreview::AnsiText(string_result)
    }
}
