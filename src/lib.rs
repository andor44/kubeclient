// (De)serialization
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

// HTTP client library
extern crate reqwest;

// Logging macros
#[macro_use] extern crate log;

// Misc utilities
extern crate base64;
// TODO: see if this crate can be used
// at the moment it cannot, due to it appearing in a compound type
// #[macro_use] extern crate base64_serde;
extern crate uuid;
extern crate chrono;
extern crate num_traits;

mod utils;
pub mod client;
pub mod api;
pub mod config;

pub use client::{RequestResult, RequestError, KubeClient};
pub use config::{ClientConfig, AuthConfig};
