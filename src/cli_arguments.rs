use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Record, ShellError, Signature, SyntaxShape, Value};
use skim::{CaseMatching, FuzzyAlgorithm, SkimOptions};

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
    min_height: Option<String>,
    height: Option<String>,
    //preview: Option<String>,
    preview_window: Option<String>,
    reverse: bool, // note that this does not (just) get paseed to CliArguments as is - it's there to modify --layout
    tabstop: Option<String>,
    no_hscroll: bool,
    no_mouse: bool,
    inline_info: bool,
    //header: Option<String>,
    //header_lines: usize,
    layout: Option<String>,
    algorithm: FuzzyAlgorithm,
    case: CaseMatching,
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
            min_height: call
                .get_flag::<i64>("min-height")?
                .map(|num| num.to_string()),
            height: call.get_flag("height")?,
            preview_window: call.get_flag("preview-window")?,
            reverse: call.has_flag("reverse")?,
            tabstop: call.get_flag::<i64>("tabstop")?.map(|num| num.to_string()),
            no_hscroll: call.has_flag("no-hscroll")?,
            no_mouse: call.has_flag("no-mouse")?,
            inline_info: call.has_flag("inline-info")?,
            layout: call.get_flag("layout")?,
            algorithm: call
                .get_flag::<String>("algo")?
                .as_deref()
                .map(|flag| match flag {
                    "skim_v1" => Ok(FuzzyAlgorithm::SkimV1),
                    "skim_v2" => Ok(FuzzyAlgorithm::SkimV2),
                    "clangd" => Ok(FuzzyAlgorithm::Clangd),
                    _ => Err(ShellError::UnsupportedConfigValue {
                        expected: "skim_v1|skim_v2|clangd".to_owned(),
                        value: flag.to_owned(),
                        span: call
                            .get_flag_value("algo")
                            .expect("we already know the flag exists")
                            .span(),
                    }),
                })
                .transpose()?
                .unwrap_or_default(),
            case: call
                .get_flag::<String>("case")?
                .as_deref()
                .map(|flag| match flag {
                    "smart" => Ok(CaseMatching::Smart),
                    "ignore" => Ok(CaseMatching::Ignore),
                    "respect" => Ok(CaseMatching::Respect),
                    _ => Err(ShellError::UnsupportedConfigValue {
                        expected: "[smart|ignore|respect]".to_owned(),
                        value: flag.to_owned(),
                        span: call
                            .get_flag_value("case")
                            .expect("we already know the flag exists")
                            .span(),
                    }),
                })
                .transpose()?
                .unwrap_or_default(),
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
            .named(
                "height",
                SyntaxShape::String,
                "Display sk window below the cursor with the given height instead of using the full screen",
                None,
            )
            .named(
                "min-height",
                SyntaxShape::Number,
                "Minimum height when --height is given in percent. Ignored when --height is not specified",
                None,
            )
            .named(
                "preview-window",
                SyntaxShape::String,
                "Determines the layout of the preview window",
                None,
            )
            .switch(
                "reverse",
                "A synonym for --layout=reverse",
                None,
            )
            .named(
                "tabstop",
                SyntaxShape::Number,
                "Number of spaces for a tab character",
                None,
            )
            .switch(
                "no-hscroll",
                "Disable horizontal scroll",
                None,
            )
            .switch(
                "no-mouse",
                "Disable mouse",
                None,
            )
            .switch(
                "inline-info",
                "Display the finder info after the prompt with the default prefix ' < '",
                None,
            )
            .named(
                "layout",
                SyntaxShape::String,
                "Choose the layout",
                None,
            )
            .named(
                "algo",
                SyntaxShape::String,
                "Fuzzy matching algorithm: [skim_v1|skim_v2|clangd] (default: skim_v2)",
                None,
            )
            .named(
                "case",
                SyntaxShape::String,
                "Case sensitivity: [smart|ignore|respect] (default: smart)",
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
            min_height,
            height,
            preview_window,
            reverse,
            tabstop,
            no_hscroll,
            no_mouse,
            inline_info,
            layout,
            algorithm,
            case,
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
            min_height: min_height.as_deref().or(Some("10")),
            height: height.as_deref().or(Some("100%")),
            preview_window: preview_window.as_deref().or(Some("right:50%")),
            reverse: *reverse,
            tabstop: tabstop.as_deref(),
            no_hscroll: *no_hscroll,
            no_mouse: *no_mouse,
            inline_info: *inline_info,
            layout: if *reverse {
                "reverse"
            } else {
                layout.as_deref().unwrap_or("default")
            },
            algorithm: *algorithm,
            case: *case,
            sync: *sync,
            ..Default::default()
        }
    }
}
