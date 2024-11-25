use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    rc::Rc,
};

use clap::ValueEnum;
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{
    engine::Closure, LabeledError, Record, ShellError, Signature, Spanned, SyntaxShape,
};
use skim::{
    prelude::DefaultSkimSelector, CaseMatching, FuzzyAlgorithm, RankCriteria, Selector, SkimOptions,
};

use crate::predicate_based_selector::{CombinedSelector, PredicateBasedSelector};

pub struct CliArguments {
    bind: Vec<String>,
    multi: bool,
    prompt: Option<String>,
    cmd_prompt: Option<String>,
    expect: Vec<String>,
    tac: bool,
    no_sort: bool,
    tiebreak: Vec<RankCriteria>,
    exact: bool,
    //cmd: Option<Closure>,
    interactive: bool,
    query: Option<String>,
    cmd_query: Option<String>,
    regex: bool,
    //delimiter: Option<String>,
    //replstr: Option<String>,
    color: Option<String>,
    margin: Option<String>,
    no_height: bool,
    no_clear: bool,
    no_clear_start: bool,
    min_height: Option<String>,
    height: Option<String>,
    //preview: Option<String>,
    preview_window: Option<String>,
    reverse: bool, // note that this does not (just) get paseed to CliArguments as is - it's there to modify --layout
    tabstop: Option<usize>,
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
    keep_right: bool,
    skip_to_pattern: Option<String>,
    select1: bool,
    exit0: bool,
    sync: bool,
    selector: Option<Rc<dyn Selector>>,
    no_clear_if_empty: bool,
}

impl CliArguments {
    #[allow(clippy::result_large_err)]
    pub fn new(call: &EvaluatedCall, engine: &EngineInterface) -> Result<Self, LabeledError> {
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
            cmd_prompt: call.get_flag("cmd-prompt")?,
            expect: call.get_flag("expect")?.unwrap_or_default(),
            tac: call.has_flag("tac")?,
            no_sort: call.has_flag("no-sort")?,
            tiebreak: call
                .get_flag::<Vec<Spanned<String>>>("tiebreak")?
                .unwrap_or_default()
                .into_iter()
                .map(|flag| {
                    RankCriteria::from_str(&flag.item, true).map_err(|_| {
                        let possible_values = RankCriteria::value_variants()
                            .iter()
                            .flat_map(|v| Some(format!("`{}`", v.to_possible_value()?.get_name())))
                            .collect::<Vec<_>>()
                            .join("/");
                        LabeledError::new(format!(
                            "Invalid tiebreak - legal options are {possible_values}"
                        ))
                        .with_label("here", flag.span)
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
            exact: call.has_flag("exact")?,
            interactive: call.has_flag("interactive")?,
            query: call.get_flag("query")?,
            cmd_query: call.get_flag("cmd-query")?,
            regex: call.has_flag("regex")?,
            color: call.get_flag("color")?,
            margin: call.get_flag("margin")?,
            no_height: call.has_flag("no-height")?,
            no_clear: call.has_flag("no-clear")?,
            no_clear_start: call.has_flag("no-clear-start")?,
            min_height: call
                .get_flag::<i64>("min-height")?
                .map(|num| num.to_string()),
            height: call.get_flag("height")?,
            preview_window: call.get_flag("preview-window")?,
            reverse: call.has_flag("reverse")?,
            tabstop: call.get_flag::<usize>("tabstop")?,
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
                    _ => Err(ShellError::InvalidValue {
                        valid: "[skim_v1|skim_v2|clangd]".to_owned(),
                        actual: flag.to_owned(),
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
                    _ => Err(ShellError::InvalidValue {
                        valid: "[smart|ignore|respect]".to_owned(),
                        actual: flag.to_owned(),
                        span: call
                            .get_flag_value("case")
                            .expect("we already know the flag exists")
                            .span(),
                    }),
                })
                .transpose()?
                .unwrap_or_default(),
            keep_right: call.has_flag("keep-right")?,
            skip_to_pattern: call.get_flag("skip-to-pattern")?,
            select1: call.has_flag("select-1")?,
            exit0: call.has_flag("exit-0")?,
            sync: call.has_flag("sync")?,
            selector: {
                let mut dumb_selector: Option<DefaultSkimSelector> = None;
                if let Some(n) = call.get_flag::<usize>("pre-select-n")? {
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().first_n(n));
                }
                if let Some(pat) = call.get_flag::<String>("pre-select-pat")? {
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().regex(&pat));
                }
                if let Some(items) = call.get_flag::<Vec<String>>("pre-select-items")? {
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().preset(items));
                }
                if let Some(file_path) = call.get_flag::<Spanned<PathBuf>>("pre-select-file")? {
                    let file = File::open(file_path.item).map_err(|e| {
                        LabeledError::new(e.to_string()).with_label("here", file_path.span)
                    })?;
                    let items = BufReader::new(file)
                        .lines()
                        .collect::<Result<Vec<String>, _>>()
                        .map_err(|e| LabeledError::new(e.to_string()))?;
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().preset(items));
                }
                if let Some(predicate) = call.get_flag::<Spanned<Closure>>("pre-select")? {
                    let predicate_based_selector = PredicateBasedSelector {
                        engine: engine.clone(),
                        predicate,
                    };
                    if let Some(dumb_selector) = dumb_selector {
                        Some(Rc::new(CombinedSelector(
                            dumb_selector,
                            predicate_based_selector,
                        )))
                    } else {
                        Some(Rc::new(predicate_based_selector))
                    }
                } else if let Some(dumb_selector) = dumb_selector {
                    Some(Rc::new(dumb_selector))
                } else {
                    None
                }
            },
            no_clear_if_empty: call.has_flag("no-clear-if-empty")?,
        })
    }

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
            .named("cmd-prompt", SyntaxShape::String, "Command mode prompt", None)
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
            .switch("interactive", "Start skim in interactive(command) mode", Some('i'))
            .named(
                "query",
                SyntaxShape::String,
                "Specify the initial query",
                Some('q'),
            )
            .named(
                "cmd-query",
                SyntaxShape::String,
                "Specify the initial query for interactive mode",
                None,
            )
            .switch(
                "regex",
                "Search with regular expression instead of fuzzy match",
                None,
            )
            .named("color", SyntaxShape::String, "Color configuration", None)
            .named("margin", SyntaxShape::String, "Comma-separated expression for margins around the finder.", None)
            .switch(
                "no-height",
                "Disable height feature",
                 None,
            )
            .switch(
                "no-clear",
                "Do not clear finder interface on exit",
                 None,
            )
            .switch(
                "no-clear-start",
                "Do not clear on start",
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
                "keep-right",
                "Keep the right end of the line visible when it's too long",
                None,
            )
            .named(
                "skip-to-pattern",
                SyntaxShape::String,
                "Line will start with the start of the matched pattern",
                None,
            )
            .switch(
                "select-1",
                "Automatically select the only match",
                Some('1'),
            )
            .switch(
                "exit-0",
                "Exit immediately when there's no match",
                Some('0'),
            )
            .switch(
                "sync",
                "Wait for all the options to be available before choosing",
                None,
            )
            .named(
                "pre-select-n",
                SyntaxShape::Number,
                "Pre-select the first n items in multi-selection mode",
                None,
            )
            .named(
                "pre-select-pat",
                SyntaxShape::String,
                "Pre-select the matched items in multi-selection mode",
                None,
            )
            .named(
                "pre-select-items",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "Pre-select the items separated by newline character",
                None,
            )
            .named(
                "pre-select-file",
                SyntaxShape::Filepath,
                "Pre-select the items read from file",
                None,
            )
            .named(
                "pre-select",
                SyntaxShape::Closure(Some(vec![])),
                "Pre-select the items that match the predicate",
                None,
            )
            .switch(
                "no-clear-if-empty",
                "Do not clear previous items if command returns empty result",
                None,
            )
    }

    pub fn to_skim_options(&self) -> SkimOptions {
        let Self {
            bind,
            multi,
            prompt,
            cmd_prompt,
            expect,
            tac,
            no_sort: nosort,
            tiebreak,
            exact,
            interactive,
            query,
            cmd_query,
            regex,
            color,
            margin,
            no_height,
            no_clear,
            no_clear_start,
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
            keep_right,
            skip_to_pattern,
            select1: select_1,
            exit0: exit_0,
            sync,
            selector,
            no_clear_if_empty,
        } = self;

        SkimOptions {
            bind: bind.clone(),
            multi: *multi,
            prompt: prompt.as_deref().unwrap_or_default().to_owned(),
            cmd_prompt: cmd_prompt.as_deref().unwrap_or_default().to_owned(),
            expect: expect.clone(),
            tac: *tac,
            no_sort: *nosort,
            // TODO: Get Skim to export RankCriteria and move the conversion to somewhere that can
            // return a failure
            tiebreak: tiebreak.clone(),
            exact: *exact,
            //cmd: cmd.is_some().then(|| "ls"),
            cmd: Some("ls".to_owned()),
            interactive: *interactive,
            query: query.clone(),
            cmd_query: cmd_query.clone(),
            regex: *regex,
            color: color.clone(),
            margin: margin.as_deref().unwrap_or("0,0,0,0").to_owned(),
            no_height: *no_height,
            no_clear: *no_clear,
            no_clear_start: *no_clear_start,
            min_height: min_height.as_deref().unwrap_or("10").to_owned(),
            height: height.as_deref().unwrap_or("100%").to_owned(),
            preview_window: preview_window.as_deref().unwrap_or("right:50%").to_owned(),
            reverse: *reverse,
            tabstop: tabstop.unwrap_or(8),
            no_hscroll: *no_hscroll,
            no_mouse: *no_mouse,
            inline_info: *inline_info,
            layout: if *reverse {
                "reverse"
            } else {
                layout.as_deref().unwrap_or("default")
            }
            .to_owned(),
            algorithm: *algorithm,
            case: *case,
            // cmd_collector: if let Some(cmd) = cmd {
            // use std::fs;
            // use std::io::Write;
            // let mut file = fs::File::options().create(true).append(true).open("/tmp/sklog.log").unwrap();
            // writeln!(&mut file, "Creating it").unwrap();
            // Rc::new(RefCell::new(NuCommandCollector {
            // context,
            // closure: cmd.clone(),
            // }))
            // } else {
            // Rc::new(RefCell::new(SkimItemReader::default()))
            // },
            keep_right: *keep_right,
            skip_to_pattern: skip_to_pattern.clone(),
            select_1: *select_1,
            exit_0: *exit_0,
            sync: *sync,
            // selector: selector.clone(),
            selector: {
                struct S;

                // Note
                impl Selector for S {
                    fn should_select(&self, _: usize, _: &dyn skim::SkimItem) -> bool {
                        true
                    }
                }
                Some(Rc::new(S))
            },
            no_clear_if_empty: *no_clear_if_empty,
            ..Default::default()
        }
    }
}
