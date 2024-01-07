# roster

<p align="center">
    <img src="./docs/logo.png" height="256" width="256" alt="A rooster in minimalist style with rainbow colours generated via StableDiffusion">
</p>

> Replacement of Redis with Rust & io-uring

[![release](https://github.com/Miaxos/roster/actions/workflows/release.yml/badge.svg)](https://github.com/Miaxos/roster/actions/workflows/release.yml)
[![Crates.io version](https://img.shields.io/crates/v/roster.svg)](https://crates.io/crates/roster)
[![dependency status](https://deps.rs/repo/github/miaxos/roster/status.svg)](https://deps.rs/repo/github/miaxos/roster)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/miaxos/roster/compare)

`roster` is an in-memory data store which is aiming to provide a fully
comptabile redis APIs.

It is more like an expirement right now on, feel free to contribute. Some of the
initial code involving the resp protocol comes from `mini-redis`.

## Redis RESP

Only the RESP3 is wanted for now.

## Architecture

### Performance



### Shared nothing architecture

#### Idea
Each thread got his own shard of data to handle, it can be a replica of another
thread or another server.

#### Current implementation

We use
[scc::Hashmap](https://github.com/wvwwvwwv/scalable-concurrent-containers#HashMap) behind an `Arc` for now while Sharding APIs are not implemented on [monoio](https://github.com/bytedance/monoio/issues/213).

### Thread per core
"*The idea is to use every ressources avaiable on the hardware. To do that, we use
a shared-nothing architecture with a thread-per-core runtime (monoio).

Application tail latency is critical for services to meet their latency 
expectations. We have shown that the thread-per-core approach can reduce 
application tail latency of a key-value store by up to 71% compared to baseline 
Memcached running on commodity hardware and Linux.*"[^1]

## References

[^1]: [The Impact of Thread-Per-Core Architecture on Application Tail Latency](https://helda.helsinki.fi/server/api/core/bitstreams/3142abaa-16e3-4ad0-beee-e62add589fc4/content)
- [RESP3](https://github.com/redis/redis-specifications/blob/master/protocol/RESP3.md)
- https://github.com/tair-opensource/compatibility-test-suite-for-redis
- https://github.com/redis/redis-specifications
- https://github.com/redis/redis-benchmarks-specification
