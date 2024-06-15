use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Record, Signature, SyntaxShape, Value};
use skim::SkimOptions;

pub struct CliArguments {
    bind: Vec<String>,
    multi: bool,
    prompt: Option<String>,
    //cmd_prompt: Option<String>,
    expect: Option<String>,
    tac: bool,
    nosort: bool,
    tiebreak: Option<String>,
    exact: bool,
    //cmd: Option<String>,
    //interactive: bool,
    //query: Option<String>,
    //cmd_query: Option<String>,
    regex: bool,
    //delimiter: Option<String>,
    //replstr: Option<String>,
    color: Option<String>,
    margin: Option<String>,
    //no_height: bool,
    no_clear: bool,
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
        fn to_comma_separated_list(
            call: &EvaluatedCall,
            flag_name: &str,
        ) -> Result<Option<String>, LabeledError> {
            if let Some(flag_value) = call.get_flag::<Vec<Value>>(flag_name)? {
                let mut result = String::new();
                for key in flag_value.iter() {
                    let key = key.coerce_str()?;
                    if !result.is_empty() {
                        result.push(',');
                    }
                    result.push_str(&key);
                }
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        Ok(Self {
            bind: if let Some(bind) = call.get_flag::<Record>("bind")? {
                bind.iter()
                    .map(|(key, value)| {
                        let value = value.coerce_string()?;
                        Ok(format!("{key}:{value}"))
                    })
                    .collect::<Result<Vec<String>, LabeledError>>()?
            } else {
                Vec::default()
            },
            multi: call.has_flag("multi")?,
            prompt: call.get_flag("prompt")?,
            expect: to_comma_separated_list(call, "expect")?,
            tac: call.has_flag("tac")?,
            nosort: call.has_flag("no-sort")?,
            tiebreak: to_comma_separated_list(call, "tiebreak")?,
            exact: call.has_flag("exact")?,
            regex: call.has_flag("regex")?,
            color: call.get_flag("color")?,
            margin: call.get_flag("margin")?,
            no_clear: call.has_flag("no-clear")?,
            sync: call.has_flag("sync")?,
        })
    }
}

impl CliArguments {
    pub fn add_to_signature(signature: Signature) -> Signature {
        signature
            .named(
                "bind",
                SyntaxShape::Record(Vec::default()),
                "Custom key bindings. A record where the keys arae keymaps and the values are actions",
                None,
            )
            .switch("multi", "Select multiple values", Some('m'))
            .named("prompt", SyntaxShape::String, "Input prompt", None)
            .named(
                "expect",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "List of keys that can be used to complete sk in addition to the default enter key",
                None,
            )
            .switch("tac", "Reverse  the  order  of  the search result (normally used together with --no-sort)", None)
            .switch("no-sort", "Do not sort the search result (normally used together with --tac)", None)
            .named(
                "tiebreak",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "List of sort criteria to apply  when  the  scores are tied.",
                None,
            )
            .switch(
                "exact",
                "Enable exact-match",
                Some('e'),
            )
            .switch(
                "regex",
                "Search with regular expression instead of fuzzy match",
                None,
            )
            .named("color", SyntaxShape::String, "Color configuration", None)
            .named("margin", SyntaxShape::String, "Comma-separated expression for margins around the finder.", None)
            .switch(
                "no-clear",
                "Do not clear finder interface on exit",
                 None,
            )
            .switch(
                "sync",
                "Wait for all the options to be available before choosing",
                None,
            )
    }

    pub fn to_skim_options(&self) -> SkimOptions {
        let Self {
            bind,
            multi,
            prompt,
            expect,
            tac,
            nosort,
            tiebreak,
            exact,
            regex,
            color,
            margin,
            no_clear,
            sync,
        } = self;

        SkimOptions {
            bind: bind.iter().map(|b| b.as_str()).collect(),
            multi: *multi,
            prompt: prompt.as_deref(),
            expect: expect.clone(),
            tac: *tac,
            nosort: *nosort,
            tiebreak: tiebreak.clone(),
            exact: *exact,
            regex: *regex,
            color: color.as_deref(),
            margin: margin.as_deref().or(Some("0,0,0,0")),
            no_clear: *no_clear,
            sync: *sync,
            ..Default::default()
        }
    }
}
