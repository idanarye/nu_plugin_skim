mod cli_arguments;
mod command_collector;
mod command_context;
mod nu_item;
mod predicate_based_selector;

use cli_arguments::CliArguments;
use command_collector::NuCommandCollector;
use command_context::CommandContext;
use nu_item::NuItem;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{
    Category, LabeledError, ListStream, PipelineData, Record, Signals, Signature, SyntaxShape,
    Type, Value,
};
use skim::prelude::*;
use skim::tui::event::Action;

pub struct SkimPlugin;

impl Plugin for SkimPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Sk)]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }
}

pub struct Sk;

impl PluginCommand for Sk {
    type Plugin = SkimPlugin;

    fn name(&self) -> &str {
        "sk"
    }

    fn signature(&self) -> Signature {
        let signature = {
            Signature::build(self.name())
                .input_output_type(Type::List(Type::Any.into()), Type::List(Type::Any.into()))
                .category(Category::Filters)
                .filter()
                .named(
                    "format",
                    SyntaxShape::Closure(Some(vec![])),
                    "Modify the string to display",
                    Some('f'),
                )
                .named(
                    "preview",
                    SyntaxShape::Closure(Some(vec![])),
                    "Generate a preview",
                    Some('p'),
                )
                .named(
                    "cmd",
                    SyntaxShape::Closure(Some(vec![SyntaxShape::String])),
                    "Command to invoke dynamically. A closure that receives the command query as its argument",
                    Some('c'),
                )
        };
        CliArguments::add_to_signature(signature)
    }

    fn description(&self) -> &str {
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

        let cli_arguments = CliArguments::new(call, engine)?;
        let mut skim_options = cli_arguments.to_skim_options();

        let mut command_context = CommandContext::new(engine)?;
        if let Some(format) = call.get_flag_value("format") {
            command_context.format = format.try_into()?;
        }

        if let Some(preview) = call.get_flag_value("preview") {
            command_context.preview = preview.try_into()?;
            skim_options.preview = Some("".to_owned());
        }

        let command_context = Arc::new(command_context);

        if let Some(closure) = call.get_flag("cmd")? {
            // This is a hack to make Skim conjure what it thinks is the actual command but is
            // actually just the query, which will be sent to as the `cmd` argument to
            // `NuCommandCollector.invoke`.
            skim_options.cmd = Some("{}".to_owned());
            skim_options.cmd_collector = Rc::new(RefCell::new(NuCommandCollector {
                context: command_context.clone(),
                closure,
            }))
        }

        let receiver = match input {
            PipelineData::Empty => {
                if skim_options.cmd.is_none() {
                    return Ok(PipelineData::empty());
                }
                None
            }
            PipelineData::Value(_, _) | PipelineData::ListStream(_, _) => {
                let (sender, receiver) = unbounded::<Vec<Arc<dyn SkimItem>>>();
                std::thread::spawn(move || {
                    for (index, entry) in input.into_iter().enumerate() {
                        if sender
                            .send(vec![Arc::new(NuItem::new(
                                index,
                                command_context.clone(),
                                entry,
                            ))])
                            .is_err()
                        {
                            // Assuming the receiver was closed because the user picked an item
                            return;
                        }
                    }
                });
                Some(receiver)
            }
            PipelineData::ByteStream(byte_stream, _) => {
                let Some(lines) = byte_stream.lines() else {
                    return Ok(PipelineData::empty());
                };
                let (sender, receiver) = unbounded::<Vec<Arc<dyn SkimItem>>>();
                std::thread::spawn(move || {
                    for (index, line) in lines.enumerate() {
                        if sender
                            .send(vec![Arc::new(NuItem::new(
                                index,
                                command_context.clone(),
                                match line {
                                    Ok(text) => Value::string(text, span),
                                    Err(err) => Value::error(err, span),
                                },
                            ))])
                            .is_err()
                        {
                            // Assuming the receiver was closed because the user picked an item
                            return;
                        }
                    }
                });
                Some(receiver)
            }
        };

        let _foreground = engine.enter_foreground()?;
        let option_expect_is_empty = skim_options.expect.is_empty();
        let option_multi = skim_options.multi;
        let skim_output = Skim::run_with(skim_options, receiver).unwrap();

        if skim_output.is_abort {
            return Ok(PipelineData::empty());
        }

        let mut result = skim_output.selected_items.into_iter().map(|item| {
            (*item.item)
                .as_any()
                .downcast_ref::<NuItem>()
                .unwrap()
                .value
                .clone()
        });
        if option_expect_is_empty {
            if option_multi {
                Ok(PipelineData::ListStream(
                    ListStream::new(result, span, Signals::EMPTY),
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
                if let Event::Action(Action::Accept(Some(action))) = skim_output.final_event {
                    Value::string(action, span)
                } else {
                    Value::nothing(span)
                },
            );

            record.push(
                "selected",
                if option_multi {
                    Value::list(result.collect(), span)
                } else if let Some(result) = result.next() {
                    result
                } else {
                    Value::nothing(span)
                },
            );

            Ok(PipelineData::Value(
                Value::record(record, span),
                pipeline_metadata,
            ))
        }
    }
}

fn main() {
    serve_plugin(&SkimPlugin, MsgPackSerializer);
}
