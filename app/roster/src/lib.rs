#![feature(negative_impls)]
pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::server::ServerConfigBuilder;

#[cfg(debug_assertions)]
pub const VERSION: &str =
    concat!("(dev) ", env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH"),);

#[cfg(not(debug_assertions))]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
