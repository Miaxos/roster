#![feature(negative_impls)]
pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::server::ServerConfigBuilder;
