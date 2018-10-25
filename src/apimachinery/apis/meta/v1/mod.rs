use std::collections::HashMap;
use chrono;
use num_traits::Zero;
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
    #[serde(default, skip_serializing_if = "i64::is_zero")]
    pub generation: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<Time>,
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
    #[serde(default, rename = "continue", skip_serializing_if = "String::is_empty")]
    pub _continue: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TypeMeta {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
}

pub type Time = chrono::DateTime<chrono::FixedOffset>;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct LabelSelector {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub match_labels: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<LabelSelectorRequirement>,
}

pub type LabelSelectorOperator = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct LabelSelectorRequirement {
    pub key: String,
    pub operator: LabelSelectorOperator,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

type CauseType = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCause {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cause: CauseType,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusDetails {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uuid>,
    pub causes: Vec<StatusCause>,
    #[serde(default)]
    pub retry_after_seconds: i32,
}

type StatusReason = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: StatusReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<StatusDetails>,
    #[serde(default)]
    pub code: i32,
}
