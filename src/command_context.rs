use std::borrow::Cow;

use nu_plugin::EngineInterface;
use nu_protocol::{engine::Closure, IntoSpanned, LabeledError, ShellError, Spanned, Value};

use crate::nu_item::NuItem;

pub struct CommandContext {
    pub engine: EngineInterface,
    pub nu_config: nu_protocol::Config,
    pub format: MapperFlag,
    pub preview: MapperFlag,
}

impl CommandContext {
    pub fn new(engine: &EngineInterface) -> Result<Self, LabeledError> {
        Ok(Self {
            engine: engine.clone(),
            nu_config: *engine.get_config()?,
            format: MapperFlag::None,
            preview: MapperFlag::None,
        })
    }
}

pub enum MapperFlag {
    None,
    Closure(Spanned<Closure>),
}

impl TryFrom<Value> for MapperFlag {
    type Error = LabeledError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Closure { val, internal_span } => {
                Ok(Self::Closure((*val).into_spanned(internal_span)))
            }
            _ => Err(ShellError::CantConvert {
                to_type: "closure".to_owned(),
                from_type: value.get_type().to_string(),
                span: value.span(),
                help: None,
            }
            .into()),
        }
    }
}

impl MapperFlag {
    pub fn map<'a>(&self, item: &'a NuItem) -> Cow<'a, Value> {
        match self {
            MapperFlag::None => Cow::Borrowed(&item.value),
            MapperFlag::Closure(closure) => Cow::Owned(
                match item
                    .context
                    .engine
                    .eval_closure(closure, vec![], Some(item.value.clone()))
                {
                    Ok(ok) => ok,
                    Err(err) => Value::error(err, closure.span),
                },
            ),
        }
    }
}
