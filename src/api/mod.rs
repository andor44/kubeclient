use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod corev1;

pub trait KubeKind: DeserializeOwned + Serialize {
    const KIND_NAME: &'static str;
}
