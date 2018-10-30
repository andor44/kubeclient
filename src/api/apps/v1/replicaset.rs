use num_traits::Zero;

use api;
use apimachinery::apis::meta;
use super::{API_GROUP, API_VERSION};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplicaSet {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: ReplicaSetSpec,
    #[serde(default)]
    pub status: ReplicaSetStatus,
}

kube_kind!(ReplicaSet, ReplicaSetList, "replicasets");

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ReplicaSetSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    pub selector: Option<meta::v1::LabelSelector>,
    #[serde(default)]
    pub template: api::core::v1::PodTemplateSpec,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub min_ready_seconds: i32,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ReplicaSetStatus {
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub fully_labeled_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub ready_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub available_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub observed_generation: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ReplicaSetCondition>,
}

pub type ReplicaSetConditionType = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplicaSetCondition {
    #[serde(rename = "type")]
    pub condition_type: ReplicaSetConditionType,
    pub status: api::core::v1::ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
