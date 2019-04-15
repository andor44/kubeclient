use num_traits::Zero;

use crate::api;
use crate::apimachinery::apis::meta;

use super::API_GROUP;
pub const API_VERSION: &str = "v1";


#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    pub metadata: meta::v1::ObjectMeta,
    pub spec: JobSpec,
    #[serde(default)]
    pub status: JobStatus,
}

kube_kind!(Job, JobList, "jobs");

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct JobSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<meta::v1::LabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_selector: Option<bool>,
    pub template: api::core::v1::PodTemplateSpec,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct JobStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<JobCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub active: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub succeeded: i32,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub failed: i32,
}

pub type JobConditionType = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct JobCondition {
    #[serde(rename = "type")]
    pub condition_type: JobConditionType,
    pub status: api::core::v1::ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
