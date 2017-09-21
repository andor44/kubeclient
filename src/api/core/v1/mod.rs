use api::{KubeKind, meta};

// Core is special
pub const API_GROUP: &str = "core";
pub const API_VERSION: &str = "v1";

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
    pub metadata: meta::ObjectMeta,
    pub spec: NamespaceSpec,
    pub status: Option<NamespaceStatus>,
}

kube_kind!(Namespace, NamespaceList, "namespaces");
