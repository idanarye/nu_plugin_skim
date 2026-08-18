use std::collections::HashSet;
use std::ops::Range;

use ansi_to_tui::IntoText;
use nu_plugin::EvaluatedCall;
use nu_protocol::shell_error::generic::GenericError;
use nu_protocol::{IntoSpanned, PipelineData, ShellError, Span as NuSpan, Value};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use skim::prelude::*;

use crate::command_context::CommandContext;

pub struct NuItem {
    pub context: Arc<CommandContext>,
    pub value: Value,
    text: String,
    display: Line<'static>,
}

impl NuItem {
    pub fn new(context: Arc<CommandContext>, value: Value) -> Self {
        let display = parse_ansi(
            context
                .format
                .map(&context, &value)
                .to_expanded_string(", ", &context.nu_config),
        );
        let text = display
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect();
        Self {
            context,
            value,
            text,
            display,
        }
    }
}

fn parse_ansi(formatted: String) -> Line<'static> {
    if !formatted.contains('\x1b') {
        return Line::from(formatted);
    }
    let Ok(parsed) = formatted.as_bytes().into_text() else {
        return Line::from(formatted);
    };
    let mut spans = Vec::new();
    for (index, line) in parsed.lines.into_iter().enumerate() {
        // Re-insert newlines dropped by `into_text`, for --multiline
        if index > 0 {
            spans.push(Span::raw("\n"));
        }
        spans.extend(line.spans);
    }
    Line::from(spans)
}

// Taken from `skim::tui::util::merge_styles`, which is not public
fn merge_styles(left: Style, right: Style) -> Style {
    fn merge_color(left: Option<Color>, right: Option<Color>) -> Option<Color> {
        match (left, right) {
            (Some(Color::Reset), _) => right,
            (_, Some(Color::Reset)) => left,
            _ => right.or(left),
        }
    }
    Style {
        fg: merge_color(left.fg, right.fg),
        bg: merge_color(left.bg, right.bg),
        underline_color: merge_color(left.underline_color, right.underline_color),
        add_modifier: left.add_modifier | right.add_modifier,
        ..Style::default()
    }
}

enum MatchedChars {
    Indices(HashSet<usize>),
    Range(Range<usize>),
}

impl MatchedChars {
    fn new(matches: &Matches, text: &str) -> Option<Self> {
        Some(match matches {
            Matches::None => return None,
            Matches::CharIndices(indices) => Self::Indices(indices.iter().copied().collect()),
            Matches::CharRange(start, end) => Self::Range(*start..*end),
            Matches::ByteRange(start, end) => {
                // Same fallbacks as Skim
                let char_start = text
                    .get(..*start)
                    .map_or(0, |prefix| prefix.chars().count());
                let char_end = text
                    .get(..*end)
                    .map_or_else(|| text.chars().count(), |prefix| prefix.chars().count());
                Self::Range(char_start..char_end)
            }
        })
    }

    fn contains(&self, char_index: usize) -> bool {
        match self {
            Self::Indices(indices) => indices.contains(&char_index),
            Self::Range(range) => range.contains(&char_index),
        }
    }
}

fn highlight(display: &Line<'_>, text: &str, context: &DisplayContext) -> Line<'static> {
    let styled = |content: String, ansi_style: Style, is_match: bool| {
        let style = merge_styles(context.base_style, ansi_style);
        let style = if is_match {
            merge_styles(style, context.matched_style)
        } else {
            style
        };
        Span::styled(content, style)
    };

    let Some(matched) = MatchedChars::new(&context.matches, text) else {
        return display
            .spans
            .iter()
            .map(|span| styled(span.content.to_string(), span.style, false))
            .collect();
    };

    let mut spans = Vec::new();
    let mut char_index = 0;
    for span in &display.spans {
        // Consecutive characters with the same match state share a span.
        let mut run = String::new();
        let mut run_is_match = false;
        for ch in span.content.chars() {
            let is_match = matched.contains(char_index);
            if is_match != run_is_match && !run.is_empty() {
                spans.push(styled(std::mem::take(&mut run), span.style, run_is_match));
            }
            run_is_match = is_match;
            run.push(ch);
            char_index += 1;
        }
        if !run.is_empty() {
            spans.push(styled(run, span.style, run_is_match));
        }
    }
    Line::from(spans)
}

impl SkimItem for NuItem {
    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.text)
    }

    fn display(&self, context: DisplayContext) -> Line<'_> {
        highlight(&self.display, &self.text, &context)
    }

    fn preview(&self, context: PreviewContext) -> ItemPreview {
        let preview_result = self.context.preview.map(&self.context, &self.value);
        if let Ok(preview_result) = preview_result.coerce_string() {
            return ItemPreview::AnsiText(preview_result);
        }
        let result = self.context.engine.find_decl("table").and_then(
            #[allow(clippy::result_large_err)]
            |table_decl| {
                let table_decl = table_decl.ok_or_else(|| {
                    ShellError::Generic(GenericError::new(
                        "`table` decl is empty",
                        "`table` decl is empty",
                        NuSpan::unknown(),
                    ))
                })?;
                let as_table = self.context.engine.call_decl(
                    table_decl,
                    // TODO: get the actual span
                    EvaluatedCall::new(NuSpan::unknown()).with_named(
                        "width".into_spanned(NuSpan::unknown()),
                        Value::int(context.width as i64, NuSpan::unknown()),
                    ),
                    PipelineData::Value((*preview_result).clone(), None),
                    true,
                    false,
                )?;
                let as_table_text = as_table.collect_string("\n", &self.context.nu_config)?;
                Ok(as_table_text)
            },
        );
        match result {
            Ok(text) => ItemPreview::AnsiText(text),
            Err(err) => ItemPreview::AnsiText(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: Style = Style::new().fg(Color::White).bg(Color::Black);
    const MATCHED: Style = Style::new().fg(Color::Red);

    fn context(matches: Matches) -> DisplayContext {
        DisplayContext {
            matches,
            base_style: BASE,
            matched_style: MATCHED,
            ..Default::default()
        }
    }

    /// The rendered line, as `(text, foreground, background)` per span.
    fn rendered(line: &Line<'_>) -> Vec<(String, Option<Color>, Option<Color>)> {
        line.spans
            .iter()
            .map(|span| (span.content.to_string(), span.style.fg, span.style.bg))
            .collect()
    }

    fn highlight_str(formatted: &str, matches: Matches) -> Line<'static> {
        let display = parse_ansi(formatted.to_owned());
        let text: String = display
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect();
        let highlighted = highlight(&display, &text, &context(matches));
        // Whatever the styling, Skim clips the line by the character count of `text()`, so
        // the two must always hold the same characters.
        assert_eq!(
            highlighted
                .spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<String>(),
            text,
        );
        highlighted
    }

    #[test]
    fn plain_text_with_char_indices() {
        let line = highlight_str("hello", Matches::CharIndices(vec![1, 2]));
        assert_eq!(
            rendered(&line),
            [
                ("h".to_owned(), Some(Color::White), Some(Color::Black)),
                // Adjacent matched characters are merged into one span, and the base
                // style's background survives underneath the match color.
                ("el".to_owned(), Some(Color::Red), Some(Color::Black)),
                ("lo".to_owned(), Some(Color::White), Some(Color::Black)),
            ],
        );
    }

    #[test]
    fn plain_text_with_char_range() {
        let line = highlight_str("hello", Matches::CharRange(3, 5));
        assert_eq!(
            rendered(&line),
            [
                ("hel".to_owned(), Some(Color::White), Some(Color::Black)),
                ("lo".to_owned(), Some(Color::Red), Some(Color::Black)),
            ],
        );
    }

    #[test]
    fn byte_range_is_converted_to_char_positions() {
        // "é" is two bytes, so the byte range 1..4 is the char range 1..3.
        let line = highlight_str("aébc", Matches::ByteRange(1, 4));
        assert_eq!(
            rendered(&line),
            [
                ("a".to_owned(), Some(Color::White), Some(Color::Black)),
                ("éb".to_owned(), Some(Color::Red), Some(Color::Black)),
                ("c".to_owned(), Some(Color::White), Some(Color::Black)),
            ],
        );
    }

    #[test]
    fn no_matches_still_applies_the_base_style() {
        let line = highlight_str("hello", Matches::None);
        assert_eq!(
            rendered(&line),
            [("hello".to_owned(), Some(Color::White), Some(Color::Black))],
        );
    }

    #[test]
    fn ansi_colors_survive_and_matches_override_them() {
        let line = highlight_str("ab\x1b[32mcd\x1b[0mef", Matches::CharIndices(vec![0, 2, 4]));
        assert_eq!(
            rendered(&line),
            [
                // Matched, so the theme's match color wins over the item's own color.
                ("a".to_owned(), Some(Color::Red), Some(Color::Black)),
                ("b".to_owned(), Some(Color::White), Some(Color::Black)),
                ("c".to_owned(), Some(Color::Red), Some(Color::Black)),
                // Not matched, so the item's green survives - over the theme's background.
                ("d".to_owned(), Some(Color::Green), Some(Color::Black)),
                ("e".to_owned(), Some(Color::Red), Some(Color::Black)),
                ("f".to_owned(), Some(Color::White), Some(Color::Black)),
            ],
        );
    }

    #[test]
    fn newlines_are_preserved_across_ansi_parsing() {
        let display = parse_ansi("a\x1b[32mb\nc\x1b[0md".to_owned());
        let text: String = display
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect();
        assert_eq!(text, "ab\ncd");
    }

    #[test]
    fn reset_does_not_override_the_other_style() {
        let reset = Style::new().fg(Color::Reset).bg(Color::Reset);
        assert_eq!(merge_styles(BASE, reset).fg, Some(Color::White));
        assert_eq!(merge_styles(BASE, reset).bg, Some(Color::Black));
        assert_eq!(merge_styles(reset, MATCHED).fg, Some(Color::Red));
    }
}
