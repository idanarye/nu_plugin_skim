use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Signature};
use skim::SkimOptions;

#[allow(clippy::needless_question_mark)]
pub fn do_bridging<B: BridgeCliArguments>(bridge: B) -> Result<B, LabeledError> {
    Ok(bridge
        .switch("multi", "Select multiple values", Some('m'), |opt| {
            &mut opt.multi
        })?
        .switch(
            "sync",
            "Wait for all the options to be available before choosing",
            None,
            |opt| &mut opt.sync,
        )?)
}

pub trait BridgeCliArguments: Sized {
    fn switch(
        self,
        name: &str,
        desc: &str,
        short: Option<char>,
        get_option_mut: impl for<'a> FnOnce(&'a mut SkimOptions) -> &'a mut bool,
    ) -> Result<Self, LabeledError>;
}

pub struct BridgeNuSignature(pub Signature);

impl BridgeCliArguments for BridgeNuSignature {
    fn switch(
        self,
        name: &str,
        desc: &str,
        short: Option<char>,
        _get_option_mut: impl for<'a> FnOnce(&'a mut SkimOptions) -> &'a mut bool,
    ) -> Result<Self, LabeledError> {
        Ok(Self(self.0.switch(name, desc, short)))
    }
}

pub struct BridgeSkimSide<'a, 'b> {
    pub call: &'a EvaluatedCall,
    pub skim_options: &'b mut SkimOptions<'a>,
}

impl BridgeCliArguments for BridgeSkimSide<'_, '_> {
    fn switch(
        self,
        name: &str,
        _desc: &str,
        _short: Option<char>,
        get_option_mut: impl for<'a> FnOnce(&'a mut SkimOptions) -> &'a mut bool,
    ) -> Result<Self, LabeledError> {
        *get_option_mut(self.skim_options) = self.call.has_flag(name)?;
        Ok(self)
    }
}
