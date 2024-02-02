# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3](https://github.com/Miaxos/roster/compare/v0.1.2...v0.1.3) - 2024-02-02

### Other
- add a way to publish releases

## [0.1.2](https://github.com/Miaxos/roster/compare/v0.1.1...v0.1.2) - 2024-02-02

### Added
- update monoio and allow deploy
- add client get name
- add client info command
- add set_name
- add ascii data for initial launch
- add startup ascii
- add help & test
- add client list capabilities
- add supervisor into context
- add client id
- add some metadata connection draft
- add acl cat
- solve issues with memory explosion
- prepare dialer & cluster
- update storage & storage segment
- *(storage)* add storage & storage segment
- add hash to infrastructure layer
- use sharded-thread 1
- add base structure for dialer
- add initial work for sharding

### Fixed
- add help for setname
- Cargo.toml target windows
- add target windows

### Other
- Merge pull request [#19](https://github.com/Miaxos/roster/pull/19) from Miaxos/feat-add-hello
- Merge pull request [#21](https://github.com/Miaxos/roster/pull/21) from Miaxos/renovate/tokio-1.x
- *(deps)* update rust crate tokio to 1.36
- clippy
- fmt
- clippy
- clippy
- wait 1s for CI to work
- clippy
- remove useless code from set_info
- add simple test for client list
- add client_id tests
- add test for parsing client id
- clippy
- add client list parsing tests
- add some test for parsing
- fmt
- clippy
- allow dead code for now
- clippy
- comment useless stuff for now
- remove useless stuff
- add comment
- remove clippy warning for now
- clippy
- fmt
- remove useless code
- clippy
- change a little the server instantiation
- remove useless code
- conclusion
- wip
- remove eviction
- remove useless dependencies
- *(deps)* update rust crate tokio to 1.35
- enable tests
- clippy
- cargo fmt

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
