extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate pem_parser;

mod utils;
pub mod client;
pub mod api;

pub use client::{KubeClient, KubeClientConfig};
