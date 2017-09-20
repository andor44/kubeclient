use api::{KubeKind, meta};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterRole {
    pub metadata: meta::ObjectMeta,
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PolicyRule {
    pub verbs: Vec<String>,
    #[serde(rename = "apiGroups")]
    pub api_groups: Option<Vec<String>>,
    pub resources: Option<Vec<String>>,
    #[serde(rename = "resourceNames")]
    pub resource_names: Option<Vec<String>>,
    #[serde(rename = "nonResourceURLs")]
    pub non_resource_urls: Option<Vec<String>>,
}

impl KubeKind for ClusterRole {
    const KIND_NAME: &'static str = "clusterroles";
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterRoleBinding {
    pub metadata: meta::ObjectMeta,
    pub subjects: Vec<Subject>,
    #[serde(rename = "roleRef")]
    pub role_ref: RoleRef,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Subject {
    pub kind: String,
    #[serde(rename = "apiGroup")]
    pub api_group: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RoleRef {
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    pub kind: String,
    pub name: String,
}
