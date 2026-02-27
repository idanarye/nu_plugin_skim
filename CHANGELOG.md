# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.24.1](https://github.com/idanarye/nu_plugin_skim/compare/v0.24.0...v0.24.1) (2026-02-27)


### Bug Fixes

* force an old version of interprocess because that package does not respect SemVer ([bf09862](https://github.com/idanarye/nu_plugin_skim/commit/bf09862d34c713d8811a806c08fb845be43a6d0e))

## [0.24.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.23.1...v0.24.0) (2026-02-27)


### Features

* Upgrade Rust edition to 2024 ([bd7bcba](https://github.com/idanarye/nu_plugin_skim/commit/bd7bcbae7a5db6397aba18adcf7ff8a546632c85))
* Upgrade Skim to 2.*.* ([9f42ce2](https://github.com/idanarye/nu_plugin_skim/commit/9f42ce244d3a41dedf1991d98fdaa8aeaea30226))


### Bug Fixes

* `--cmd` flag (Fixes [#50](https://github.com/idanarye/nu_plugin_skim/issues/50)) ([45bf44a](https://github.com/idanarye/nu_plugin_skim/commit/45bf44a08fa61d8eb14ada0dac73ab0602245057))

## [0.23.1](https://github.com/idanarye/nu_plugin_skim/compare/v0.23.0...v0.23.1) (2026-02-01)


### Bug Fixes

* parse `--bind` into `keymap` (since Skim now ignores `bind` after it does its own parsing) ([6f180e4](https://github.com/idanarye/nu_plugin_skim/commit/6f180e4b7b6e288d54a741239419cd82bb5105d2))

## [0.23.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.22.0...v0.23.0) (2026-01-29)


### Features

* Upgrade Skim to 1.*.* ([7294db9](https://github.com/idanarye/nu_plugin_skim/commit/7294db9ffbca4557b80cf97f9197790bbaa7c322))

## [0.22.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.21.0...v0.22.0) (2026-01-18)


### Features

* Upgrade Nu version to 0.110 ([55ae583](https://github.com/idanarye/nu_plugin_skim/commit/55ae583ad698816ddeabfa1a8094d860dbddd426))

## [0.21.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.20.1...v0.21.0) (2025-11-29)


### Features

* Upgrade Nu version to 0.109 ([ee349e7](https://github.com/idanarye/nu_plugin_skim/commit/ee349e753456599227280762b74a9eccb56bdc5c))

## [0.20.1](https://github.com/idanarye/nu_plugin_skim/compare/v0.20.0...v0.20.1) (2025-10-16)


### Bug Fixes

* Use the `sqlite` feature of the `nu-protocol` requirement, to be able to load the user nushell config if they use sqlite ([cdbd145](https://github.com/idanarye/nu_plugin_skim/commit/cdbd1456048b2023bc0a6a65067513ebf53e64ed))

## [0.20.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.19.0...v0.20.0) (2025-10-16)


### Features

* Upgrade Nu version to 0.108 ([762f232](https://github.com/idanarye/nu_plugin_skim/commit/762f2329abea9e931c8d7bcbd3013883e45282c9))

## [0.19.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.18.0...v0.19.0) (2025-09-03)


### Features

* Upgrade Nu version to 0.107 ([3b87af2](https://github.com/idanarye/nu_plugin_skim/commit/3b87af2613630860c41698e734fa5a9ea0df7a13))

## [0.18.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.17.0...v0.18.0) (2025-09-01)


### Features

* add default argument by $SKIM_DEFAULT_OPTIONS. Code is generate by ChatGPT-codex, review before merge. ([32ec382](https://github.com/idanarye/nu_plugin_skim/commit/32ec38263fc72c901cf6cafc5842ffa1d7f9a53a))
* add default argument doc to README; sty: format by prettier ([2e5a8b4](https://github.com/idanarye/nu_plugin_skim/commit/2e5a8b4610d57d47f5791117fabc2765446cb7e6))
* use $SKIM_DEFAULT_OPTIONS for default arguments ([cebe658](https://github.com/idanarye/nu_plugin_skim/commit/cebe658e1d67ba396ee7ba88777b46f6fcf270b7))


### Bug Fixes

* resolve cargo check and clippy warnings; sty: format code with cargo fmt &lt;NO BREAKING CHANGES&gt; ([f9cfa4c](https://github.com/idanarye/nu_plugin_skim/commit/f9cfa4c865afeb4c78ac949df937d0a416da6540))
* use `EngineInterface::get_env_var` instead of `std:env:var` ([98a9c34](https://github.com/idanarye/nu_plugin_skim/commit/98a9c34bab4ca7cd4fbdac35af67e8429e9c8d38))
* use both environmentVariable and arguments input now, like `skim` ([64ace17](https://github.com/idanarye/nu_plugin_skim/commit/64ace17916686a7c50d9e0c960c13f6a269d6e08))

## [0.17.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.16.0...v0.17.0) (2025-08-26)


### Features

* Upgrade skim to 0.20 ([4bdaca1](https://github.com/idanarye/nu_plugin_skim/commit/4bdaca17b72c4036be8fbeb5a688a7fa9bb44c0c))


### Bug Fixes

* color highlight ([4d28af4](https://github.com/idanarye/nu_plugin_skim/commit/4d28af4b497ce075f6f9821da391bf2e9f148ca2))
* color highlight ([a0391bc](https://github.com/idanarye/nu_plugin_skim/commit/a0391bc5ef0d95466bbf495a0544927820e09182))

## [0.16.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.15.0...v0.16.0) (2025-07-26)


### Features

* Upgrade Nu version to 0.106 ([fc920ce](https://github.com/idanarye/nu_plugin_skim/commit/fc920cedb86b7813757b0c67303e25127e119074))

## [0.15.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.14.0...v0.15.0) (2025-06-10)


### Features

* Upgrade Nu version to 0.105 ([a476a00](https://github.com/idanarye/nu_plugin_skim/commit/a476a00305d6e59bec8b86790d04af21782577bd))

## [0.14.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.13.0...v0.14.0) (2025-04-30)


### Features

* Upgrade Nu version to 0.104 ([b5fdfd2](https://github.com/idanarye/nu_plugin_skim/commit/b5fdfd2a8151641a9ab1494598409e6e3111b898))

## [0.13.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.12.0...v0.13.0) (2025-03-21)


### Features

* Upgrade Nu version to 0.103 ([80ad80b](https://github.com/idanarye/nu_plugin_skim/commit/80ad80bdfca5878447a84baf054f54ef2c1da694))

## [0.12.0](https://github.com/idanarye/nu_plugin_skim/compare/v0.11.1...v0.12.0) (2025-02-10)


### Features

* Upgrade Nu version to 0.102 ([2066d88](https://github.com/idanarye/nu_plugin_skim/commit/2066d8842339421b09b9f3f10b8cbc337052514a))

## [0.11.1](https://github.com/idanarye/nu_plugin_skim/compare/v0.11.0...v0.11.1) (2024-12-23)


### Bug Fixes

* Downgrade skim back to 0.13 to fix multiselect ([a7bccd2](https://github.com/idanarye/nu_plugin_skim/commit/a7bccd242c57f4fcdde0d0b1a6134047b644c2e7))

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
