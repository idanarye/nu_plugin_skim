mod nu_item;
mod command_context;

use command_context::CommandContext;
use nu_item::NuItem;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, Type, Value};
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
            .input_output_type(
                Type::List(Type::Any.into()),
                Type::List(Type::Any.into()),
            )
            .category(Category::Experimental)
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

        let skim_options = SkimOptionsBuilder::default()
            .build()
            .map_err(|err| LabeledError::new(err.to_string()))?;

        let command_context = Arc::new(CommandContext {
            engine: engine.clone(),
        });

        let (sender, receiver) = unbounded::<Arc<dyn SkimItem>>();

        for entry in input.into_iter() {
            sender.send(Arc::new(NuItem {
                value: entry,
                context: command_context.clone(),
            })).unwrap();
        }

        let foreground = engine.enter_foreground()?;
        let selected = Skim::run_with(&skim_options, Some(receiver)).unwrap();
        let _ = foreground;
        let result = (*selected.selected_items[0]).as_any().downcast_ref::<NuItem>().unwrap().value.clone();

        Ok(PipelineData::Value(result, pipeline_metadata))
    }
}

fn main() {
    serve_plugin(&SkimPlugin, MsgPackSerializer);
}
