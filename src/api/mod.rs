use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod meta;
pub mod core;
pub mod rbac;


pub trait KubeKind: DeserializeOwned + Serialize {
    const KIND_NAME: &'static str;
}
