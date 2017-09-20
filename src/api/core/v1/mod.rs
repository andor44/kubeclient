use api::{KubeKind, meta};

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

impl KubeKind for Namespace {
    const KIND_NAME: &'static str = "namespaces";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceList {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub items: Vec<Namespace>,
    pub kind: String,
    pub metadata: meta::ListMeta,
}

impl KubeKind for NamespaceList {
    const KIND_NAME: &'static str = "namespaces";
}
