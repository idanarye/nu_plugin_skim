use nu_plugin::EngineInterface;
use nu_protocol::{engine::Closure, PipelineData, Spanned};
use skim::prelude::*;

use crate::nu_item::NuItem;

pub struct PredicateBasedSelector {
    pub engine: EngineInterface,
    pub predicate: Spanned<Closure>,
}

impl Selector for PredicateBasedSelector {
    fn should_select(&self, _index: usize, item: &dyn SkimItem) -> bool {
        let Some(nu_item) = item.as_any().downcast_ref::<NuItem>() else {
            return false;
        };
        let Ok(result) = self.engine.eval_closure_with_stream(
            &self.predicate,
            vec![],
            PipelineData::Value(nu_item.value.clone(), None),
            true,
            true,
        ) else {
            return false;
        };
        match result {
            PipelineData::Value(value, _) => value.is_true(),
            _ => false,
        }
    }
}

pub struct CombinedSelector(pub DefaultSkimSelector, pub PredicateBasedSelector);

impl Selector for CombinedSelector {
    fn should_select(&self, index: usize, item: &dyn SkimItem) -> bool {
        self.0.should_select(index, item) || self.1.should_select(index, item)
    }
}
