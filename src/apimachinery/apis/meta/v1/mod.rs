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
    #[serde(default, skip_serializing_if = "Uuid::is_nil")]
    pub uid: Uuid,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(default, skip_serializing_if = "i64::is_zero")]
    pub generation: i64,
    // XXX: This *should not* be Option but I haven't been able to think of a way
    // to better represent this. It's a value field in the k8s API with `omitempty`.
    // Truthfully, that means that it has a "zero value". Time is a wrapper for
    // Go's official `time` type. The zero value of that is documented here:
    // https://golang.org/pkg/time/#Time
    // That's apparently Year 1, January 1, 00:00:00.0 UTC
    // We could make a fixed function and skip serialization on that, but the sheer
    // stupidity of a "zero value" for time makes my brain hurt.
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owner_references: Vec<OwnerReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initializers: Option<Initializers>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cluster_name: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Initializers {
    pub pending: Vec<Initializer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Status>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Initializer {
    pub name: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerReference {
    pub api_version: String,
    pub kind: String,
    pub name: String,
    pub uid: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_owner_deletion: Option<bool>,
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
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StatusDetails {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causes: Vec<StatusCause>,
    #[serde(default)]
    pub retry_after_seconds: i32,
}

type StatusReason = String;

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize, Deserialize)]
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
