use num_traits::Zero;

use api;
use apimachinery::apis::meta;
use apimachinery::util::IntOrString;
use super::{API_GROUP, API_VERSION};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct DaemonSet {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: DaemonSetSpec,
    #[serde(default)]
    pub status: DaemonSetStatus,
}

kube_kind!(DaemonSet, DaemonSetList, "daemonsets");

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DaemonSetSpec {
    #[serde(default)]
    pub selector: Option<meta::v1::LabelSelector>,
    pub template: api::core::v1::PodTemplateSpec,
    #[serde(default)]
    pub update_strategy: DaemonSetUpdateStrategy,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub min_ready_seconds: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
}

pub type DaemonSetUpdateStrategyType = String;

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DaemonSetUpdateStrategy {
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub strategy_type: DaemonSetUpdateStrategyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDaemonSet>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct RollingUpdateDaemonSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DaemonSetStatus {
    pub current_number_scheduled: i32,
    pub number_misscheduled: i32,
    pub desired_number_scheduled: i32,
    pub number_ready: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub observed_generation: i64,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub updated_number_scheduled: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub number_available: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub number_unavailable: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DaemonSetCondition>,
}

type DaemonSetConditionType = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct DaemonSetCondition {
    #[serde(rename = "type")]
    pub condition_type: DaemonSetConditionType,
    pub status: api::core::v1::ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
