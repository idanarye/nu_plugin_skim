mod command_context;
mod nu_item;

use command_context::CommandContext;
use nu_item::NuItem;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{Category, LabeledError, ListStream, PipelineData, Signature, SyntaxShape, Type};
use skim::prelude::*;

pub struct SkimPlugin;

impl Plugin for SkimPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Sk)]
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
            .named(
                "preview",
                SyntaxShape::Closure(Some(vec![])),
                "generate a preview",
                None,
            )
            .switch("multi", "Select multiple values", Some('m'))
            .switch(
                "sync",
                "Wait for all the options to be available before choosing",
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
        let span = call.head;

        let pipeline_metadata = input.metadata();

        let mut skim_options = SkimOptionsBuilder::default();
        let mut command_context = CommandContext::new(engine)?;
        if let Some(format) = call.get_flag_value("format") {
            command_context.format = format.try_into()?;
        }

        if let Some(preview) = call.get_flag_value("preview") {
            command_context.preview = preview.try_into()?;
            skim_options.preview(Some(""));
        }

        let multi = call.has_flag("multi")?;
        skim_options.multi(multi);

        skim_options.sync(call.has_flag("sync")?);

        let skim_options = skim_options
            .build()
            .map_err(|err| LabeledError::new(err.to_string()))?;

        let command_context = Arc::new(command_context);

        let (sender, receiver) = unbounded::<Arc<dyn SkimItem>>();

        std::thread::spawn(move || {
            for entry in input.into_iter() {
                if sender
                    .send(Arc::new(NuItem {
                        value: entry,
                        context: command_context.clone(),
                    }))
                    .is_err()
                {
                    // Assuming the receiver was closed because the user picked an item
                    return;
                }
            }
        });

        let _foreground = engine.enter_foreground()?;
        let selected = Skim::run_with(&skim_options, Some(receiver)).unwrap();

        let mut result = selected.selected_items.into_iter().map(|item| {
            (*item)
                .as_any()
                .downcast_ref::<NuItem>()
                .unwrap()
                .value
                .clone()
        });
        if multi {
            Ok(PipelineData::ListStream(
                ListStream::new(result, span, None),
                pipeline_metadata,
            ))
        } else {
            Ok(if let Some(result) = result.next() {
                PipelineData::Value(result, pipeline_metadata)
            } else {
                PipelineData::empty()
            })
        }
    }
}

fn main() {
    serve_plugin(&SkimPlugin, MsgPackSerializer);
}
