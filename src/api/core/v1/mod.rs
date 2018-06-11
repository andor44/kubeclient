use api::meta;

// Core is special
pub const API_GROUP: &str = "core";
pub const API_VERSION: &str = "v1";

mod pod;
mod secret;

pub use self::pod::*;
pub use self::secret::*;


// Namespace
type FinalizerName = String;
type NamespacePhase = String;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NamespaceSpec {
    pub finalizers: Vec<FinalizerName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceStatus {
    pub phase: NamespacePhase,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Namespace {
    pub metadata: meta::v1::ObjectMeta,
    pub spec: NamespaceSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<NamespaceStatus>,
}

kube_kind!(Namespace, NamespaceList, "namespaces");
