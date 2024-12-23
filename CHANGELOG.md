# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.11.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.10.0...v0.11.0) (2024-12-23)


### Features

* Upgrade Nu version to 0.101 ([e39120e](https://github.com/idanarye/nu_plugin_skim/commit/e39120ecedee63a795a9b74933c2276d2c6dc204))

## [0.10.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.9.1...v0.10.0) (2024-11-25)


### Features

* Migrate back to Skim, since it's now maintained again ([c678542](https://github.com/idanarye/nu_plugin_skim/commit/c678542f3c8569f4828f0ba6fd66f7fdc6f1751a))

## [0.9.1](https://github.com/idanarye/nu_plugin_skim/compare/v0.9.0...v0.9.1) (2024-11-21)


### Bug Fixes

* 18: pre-calculate the `--format` lambda ([71b0df3](https://github.com/idanarye/nu_plugin_skim/commit/71b0df3339a4a82798bac2a0fd5fd835a6f8a218))

## [0.9.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.8.0...v0.9.0) (2024-11-13)


### Features

* Upgrade Nu version to 0.100 ([32437bd](https://github.com/idanarye/nu_plugin_skim/commit/32437bd42ef33ace27034aa343e396c19fb461e8))

## [0.8.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.7.0...v0.8.0) (2024-10-20)


### Features

* Upgrade Nu version to 0.99 ([e77ad4b](https://github.com/idanarye/nu_plugin_skim/commit/e77ad4b1f4d8d7c249c83dd4714816077058e8fc))

## [0.7.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.6.0...v0.7.0) (2024-09-30)


### Features

* Properly pass `--width` when invoking `table` in the preview window ([253cc7c](https://github.com/idanarye/nu_plugin_skim/commit/253cc7c7060f9c72cde4175f1c3d575819d40833))


### Bug Fixes

* Properly handle ANSI ([d400868](https://github.com/idanarye/nu_plugin_skim/commit/d4008680000f2614e855a02f254ef2ef1f45199a))

## [0.6.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.5.0...v0.6.0) (2024-09-18)


### Features

* Update Nu and two-percent versions ([8d3ebaf](https://github.com/idanarye/nu_plugin_skim/commit/8d3ebaf3afac5936f08f6283906a8b31576c8b15))

## [0.5.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.4.1...v0.5.0) (2024-08-21)


### Features

* Upgrade to Nu protocol 0.97 ([9ed8191](https://github.com/idanarye/nu_plugin_skim/commit/9ed8191e2e79d83238fa0d0764718483e587af30))

## [0.4.1](https://github.com/idanarye/nu_plugin_skim/compare/v0.4.0...v0.4.1) (2024-08-07)


### Bug Fixes

* Commit `Cargo.lock` to the repository (Fix [#10](https://github.com/idanarye/nu_plugin_skim/issues/10)) ([2e1739a](https://github.com/idanarye/nu_plugin_skim/commit/2e1739a3c036554341139e79e33497d19fff5712))

## [0.4.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.3.0...v0.4.0) (2024-08-05)


### Features

* Add shortopts for `--format` (`-f`) and `--preview` (`-p`) ([e77e3e2](https://github.com/idanarye/nu_plugin_skim/commit/e77e3e21d8438f366dfd7a6afcb8f86203ec7230))
* Add the `--pre-select` flag (Close [#5](https://github.com/idanarye/nu_plugin_skim/issues/5)) ([3c16d7c](https://github.com/idanarye/nu_plugin_skim/commit/3c16d7cd1a427f338182ad1865257fdc9a076f56))
* Support interactive mode with `-i` and `-c` (Close [#4](https://github.com/idanarye/nu_plugin_skim/issues/4)) ([b81ee88](https://github.com/idanarye/nu_plugin_skim/commit/b81ee8892f54a6e18bd5d88890737b7194a736e2))


### Bug Fixes

* Set the signature of `--cmd` to accept one string parameter ([3cde093](https://github.com/idanarye/nu_plugin_skim/commit/3cde0937509c9c5eadab06efad3f592a1aee6a7b))

## [0.3.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.2.0...v0.3.0) (2024-07-30)


### Features

* Upgrade Nu version to 0.96 ([cd4402f](https://github.com/idanarye/nu_plugin_skim/commit/cd4402f0e76b574e834baff7bbc9321a0c3f9415))

## [Unreleased]

## [0.2.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.1.1...v0.2.0) - 2024-07-01

### Changed
- Upgrade Nushell API to 0.95.

## [0.1.1](https://github.com/idanarye/nu_plugin_skim/compare/v0.1.0...v0.1.1) - 2024-06-16

### Fixed
- Make the install command in the README install from crates.io

## [0.1.0](https://github.com/idanarye/nu_plugin_skim/releases/tag/v0.1.0) - 2024-06-16

### Added
- sk` command that can handle Nushell structure data.
