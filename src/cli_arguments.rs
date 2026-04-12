use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    rc::Rc,
};

use clap::ValueEnum;
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{
    LabeledError, Record, ShellError, Signature, Spanned, SyntaxShape, engine::Closure,
};
use shlex::Shlex;
use skim::{
    CaseMatching, FuzzyAlgorithm, RankCriteria, Selector, SkimOptions,
    binds::KeyMap,
    prelude::DefaultSkimSelector,
    tui::options::{PreviewLayout, TuiLayout},
};

use crate::predicate_based_selector::{CombinedSelector, PredicateBasedSelector};

pub struct CliArguments {
    bind: Vec<String>,
    multi: bool,
    prompt: Option<String>,
    cmd_prompt: Option<String>,
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
    layout: Option<TuiLayout>,
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
        let env_defaults = EnvDefaults::from_env(engine);
        Ok(Self {
            bind: if let Some(bind) = call.get_flag::<Record>("bind")? {
                bind.iter()
                    .map(|(key, value)| {
                        let value = value.coerce_string()?;
                        Ok(format!("{key}:{value}"))
                    })
                    .collect::<Result<Vec<String>, LabeledError>>()?
            } else {
                env_defaults.bind.unwrap_or_default()
            },
            multi: call.has_flag("multi")? || env_defaults.multi.unwrap_or(false),
            prompt: call.get_flag("prompt")?.or(env_defaults.prompt),
            cmd_prompt: call.get_flag("cmd-prompt")?.or(env_defaults.cmd_prompt),
            tac: call.has_flag("tac")? || env_defaults.tac.unwrap_or(false),
            no_sort: call.has_flag("no-sort")? || env_defaults.no_sort.unwrap_or(false),
            tiebreak: {
                let from_call = call
                    .get_flag::<Vec<Spanned<String>>>("tiebreak")?
                    .unwrap_or_default()
                    .into_iter()
                    .map(|flag| {
                        RankCriteria::from_str(&flag.item, true).map_err(|_| {
                            let possible_values = RankCriteria::value_variants()
                                .iter()
                                .flat_map(|v| {
                                    Some(format!("`{}`", v.to_possible_value()?.get_name()))
                                })
                                .collect::<Vec<_>>()
                                .join("/");
                            LabeledError::new(format!(
                                "Invalid tiebreak - legal options are {possible_values}"
                            ))
                            .with_label("here", flag.span)
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if from_call.is_empty() {
                    env_defaults.tiebreak.unwrap_or_default()
                } else {
                    from_call
                }
            },
            exact: call.has_flag("exact")? || env_defaults.exact.unwrap_or(false),
            interactive: call.has_flag("interactive")? || env_defaults.interactive.unwrap_or(false),
            query: call.get_flag("query")?.or(env_defaults.query),
            cmd_query: call.get_flag("cmd-query")?.or(env_defaults.cmd_query),
            regex: call.has_flag("regex")? || env_defaults.regex.unwrap_or(false),
            color: call.get_flag("color")?.or(env_defaults.color),
            margin: call.get_flag("margin")?.or(env_defaults.margin),
            no_height: call.has_flag("no-height")? || env_defaults.no_height.unwrap_or(false),
            no_clear: call.has_flag("no-clear")? || env_defaults.no_clear.unwrap_or(false),
            no_clear_start: call.has_flag("no-clear-start")?
                || env_defaults.no_clear_start.unwrap_or(false),
            min_height: call
                .get_flag::<i64>("min-height")?
                .map(|num| num.to_string())
                .or(env_defaults.min_height),
            height: call.get_flag("height")?.or(env_defaults.height),
            preview_window: call
                .get_flag("preview-window")?
                .or(env_defaults.preview_window),
            reverse: call.has_flag("reverse")? || env_defaults.reverse.unwrap_or(false),
            tabstop: call.get_flag::<usize>("tabstop")?.or(env_defaults.tabstop),
            no_hscroll: call.has_flag("no-hscroll")? || env_defaults.no_hscroll.unwrap_or(false),
            no_mouse: call.has_flag("no-mouse")? || env_defaults.no_mouse.unwrap_or(false),
            inline_info: call.has_flag("inline-info")? || env_defaults.inline_info.unwrap_or(false),
            layout: call
                .get_flag_value("layout")
                .map(parse_value_enum_from_flag)
                .transpose()?
                .or(env_defaults.layout),
            algorithm: call
                .get_flag_value("algo")
                .map(parse_value_enum_from_flag)
                .transpose()?
                .unwrap_or(env_defaults.algorithm.unwrap_or_default()),
            case: call
                .get_flag_value("case")
                .map(parse_value_enum_from_flag)
                .transpose()?
                .unwrap_or(env_defaults.case.unwrap_or_default()),
            keep_right: call.has_flag("keep-right")? || env_defaults.keep_right.unwrap_or(false),
            skip_to_pattern: call
                .get_flag("skip-to-pattern")?
                .or(env_defaults.skip_to_pattern),
            select1: call.has_flag("select-1")? || env_defaults.select1.unwrap_or(false),
            exit0: call.has_flag("exit-0")? || env_defaults.exit0.unwrap_or(false),
            sync: call.has_flag("sync")? || env_defaults.sync.unwrap_or(false),
            selector: {
                let mut dumb_selector: Option<DefaultSkimSelector> = None;

                // First apply env-derived pre-select options (if any),
                // then apply flag-derived options additively.
                if let Some(n) = env_defaults.pre_select_n {
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().first_n(n));
                }
                if let Some(pat) = env_defaults.pre_select_pat {
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().regex(&pat));
                }
                if let Some(items) = env_defaults.pre_select_items {
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().preset(items));
                }
                if let Some(file_path) = env_defaults.pre_select_file {
                    let file =
                        File::open(file_path).map_err(|e| LabeledError::new(e.to_string()))?;
                    let items = BufReader::new(file)
                        .lines()
                        .collect::<Result<Vec<String>, _>>()
                        .map_err(|e| LabeledError::new(e.to_string()))?;
                    dumb_selector = Some(dumb_selector.take().unwrap_or_default().preset(items));
                }

                // Now apply flag-derived pre-select options additively
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
            no_clear_if_empty: call.has_flag("no-clear-if-empty")?
                || env_defaults.no_clear_if_empty.unwrap_or(false),
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
                format!("Choose the layout: [{}]", value_enum_possibilities_string::<TuiLayout>()),
                None,
            )
            .named(
                "algo",
                SyntaxShape::String,
                format!("Fuzzy matching algorithm: [{}] (default: skim_v2)", value_enum_possibilities_string::<FuzzyAlgorithm>()),
                None,
            )
            .named(
                "case",
                SyntaxShape::String,
                format!("Case sensitivity: [{}] (default: smart)", value_enum_possibilities_string::<CaseMatching>()),
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
                "Pre-select the items in the given list",
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

        // I'd really prefer to use the construct syntax, but Rust won't let me (even with the
        // spread operator) because some fields are private.
        let mut result = SkimOptions::default();

        result.bind = bind.clone();
        result.keymap = {
            let mut keymap = KeyMap::default();
            keymap.add_keymaps(bind.iter().map(String::as_str));
            keymap
        };
        result.multi = *multi;
        result.no_multi = !*multi;
        result.prompt = prompt.as_deref().unwrap_or_default().to_owned();
        result.cmd_prompt = cmd_prompt.as_deref().unwrap_or_default().to_owned();
        result.tac = *tac;
        result.no_sort = *nosort;
        result.tiebreak = tiebreak.clone();
        result.exact = *exact;
        result.cmd = Some("ls".to_owned());
        result.interactive = *interactive;
        result.query = query.clone();
        result.cmd_query = cmd_query.clone();
        result.regex = *regex;
        result.color = color.clone();
        result.margin = margin.as_deref().unwrap_or("0,0,0,0").to_owned();
        result.no_height = *no_height;
        result.no_clear = *no_clear;
        result.no_clear_start = *no_clear_start;
        result.min_height = min_height.as_deref().unwrap_or("10").to_owned();
        result.height = height.as_deref().unwrap_or("100%").to_owned();
        result.preview_window = preview_window
            .as_deref()
            .map(PreviewLayout::from)
            .unwrap_or_default();
        result.reverse = *reverse;
        result.tabstop = tabstop.unwrap_or(8);
        result.no_hscroll = *no_hscroll;
        result.no_mouse = *no_mouse;
        result.inline_info = *inline_info;
        result.layout = if *reverse {
            TuiLayout::Reverse
        } else {
            layout.unwrap_or_default()
        }
        .to_owned();
        result.algorithm = *algorithm;
        result.case = *case;
        result.keep_right = *keep_right;
        result.skip_to_pattern = skip_to_pattern.clone();
        result.select_1 = *select_1;
        result.exit_0 = *exit_0;
        result.sync = *sync;
        result.selector = selector.clone();
        result.no_clear_if_empty = *no_clear_if_empty;
        result
    }
}

#[derive(Default)]
struct EnvDefaults {
    bind: Option<Vec<String>>,
    multi: Option<bool>,
    prompt: Option<String>,
    cmd_prompt: Option<String>,
    expect: Option<Vec<String>>,
    tac: Option<bool>,
    no_sort: Option<bool>,
    tiebreak: Option<Vec<RankCriteria>>,
    exact: Option<bool>,
    interactive: Option<bool>,
    query: Option<String>,
    cmd_query: Option<String>,
    regex: Option<bool>,
    color: Option<String>,
    margin: Option<String>,
    no_height: Option<bool>,
    no_clear: Option<bool>,
    no_clear_start: Option<bool>,
    min_height: Option<String>,
    height: Option<String>,
    preview_window: Option<String>,
    reverse: Option<bool>,
    tabstop: Option<usize>,
    no_hscroll: Option<bool>,
    no_mouse: Option<bool>,
    inline_info: Option<bool>,
    layout: Option<TuiLayout>,
    algorithm: Option<FuzzyAlgorithm>,
    case: Option<CaseMatching>,
    keep_right: Option<bool>,
    skip_to_pattern: Option<String>,
    select1: Option<bool>,
    exit0: Option<bool>,
    sync: Option<bool>,
    pre_select_n: Option<usize>,
    pre_select_pat: Option<String>,
    pre_select_items: Option<Vec<String>>,
    pre_select_file: Option<PathBuf>,
    no_clear_if_empty: Option<bool>,
}

impl EnvDefaults {
    fn from_env(engine: &EngineInterface) -> Self {
        match engine.get_env_var("SKIM_DEFAULT_OPTIONS") {
            Ok(Some(value)) => match value.coerce_string() {
                Ok(raw) => Self::from_options_str(&raw),
                Err(_) => Self::default(),
            },
            _ => Self::default(),
        }
    }

    fn from_options_str(s: &str) -> Self {
        let mut out = EnvDefaults::default();
        let mut it = Shlex::new(s)
            .collect::<Vec<String>>()
            .into_iter()
            .peekable();

        while let Some(tok) = it.next() {
            if tok == "--" {
                break;
            }
            if let Some(rest) = tok.strip_prefix("--") {
                let (key, val_opt) = if let Some(eq_idx) = rest.find('=') {
                    (
                        rest[..eq_idx].to_string(),
                        Some(rest[eq_idx + 1..].to_string()),
                    )
                } else {
                    (rest.to_string(), None)
                };
                match key.as_str() {
                    // boolean switches
                    "multi" => out.multi = Some(true),
                    "tac" => out.tac = Some(true),
                    "no-sort" => out.no_sort = Some(true),
                    "exact" => out.exact = Some(true),
                    "interactive" => out.interactive = Some(true),
                    "regex" => out.regex = Some(true),
                    "no-height" => out.no_height = Some(true),
                    "no-clear" => out.no_clear = Some(true),
                    "no-clear-start" => out.no_clear_start = Some(true),
                    "reverse" => out.reverse = Some(true),
                    "no-hscroll" => out.no_hscroll = Some(true),
                    "no-mouse" => out.no_mouse = Some(true),
                    "inline-info" => out.inline_info = Some(true),
                    "keep-right" => out.keep_right = Some(true),
                    "select-1" => out.select1 = Some(true),
                    "exit-0" => out.exit0 = Some(true),
                    "sync" => out.sync = Some(true),
                    "no-clear-if-empty" => out.no_clear_if_empty = Some(true),

                    // string/numeric options
                    "prompt" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.prompt = Some(v);
                        }
                    }
                    "cmd-prompt" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.cmd_prompt = Some(v);
                        }
                    }
                    "query" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.query = Some(v);
                        }
                    }
                    "cmd-query" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.cmd_query = Some(v);
                        }
                    }
                    "color" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.color = Some(v);
                        }
                    }
                    "margin" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.margin = Some(v);
                        }
                    }
                    "min-height" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.min_height = Some(v);
                        }
                    }
                    "height" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.height = Some(v);
                        }
                    }
                    "preview-window" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.preview_window = Some(v);
                        }
                    }
                    "layout" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.layout = TuiLayout::from_str(&v, true).ok();
                        }
                    }
                    "skip-to-pattern" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.skip_to_pattern = Some(v);
                        }
                    }

                    "tabstop" => {
                        if let Some(v) = set_string(val_opt, &mut it)
                            && let Ok(n) = v.parse::<usize>()
                        {
                            out.tabstop = Some(n);
                        }
                    }

                    "algo" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.algorithm = FuzzyAlgorithm::from_str(&v, true).ok();
                        }
                    }
                    "case" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.case = CaseMatching::from_str(&v, true).ok();
                        }
                    }

                    "expect" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.expect = Some(split_csv_like(&v))
                        }
                    }
                    "tiebreak" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            let vals = split_csv_like(&v);
                            let mut parsed = Vec::new();
                            for s in vals {
                                if let Ok(rc) = RankCriteria::from_str(&s, true) {
                                    parsed.push(rc);
                                }
                            }
                            out.tiebreak = Some(parsed);
                        }
                    }
                    "bind" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            // accept comma-separated bind entries like skim
                            let entries = v
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect::<Vec<_>>();
                            out.bind = Some(entries);
                        }
                    }

                    "pre-select-n" => {
                        if let Some(v) = set_string(val_opt, &mut it)
                            && let Ok(n) = v.parse::<usize>()
                        {
                            out.pre_select_n = Some(n);
                        }
                    }
                    "pre-select-pat" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.pre_select_pat = Some(v);
                        }
                    }
                    "pre-select-items" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.pre_select_items = Some(split_csv_like(&v))
                        }
                    }
                    "pre-select-file" => {
                        if let Some(v) = set_string(val_opt, &mut it) {
                            out.pre_select_file = Some(PathBuf::from(v));
                        }
                    }

                    _ => { /* unrecognized; ignore */ }
                }
                continue;
            }

            if tok.starts_with('-') {
                let flags = tok.trim_start_matches('-');
                let mut chars = flags.chars().peekable();
                while let Some(c) = chars.next() {
                    match c {
                        'm' => out.multi = Some(true),
                        'e' => out.exact = Some(true),
                        'i' => out.interactive = Some(true),
                        '1' => out.select1 = Some(true),
                        '0' => out.exit0 = Some(true),
                        'q' => {
                            // remainder of this token or next token
                            let rest: String = chars.collect();
                            if !rest.is_empty() {
                                out.query = Some(rest);
                            } else if let Some(v) = it.next() {
                                out.query = Some(v);
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        out
    }
}

fn set_string<I: Iterator<Item = String>>(
    val_opt: Option<String>,
    it: &mut std::iter::Peekable<I>,
) -> Option<String> {
    if let Some(v) = val_opt {
        Some(v)
    } else {
        it.next()
    }
}

fn split_csv_like(s: &str) -> Vec<String> {
    s.split(&[',', ' '] as &[_])
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

fn with_value_enum_possible_values<T: ValueEnum>(mut dlg: impl FnMut(&str)) {
    for variant in T::value_variants() {
        let Some(possible_value) = variant.to_possible_value() else {
            continue;
        };
        for value in possible_value.get_name_and_aliases() {
            dlg(value);
        }
    }
}

fn value_enum_possibilities_string<T: ValueEnum>() -> String {
    let mut result = String::new();
    with_value_enum_possible_values::<T>(|value| {
        if !result.is_empty() {
            result.push('|');
        }
        result.push_str(value);
    });
    result
}

fn parse_value_enum_from_flag<T: ValueEnum>(flag: nu_protocol::Value) -> Result<T, ShellError> {
    let str_value = flag.as_str()?;
    T::from_str(str_value, true).map_err(|_| ShellError::InvalidValue {
        valid: format!("[{}]", value_enum_possibilities_string::<T>()),
        actual: str_value.to_owned(),
        span: flag.span(),
    })
}
