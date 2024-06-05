use std::borrow::Borrow;

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
                Type::List(Type::String.into()),
                Type::List(Type::String.into()),
            )
            .category(Category::Experimental)
    }

    fn usage(&self) -> &str {
        "(FIXME) help text for sk"
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "[ Ellie ] | sk",
            description: "Say hello to Ellie",
            result: Some(Value::test_list(vec![Value::test_string(
                "Hello, Ellie. How are you today?",
            )])),
        }]
    }

    fn run(
        &self,
        _plugin: &SkimPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        let pipeline_metadata = input.metadata();

        let skim_options = SkimOptionsBuilder::default()
            .build()
            .map_err(|err| LabeledError::new(err.to_string()))?;

        let (sender, receiver) = unbounded::<Arc<dyn SkimItem>>();

        for entry in input.into_iter() {
            sender.send(Arc::new(entry.as_str()?.to_owned())).unwrap();
        }

        let foreground = engine.enter_foreground()?;
        let selected = Skim::run_with(&skim_options, Some(receiver)).unwrap();
        let _ = foreground;
        let result = selected.selected_items[0].output();

        Ok(PipelineData::Value(
            Value::string(result, span),
            pipeline_metadata,
        ))
    }
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    // This will automatically run the examples specified in your command and compare their actual
    // output against what was specified in the example. You can remove this test if the examples
    // can't be tested this way, but we recommend including it if possible.

    PluginTest::new("skim", SkimPlugin.into())?.test_command_examples(&Sk)
}

fn main() {
    serve_plugin(&SkimPlugin, MsgPackSerializer);
}
