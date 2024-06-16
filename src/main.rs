mod cli_arguments;
mod command_context;
mod nu_item;

use cli_arguments::CliArguments;
use command_context::CommandContext;
use nu_item::NuItem;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{
    Category, LabeledError, ListStream, PipelineData, Record, Signature, SyntaxShape, Type, Value,
};
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
        let signature = Signature::build(self.name())
            .input_output_type(Type::List(Type::Any.into()), Type::List(Type::Any.into()))
            .category(Category::Filters)
            .filter()
            .named(
                "format",
                SyntaxShape::Closure(Some(vec![])),
                "Modify the string to display",
                None,
            )
            .named(
                "preview",
                SyntaxShape::Closure(Some(vec![])),
                "Generate a preview",
                None,
            );
        CliArguments::add_to_signature(signature)
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

        let cli_arguments = CliArguments::try_from(call)?;
        let mut skim_options = cli_arguments.to_skim_options();

        let mut command_context = CommandContext::new(engine)?;
        if let Some(format) = call.get_flag_value("format") {
            command_context.format = format.try_into()?;
        }

        if let Some(preview) = call.get_flag_value("preview") {
            command_context.preview = preview.try_into()?;
            skim_options.preview = Some("");
        }

        let command_context = Arc::new(command_context);

        let (sender, receiver) = unbounded::<Arc<dyn SkimItem>>();

        match input {
            PipelineData::Empty => {
                return Ok(PipelineData::empty());
            }
            PipelineData::Value(_, _) | PipelineData::ListStream(_, _) => {
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
            }
            PipelineData::ByteStream(byte_stream, _) => {
                let Some(lines) = byte_stream.lines() else {
                    return Ok(PipelineData::empty());
                };
                std::thread::spawn(move || {
                    for line in lines {
                        if sender
                            .send(Arc::new(NuItem {
                                value: match line {
                                    Ok(text) => Value::string(text, span),
                                    Err(err) => Value::error(err, span),
                                },
                                context: command_context.clone(),
                            }))
                            .is_err()
                        {
                            // Assuming the receiver was closed because the user picked an item
                            return;
                        }
                    }
                });
            }
        }

        let _foreground = engine.enter_foreground()?;
        let skim_output = Skim::run_with(&skim_options, Some(receiver)).unwrap();

        if skim_output.is_abort {
            return Ok(PipelineData::empty());
        }

        let mut result = skim_output.selected_items.into_iter().map(|item| {
            (*item)
                .as_any()
                .downcast_ref::<NuItem>()
                .unwrap()
                .value
                .clone()
        });
        if skim_options.expect.is_none() {
            if skim_options.multi {
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
        } else {
            let mut record = Record::new();
            record.push(
                "action",
                if let Event::EvActAccept(Some(action)) = skim_output.final_event {
                    Value::string(action, span)
                } else {
                    Value::Nothing {
                        internal_span: span,
                    }
                },
            );

            record.push(
                "selected",
                if skim_options.multi {
                    Value::list(result.collect(), span)
                } else if let Some(result) = result.next() {
                    result
                } else {
                    Value::nothing(span)
                },
            );

            Ok(PipelineData::Value(
                Value::Record {
                    val: record.into(),
                    internal_span: span,
                },
                pipeline_metadata,
            ))
        }
    }
}

fn main() {
    serve_plugin(&SkimPlugin, MsgPackSerializer);
}
