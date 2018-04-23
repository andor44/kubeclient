use std::collections::HashMap;
use uuid::Uuid;

mod group_version;
pub use self::group_version::*;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectMeta {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub generate_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub self_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uuid>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(default)]
    pub generation: i64,
    #[serde(default)]
    pub creation_timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, String>,

    // TODO: OwnerReferences, Initialziers, Finalizers, ClusterName
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMeta {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub self_link: String,
    // `continue` is a reserved keyword
    #[serde(rename = "continue", skip_serializing_if = "String::is_empty")]
    pub _continue: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeMeta {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub self_link: String,
}
