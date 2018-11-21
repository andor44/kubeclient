use uuid::Uuid;
use num_traits::Zero;

use apimachinery::apis::meta;
use apimachinery::api::resource::ResourceList;
use super::{API_GROUP, API_VERSION, ConditionStatus};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Node {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: NodeSpec,
    #[serde(default)]
    pub status: NodeStatus,
}

kube_kind!(Node, NodeList, "nodes");

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NodeSpec {
    #[serde(rename = "podCIDR", default, skip_serializing_if = "String::is_empty")]
    pub pod_cidr: String,
    #[serde(rename = "providerID", default, skip_serializing_if = "String::is_empty")]
    pub provider_id: String,
    #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
    pub unschedulable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taints: Vec<Taint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_source: Option<NodeConfigSource>,
    #[serde(rename = "externalID", default, skip_serializing_if = "String::is_empty")]
    pub external_id: String,
}

pub type TaintEffect = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Taint {
    pub key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    pub effect: TaintEffect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_added: Option<meta::v1::Time>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeConfigSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapNodeConfigSource>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMapNodeConfigSource {
    pub namespace: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Uuid::is_nil")]
    pub uid: Uuid,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    pub kubelet_config_key: String,
}

pub type NodePhase = String;
pub type UniqueVolumeName = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NodeStatus {
    #[serde(default, skip_serializing_if = "ResourceList::is_empty")]
    pub capacity: ResourceList,
    #[serde(default, skip_serializing_if = "ResourceList::is_empty")]
    pub allocatable: ResourceList,
    #[serde(default, skip_serializing_if = "NodePhase::is_empty")]
    pub phase: NodePhase,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NodeCondition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<NodeAddress>,
    #[serde(default)]
    pub daemon_endpoints: NodeDaemonEndpoints,
    #[serde(default)]
    pub node_info: NodeSystemInfo,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ContainerImage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_in_use: Vec<UniqueVolumeName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_attached: Vec<AttachedVolume>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<NodeConfigStatus>,
}

pub type NodeConditionType = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NodeCondition {
    #[serde(rename = "type")]
    pub condition_type: NodeConditionType,
    pub status: ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

pub type NodeAddressType = String;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NodeAddress {
    #[serde(rename = "type")]
    pub address_type: NodeAddressType,
    pub address: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NodeDaemonEndpoints {
    #[serde(default)]
    pub kubelet_endpoint: DaemonEndpoint,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DaemonEndpoint {
    #[serde(rename = "Port")]
    pub port: i32,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NodeSystemInfo {
    #[serde(rename = "machineID")]
    pub machine_id: String,
    #[serde(rename = "systemUUID")]
    pub system_uuid: String,
    #[serde(rename = "bootID")]
    pub boot_id: String,
    pub kernel_version: String,
    pub os_image: String,
    pub container_runtime_version: String,
    pub kubelet_version: String,
    pub kube_proxy_version: String,
    pub operating_system: String,
    pub architecture: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerImage {
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "i64::is_zero")]
    pub size_bytes: i64,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AttachedVolume {
    pub name: UniqueVolumeName,
    pub device_path: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NodeConfigStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<NodeConfigSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<NodeConfigSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good: Option<NodeConfigSource>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
}
