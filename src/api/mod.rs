use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod core;

pub trait KubeKind: DeserializeOwned + Serialize {
    const KIND_NAME: &'static str;
}
