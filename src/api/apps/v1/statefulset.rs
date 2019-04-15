use num_traits::Zero;

use crate::api;
use crate::apimachinery::apis::meta;
use super::{API_GROUP, API_VERSION};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct StatefulSet {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: StatefulSetSpec,
    #[serde(default)]
    pub status: StatefulSetStatus,
}

kube_kind!(StatefulSet, StatefulSetList, "statefulsets");

pub type PodManagementPolicyType = String;

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct StatefulSetSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    pub selector: Option<meta::v1::LabelSelector>,
    pub template: api::core::v1::PodTemplateSpec,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_claim_templates: Vec<api::core::v1::PersistentVolumeClaim>,
    pub service_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pod_management_policy: PodManagementPolicyType,
    #[serde(default)]
    pub update_strategy: StatefulSetUpdateStrategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
}

pub type StatefulSetUpdateStrategyType = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct StatefulSetUpdateStrategy {
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub strategy_type: StatefulSetUpdateStrategyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateStatefulSetStrategy>,
}

impl ::std::default::Default for StatefulSetUpdateStrategy {
    fn default() -> Self {
        StatefulSetUpdateStrategy {
            strategy_type: "RollingUpdate".to_string(),
            rolling_update: None,
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct RollingUpdateStatefulSetStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct StatefulSetStatus {
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub observed_generation: i64,
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub ready_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub current_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub updated_replicas: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub current_revision: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub update_revision: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<StatefulSetCondition>,
}

type StatefulSetConditionType = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct StatefulSetCondition {
    #[serde(rename = "type")]
    pub condition_type: StatefulSetConditionType,
    pub status: api::core::v1::ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
