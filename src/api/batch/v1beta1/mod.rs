use api;
use apimachinery::apis::meta;

use super::API_GROUP;
pub const API_VERSION: &str = "v1beta1";


#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct CronJob {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: CronJobSpec,
    #[serde(default)]
    pub status: CronJobStatus,
}

kube_kind!(CronJob, CronJobList, "cronjobs");

pub type ConcurrencyPolicy = String;

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CronJobSpec {
    pub schedule: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub concurrency_policy: ConcurrencyPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    pub job_template: JobTemplateSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct JobTemplateSpec {
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: api::batch::v1::JobSpec,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CronJobStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active: Vec<api::core::v1::ObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<meta::v1::Time>,
}
