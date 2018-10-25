use std::collections::HashMap;

use apimachinery::api::resource::ResourceList;
use apimachinery::apis::meta;

// Core is special
use super::API_GROUP;
pub const API_VERSION: &str = "v1";

mod pod;
mod secret;

pub use self::pod::*;
pub use self::secret::*;


// Namespace
pub type FinalizerName = String;
pub type NamespacePhase = String;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NamespaceSpec {
    pub finalizers: Vec<FinalizerName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceStatus {
    pub phase: NamespacePhase,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Namespace {
    pub metadata: meta::v1::ObjectMeta,
    pub spec: NamespaceSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<NamespaceStatus>,
}

kube_kind!(Namespace, NamespaceList, "namespaces");

pub type ConditionStatus = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentVolumeClaim {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: PersistentVolumeClaimSpec,
    #[serde(default)]
    pub status: PersistentVolumeClaimStatus,
}

kube_kind!(PersistentVolumeClaim, PersistentVolumeClaimList, "namespaces");

pub type PersistentVolumeAccessMode = String;
pub type PersistentVolumeMode = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentVolumeClaimSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<PersistentVolumeAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<meta::v1::LabelSelector>,
    pub resources: ResourceRequirements,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub storage_class_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<PersistentVolumeMode>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResourceRequirements {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub limits: ResourceList,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub requests: ResourceList,
}

type PersistentVolumeClaimPhase = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentVolumeClaimStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: PersistentVolumeClaimPhase,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<PersistentVolumeAccessMode>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub capacity: ResourceList,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PersistentVolumeClaimCondition>,
}

pub type PersistentVolumeClaimConditionType = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentVolumeClaimCondition {
    pub condition_type: PersistentVolumeClaimConditionType,
    pub status: ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<meta::Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
