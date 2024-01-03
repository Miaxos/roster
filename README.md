# roster

> Replacement of Redis with Rust

`roster` is an in-memory data store which is aiming to provide a fully
comptabile redis APIs.

It is more like an expirement right now on, feel free to contribute.

## Architecture

The idea is to use every ressources avaiable on the hardware. To do that, we use
a shared-nothing architecture with a thread-per-core runtime (monoio).

Each thread got his own shard of data to handle, it can be a replica of another
thread or another server.

## References

- [RESP3](https://github.com/redis/redis-specifications/blob/master/protocol/RESP3.md)
- https://github.com/tair-opensource/compatibility-test-suite-for-redis
- https://github.com/redis/redis-specifications
- https://github.com/redis/redis-benchmarks-specification
