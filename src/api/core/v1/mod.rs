use std::collections::HashMap;
use uuid::Uuid;

use crate::apimachinery::api::resource::ResourceList;
use crate::apimachinery::apis::meta;

// Core is special
use super::API_GROUP;
pub const API_VERSION: &str = "v1";

mod pod;
mod secret;
mod node;

pub use self::pod::*;
pub use self::secret::*;
pub use self::node::*;


// Namespace
pub type FinalizerName = String;
pub type NamespacePhase = String;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NamespaceSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<FinalizerName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NamespaceStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: NamespacePhase,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Namespace {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: NamespaceSpec,
    #[serde(default)]
    pub status: NamespaceStatus,
}

kube_kind!(Namespace, NamespaceList, "namespaces");

pub type ConditionStatus = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentVolumeClaim {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
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

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ObjectReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Uuid::is_nil")]
    pub uid: Uuid,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}
