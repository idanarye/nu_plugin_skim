use std::sync::Arc;

use nu_protocol::{engine::Closure, PipelineData, Span, Spanned, Value};
use skim::reader::CommandCollector;
use skim::{prelude::unbounded, SkimItem};

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
    ) -> (skim::SkimItemReceiver, skim::prelude::Sender<i32>) {
        let (tx, rx) = unbounded::<Vec<Arc<dyn SkimItem>>>();
        let (tx_interrupt, mut rx_interrupt) = unbounded();
        let context = self.context.clone();
        let closure = self.closure.clone();
        let cmd = cmd.to_owned();
        std::thread::spawn(move || {
            components_to_stop.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

            match context.engine.eval_closure_with_stream(
                &closure,
                vec![Value::string(cmd, Span::unknown())],
                PipelineData::Empty,
                true,
                true,
            ) {
                Ok(PipelineData::ByteStream(stream, _)) => {
                    let span = stream.span();
                    if let Some(lines) = stream.lines() {
                        for (index, line) in lines.enumerate() {
                            if rx_interrupt.try_recv().is_ok() {
                                break;
                            }
                            let send_result = match line {
                                Ok(line) => tx.send(vec![Arc::new(NuItem::new(
                                    index,
                                    context.clone(),
                                    Value::string(line, span),
                                ))]),
                                Err(err) => tx.send(vec![Arc::new(NuItem::new(
                                    index,
                                    context.clone(),
                                    Value::error(err, span),
                                ))]),
                            };
                            if send_result.is_err() {
                                break;
                            }
                        }
                    }
                }
                Ok(stream) => {
                    for (index, value) in stream.into_iter().enumerate() {
                        if rx_interrupt.try_recv().is_ok() {
                            break;
                        }
                        let send_result =
                            tx.send(vec![Arc::new(NuItem::new(index, context.clone(), value))]);
                        if send_result.is_err() {
                            break;
                        }
                    }
                }
                Err(err) => {
                    let _ = tx.send(vec![Arc::new(NuItem::new(
                        0,
                        context.clone(),
                        Value::error(err, Span::unknown()),
                    ))]);
                }
            }

            components_to_stop.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        });
        (rx, tx_interrupt)
    }
}
