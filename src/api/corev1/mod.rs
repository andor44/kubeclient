use std::collections::HashMap;
use super::KubeKind;

type FinalizerName = String;
type NamespacePhase = String;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectMeta {
    pub name: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "resourceVersion")]
    pub resource_version: Option<String>,
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    pub uid: Option<String>,
    pub annotations: Option<HashMap<String, String>>,
}

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
    pub metadata: ObjectMeta,
    pub spec: NamespaceSpec,
    pub status: Option<NamespaceStatus>,
}

impl KubeKind for Namespace {
    const KIND_NAME: &'static str = "namespaces";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMeta {
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceList {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub items: Vec<Namespace>,
    pub kind: String,
    pub metadata: ListMeta,
}

impl KubeKind for NamespaceList {
    const KIND_NAME: &'static str = "namespaces";
}
