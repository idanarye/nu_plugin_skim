use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Signature};
use skim::SkimOptions;

pub struct CliArguments {
    //bind: Vec<String>,
    multi: bool,
    //prompt: Option<String>,
    //cmd_prompt: Option<String>,
    //expect: Option<String>,
    //tac: bool,
    //nosort: bool,
    //tiebreak: Option<String>,
    //exact: bool,
    //cmd: Option<String>,
    //interactive: bool,
    //query: Option<String>,
    //cmd_query: Option<String>,
    //regex: bool,
    //delimiter: Option<String>,
    //replstr: Option<String>,
    //color: Option<String>,
    //margin: Option<String>,
    //no_height: bool,
    //no_clear: bool,
    //no_clear_start: bool,
    //min_height: Option<String>,
    //height: Option<String>,
    //preview: Option<String>,
    //preview_window: Option<String>,
    //reverse: bool,
    //tabstop: Option<String>,
    //no_hscroll: bool,
    //no_mouse: bool,
    //inline_info: bool,
    //header: Option<String>,
    //header_lines: usize,
    //layout: String,
    //algorithm: FuzzyAlgorithm,
    //case: CaseMatching,
    //engine_factory: Option<Rc<dyn MatchEngineFactory>>,
    //query_history: &'a [String],
    //cmd_history: &'a [String],
    //cmd_collector: Rc<RefCell<dyn CommandCollector>>,
    //keep_right: bool,
    //skip_to_pattern: String,
    //select1: bool,
    //exit0: bool,
    sync: bool,
    //selector: Option<Rc<dyn Selector>>,
    //no_clear_if_empty: bool,
}

impl TryFrom<&EvaluatedCall> for CliArguments {
    type Error = LabeledError;

    fn try_from(call: &EvaluatedCall) -> Result<Self, Self::Error> {
        Ok(Self {
            multi: call.has_flag("multi")?,
            sync: call.has_flag("sync")?,
        })
    }
}

impl CliArguments {
    pub fn add_to_signature(signature: Signature) -> Signature {
        signature
            .switch("multi", "Select multiple values", Some('m'))
            .switch(
                "sync",
                "Wait for all the options to be available before choosing",
                None,
            )
    }

    pub fn to_skim_options(&self) -> SkimOptions {
        let Self { multi, sync } = self;
        SkimOptions {
            multi: *multi,
            sync: *sync,
            ..Default::default()
        }
    }
}
