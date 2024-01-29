# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Documentation: Command list for reddis

## [0.1.1](https://github.com/Miaxos/roster/compare/v0.1.0...v0.1.1) - 2024-01-07

### Added
- add auto-release
- support pipelining
- change old frame to new frame parsing to gain ~30%
- add atoi_simd to optimize parse + check
- add bench for parse_frame
- prepare to optimize the basics
- add simple get storage
- add set expiration
- add initial set with basic storage
- add nesting commands
- add io-uring for linux
- add some tests and add proper ping

### Fixed
- fix use coarsetime instant

### Other
- prepare before switching to new parsing
- add basic frame test
- update core
- remove dbg
- benches
- add proper bench
- timing
- update README
- clippy & readme
- remove useless imports
