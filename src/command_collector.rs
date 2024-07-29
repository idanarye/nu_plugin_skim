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
        cmd: &str,
        components_to_stop: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    ) -> (
        skim::SkimItemReceiver,
        skim::prelude::Sender<i32>,
        Option<std::thread::JoinHandle<()>>,
    ) {
        // use std::fs;
        // use std::io::Write;
        // let mut file = fs::File::options()
        // .create(true)
        // .append(true)
        // .open("/tmp/sklog.log")
        // .unwrap();
        // writeln!(&mut file, "inside invoke {:?}", cmd).unwrap();
        let (tx, rx) = unbounded::<Arc<dyn SkimItem>>();
        let (tx_interrupt, _rx_interrupt) = unbounded();
        let context = self.context.clone();
        let closure = self.closure.clone();
        let cmd = cmd.to_owned();
        (
            rx,
            tx_interrupt,
            Some(std::thread::spawn(move || {
                components_to_stop.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                if let Ok(ok) = context.engine.eval_closure_with_stream(
                    &closure,
                    vec![Value::String {
                        val: cmd,
                        internal_span: Span { start: 0, end: 0 },
                    }],
                    PipelineData::Empty,
                    true,
                    true,
                ) {
                    match ok {
                        PipelineData::Empty => {}
                        PipelineData::Value(value, _) => {
                            tx.send(Arc::new(NuItem { context, value })).unwrap();
                        }
                        PipelineData::ListStream(_, _) => todo!(),
                        PipelineData::ByteStream(_, _) => todo!(),
                    }
                }

                // let rcv = rx_interrupt.recv();
                // writeln!(&mut file, "rcv {:?}", rcv).unwrap();
                components_to_stop.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                // writeln!(&mut file, "finished thread of {:?}", cmd).unwrap();
            })),
        )
    }
}
