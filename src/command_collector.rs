use std::sync::Arc;

use nu_protocol::{engine::Closure, PipelineData, Span, Spanned, Value};
use skim::{prelude::unbounded, CommandCollector, SkimItem};

use crate::{command_context::CommandContext, nu_item::NuItem};

pub struct NuCommandCollector {
    pub context: Arc<CommandContext>,
    pub closure: Spanned<Closure>,
}

impl CommandCollector for NuCommandCollector {
    fn invoke(
        &mut self,
        cmd: &str, // not really the command - actually the query string
        components_to_stop: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    ) -> (
        skim::SkimItemReceiver,
        skim::prelude::Sender<i32>,
        Option<std::thread::JoinHandle<()>>,
    ) {
        let (tx, rx) = unbounded::<Arc<dyn SkimItem>>();
        let (tx_interrupt, rx_interrupt) = unbounded();
        let context = self.context.clone();
        let closure = self.closure.clone();
        let cmd = cmd.to_owned();
        (
            rx,
            tx_interrupt,
            Some(std::thread::spawn(move || {
                components_to_stop.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                match context.engine.eval_closure_with_stream(
                    &closure,
                    vec![Value::String {
                        val: cmd,
                        internal_span: Span::unknown(),
                    }],
                    PipelineData::Empty,
                    true,
                    true,
                ) {
                    Ok(PipelineData::ByteStream(stream, _)) => {
                        let span = stream.span();
                        if let Some(lines) = stream.lines() {
                            for line in lines {
                                if rx_interrupt.try_recv().is_ok() {
                                    break;
                                }
                                let send_result = match line {
                                    Ok(line) => tx.try_send(Arc::new(NuItem::new(
                                        context.clone(),
                                        Value::string(line, span),
                                    ))),
                                    Err(err) => tx.try_send(Arc::new(NuItem::new(
                                        context.clone(),
                                        Value::error(err, span),
                                    ))),
                                };
                                if send_result.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    Ok(stream) => {
                        for value in stream {
                            if rx_interrupt.try_recv().is_ok() {
                                break;
                            }
                            let send_result =
                                tx.try_send(Arc::new(NuItem::new(context.clone(), value)));
                            if send_result.is_err() {
                                break;
                            }
                        }
                    }
                    Err(err) => {
                        let _ = tx.try_send(Arc::new(NuItem::new(
                            context.clone(),
                            Value::error(err, Span::unknown()),
                        )));
                    }
                }

                components_to_stop.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            })),
        )
    }
}
