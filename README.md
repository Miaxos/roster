# roster

<p align="center">
    <img src="./docs/logo.png" height="256" width="256" alt="A rooster in minimalist style with rainbow colours generated via StableDiffusion">
</p>

> Replacement of Redis with Rust & io-uring

[![release](https://github.com/Miaxos/roster/actions/workflows/release.yml/badge.svg)](https://github.com/Miaxos/roster/actions/workflows/release.yml)
[![Crates.io version](https://img.shields.io/crates/v/roster.svg)](https://crates.io/crates/roster)
[![dependency status](https://deps.rs/repo/github/miaxos/roster/status.svg)](https://deps.rs/repo/github/miaxos/roster)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/miaxos/roster/compare)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FMiaxos%2Froster.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FMiaxos%2Froster?ref=badge_shield)

`roster` is an in-memory data store which is aiming to provide a fully
comptabile redis APIs.

It is more like an expirement right now on, feel free to contribute. Some of the
initial code involving the resp protocol comes from `mini-redis`.

The current work is to have a good fundation to be able to build the Redis
Protocol on top of it.

## Benchmarks
If you want some benchmarks, feel free to check on:
[benchmarks](./docs/benchmarks.md);

The redis APIs are not properly implemented and only some basics things are
present so those benchmarks are only to check that the decisions made for the
storage & the I/O are good.

Those will be updated as implementations progress.

Benchmarks are made between Redis, Dragonfly & Roster.

### ENT1-M Scaleway

First benchmarks are based on an ENT1-M Scaleway which is a decent instance but
not a really big one and we are limited a lot by the instance and the network
between our two instances, as for Redis & Dragonfly.

- 16 vCPUS
- RAM: 64G
- BW: 3,2 Gbps 

<p align="center">
    <img src="./docs/entm1-scaleway.svg" width="60%" />
</p>

## Protocol

### Reddis

- Only the RESP3 is wanted for now.

## Architecture

### Performance

To be able to max out performances out of an application we must be able to have
a linear-scalability.[^1] Usual issues around scalability are when you share
data between threads, (like false-sharing[^3]). To solve this issue, we will use
a shared-nothing architecture (right now we aren't due to some APIs not
implemented yet) where each thread will own his own slice of the storage.

We also use a runtime which is based on `io-uring` to handle every I/O on the
application: [monoio](https://github.com/bytedance/monoio/).

[^1]: It means if we have an application running 100 op/s on one thread, if we 
add another one, we should be at 200 op/s. We have a linear scalability. (or
a near linear scalability).
[^3]: An excellent article explaining it: [alic.dev](https://alic.dev/blog/false-sharing).

In the same spirit as a shared nothing architecture we use one thread per core
to maximize ressources available on the hardware.

"*Application tail latency is critical for services to meet their latency 
expectations. We have shown that the thread-per-core approach can reduce 
application tail latency of a key-value store by up to 71% compared to baseline 
Memcached running on commodity hardware and Linux.*"[^2]

[^2]: [The Impact of Thread-Per-Core Architecture on Application Tail Latency](https://helda.helsinki.fi/server/api/core/bitstreams/3142abaa-16e3-4ad0-beee-e62add589fc4/content)

### Storage

We use
[scc::Hashmap](https://github.com/wvwwvwwv/scalable-concurrent-containers#HashMap) behind an `Arc` for now while Sharding APIs are not implemented on [monoio](https://github.com/bytedance/monoio/issues/213) but as soon as we have a way to load-balance our TCP Connection to the proper thread, we should switch to a `scc::Hashmap` per thread.

## References

- [RESP3](https://github.com/redis/redis-specifications/blob/master/protocol/RESP3.md)
- https://github.com/tair-opensource/compatibility-test-suite-for-redis
- https://github.com/redis/redis-specifications
- https://github.com/redis/redis-benchmarks-specification


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FMiaxos%2Froster.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FMiaxos%2Froster?ref=badge_large)