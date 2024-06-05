use nu_plugin::EngineInterface;
use nu_protocol::{engine::Closure, LabeledError, Spanned};

pub struct CommandContext {
    pub engine: EngineInterface,
    pub nu_config: nu_protocol::Config,
    pub format: FormatFlag,
}

impl CommandContext {
    pub fn new(engine: &EngineInterface) -> Result<Self, LabeledError> {
        Ok(Self {
            engine: engine.clone(),
            nu_config: *engine.get_config()?,
            format: FormatFlag::None,
        })
    }
}

pub enum FormatFlag {
    None,
    Closure(Spanned<Closure>),
}
