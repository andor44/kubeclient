use num_traits::Zero;

use crate::api;
use crate::apimachinery::apis::meta;
use crate::apimachinery::util::IntOrString;
use super::{API_GROUP, API_VERSION};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: DeploymentSpec,
    #[serde(default)]
    pub status: DeploymentStatus,
}

kube_kind!(Deployment, DeploymentList, "deployments");

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DeploymentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    pub selector: Option<meta::v1::LabelSelector>,
    pub template: api::core::v1::PodTemplateSpec,
    #[serde(default)]
    pub strategy: DeploymentStrategy,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub min_ready_seconds: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
    pub paused: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
}

pub type DeploymentStrategyType = String;

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DeploymentStrategy {
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub strategy_type: DeploymentStrategyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct RollingUpdateDeployment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DeploymentStatus {
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub observed_generation: i64,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub updated_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub ready_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub available_replicas: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub unavailable_replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeploymentCondition>,
}

type DeploymentConditionType = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentCondition {
    #[serde(rename = "type")]
    pub condition_type: DeploymentConditionType,
    pub status: api::core::v1::ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
