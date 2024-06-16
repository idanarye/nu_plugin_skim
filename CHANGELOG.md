# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/idanarye/nu_plugin_skim/releases/tag/v0.1.0) - 2024-06-16

### Other
- Add some metadata
- Configure release-plz
- Add `--no-height`
- Add `--query`
- Add `--no-clear-start` and `--no-clear-if-empty`
- Add `--pre-select-*`
- Add `--select-1` and `--exit-0`
- Add `--skip-to-pattern`
- Add `--keep-right`
- `--algo` and `--case` generate an error when used with unsupported values
- Add `--algo` and `--case`
- Add `--reverse` and `--layout`
- Add `--no-hscroll`, `--no-mouse` and `--inline-info`
- Add `--tabstop`
- Add `--preview-window`
- Fix `--height` (which was the one I added in the previous commit, and not `--no-height`)
- Add `--min-height` and `--no-height`
- Add `--no-clear`
- Add `--color` and `--margin`
- Add `--exact` and `--regex`
- Add `--tiebreak`
- Add `--tac` and `--no-sort`
- Return `PipelineData::empty()` when aborted
- Implement `--expect`
- Add the `--bind` argument
- Write the README
- Add a `--prompt` argument
- Organize the CLI arguments into a struct
- Support byte stream input
- Revert "Create module for bridging the CLI arguments from Nu to Skim"
- Create module for bridging the CLI arguments from Nu to Skim
- Support multi selection
- Properly handle streamed input
- Properly draw the preview panel using the `nu-table` crate
- `--preview` can properly run external command
- Add `--preview`
- Fix `--format`
- Add `--format` flag
- Handle all values, not just strings
- Initial implementation (very stupid, strings only)
- Initial commit
