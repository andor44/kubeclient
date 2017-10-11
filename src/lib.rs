#![feature(associated_type_defaults)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

extern crate reqwest;
extern crate pem_parser;

#[macro_use] extern crate log;

extern crate base64;
// TODO: see if this crate can be used
// at the moment it cannot, due to it appearing in a compound type
// #[macro_use] extern crate base64_serde;

mod utils;
pub mod client;
pub mod api;
pub mod config;

pub use client::{RequestError, KubeClient};
pub use config::{ClientConfig, AuthConfig};
