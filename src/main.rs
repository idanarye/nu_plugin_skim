mod command_context;
mod nu_item;

use command_context::{CommandContext, FormatFlag};
use nu_item::NuItem;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{
    Category, IntoSpanned, LabeledError, PipelineData, Signature, SyntaxShape, Type,
};
use skim::prelude::*;

pub struct SkimPlugin;

impl Plugin for SkimPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            // Commands should be added here
            Box::new(Sk),
        ]
    }
}

pub struct Sk;

impl PluginCommand for Sk {
    type Plugin = SkimPlugin;

    fn name(&self) -> &str {
        "sk"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::List(Type::Any.into()), Type::List(Type::Any.into()))
            .category(Category::Experimental)
            .named(
                "format",
                SyntaxShape::Closure(Some(vec![])),
                "modify the string to display",
                None,
            )
    }

    fn usage(&self) -> &str {
        "Select a value using skim (a fuzzy finder written in Rust)"
    }

    fn run(
        &self,
        _plugin: &SkimPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let _span = call.head;

        let pipeline_metadata = input.metadata();

        let mut skim_options = SkimOptionsBuilder::default();
        let mut command_context = CommandContext::new(engine)?;
        if let Some(format) = call.get_flag_value("format") {
            command_context.format = match format {
                nu_protocol::Value::Closure { val, internal_span } => {
                    FormatFlag::Closure((*val).into_spanned(internal_span))
                }
                // nu_protocol::Value::CellPath { val, internal_span } => {
                // }
                format => {
                    return Err(LabeledError::new("Invalid format")
                        .with_label("not path nor closure", format.span()));
                }
            };
        }

        let skim_options = skim_options
            .build()
            .map_err(|err| LabeledError::new(err.to_string()))?;

        let command_context = Arc::new(command_context);

        let (sender, receiver) = unbounded::<Arc<dyn SkimItem>>();

        for entry in input.into_iter() {
            sender
                .send(Arc::new(NuItem {
                    value: entry,
                    context: command_context.clone(),
                }))
                .unwrap();
        }

        let foreground = engine.enter_foreground()?;
        let selected = Skim::run_with(&skim_options, Some(receiver)).unwrap();
        let _ = foreground;
        let result = (*selected.selected_items[0])
            .as_any()
            .downcast_ref::<NuItem>()
            .unwrap()
            .value
            .clone();

        Ok(PipelineData::Value(result, pipeline_metadata))
    }
}

fn main() {
    serve_plugin(&SkimPlugin, MsgPackSerializer);
}
