use std::collections::HashMap;

use crate::apimachinery::apis::meta;
use crate::apimachinery::api::resource;
use crate::apimachinery::util::IntOrString;
use super::{API_GROUP, API_VERSION};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Pod {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: PodSpec,
    #[serde(default)]
    pub status: PodStatus,
}

kube_kind!(Pod, PodList, "pods");

pub type PodPhase = String;

pub type PodQOSClass = String;

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PodStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: PodPhase,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PodCondition>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nominated_node_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host_ip: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pod_ip: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_container_statuses: Vec<ContainerStatus>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container_statuses: Vec<ContainerStatus>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub qos_class: PodQOSClass,
}

type PodConditionType = String;

type ConditionStatus = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodCondition {
    #[serde(rename = "type")]
    pub condition_type: PodConditionType,
    pub status: ConditionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStatus {
    pub name: String,
    #[serde(default)]
    pub state: ContainerState,
    #[serde(default)]
    pub last_termination_state: ContainerState,
    pub ready: bool,
    pub restart_count: i32,
    pub image: String,
    #[serde(rename = "imageID")]
    pub image_id: String,
    #[serde(rename = "containerID", default, skip_serializing_if = "String::is_empty")]
    pub container_id: String,
}

// TODO: this could be represented way more effectively with an enum
#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ContainerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<ContainerStateWaiting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<ContainerStateRunning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated: Option<ContainerStateTerminated>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStateWaiting {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStateRunning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<meta::v1::Time>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStateTerminated {
    pub exit_code: i32,
    #[serde(default)]
    pub signal: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<meta::v1::Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<meta::v1::Time>,
    #[serde(rename = "containerID", skip_serializing_if = "String::is_empty")]
    pub container_id: String,
}

type DnsPolicy = String;
type RestartPolicy = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PodSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<Volume>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_containers: Vec<Container>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<Container>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub restart_policy: RestartPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub dns_policy: DnsPolicy,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub node_selector: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account_name: String,
    #[serde(rename = "serviceAccount", default, skip_serializing_if = "String::is_empty")]
    pub deprecated_service_account: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
    pub host_network: bool,
    #[serde(rename = "hostPID", default, skip_serializing_if = "::std::ops::Not::not")]
    pub host_pid: bool,
    #[serde(rename = "hostIPC", default, skip_serializing_if = "::std::ops::Not::not")]
    pub host_ipc: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<PodSecurityContext>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subdomain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Affinity>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheduler_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<HostAlias>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub priority_class_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<PodDnsConfig>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Volume {
    pub name: String,
    #[serde(flatten)]
    pub source: VolumeSource,
}

pub type HostPathType = String;
pub type StorageMedium = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyToPath {
    pub key: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub enum VolumeSource {
    #[serde(rename_all = "camelCase")]
    HostPath {
        path: String,
        // TODO: make this an enum?
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    EmptyDir {
        #[serde(default, skip_serializing_if = "String::is_empty")]
        medium: StorageMedium,
        // TODO: sizeLimit field
    },
    #[serde(rename_all = "camelCase")]
    Secret {
        #[serde(default, skip_serializing_if = "String::is_empty")]
        secret_name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        items: Vec<KeyToPath>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_mode: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        optional: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    PersistentVolumeClaim {
        claim_name: String,
        #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
        read_only: bool,
    },
    DownwardAPI {

    },
    #[serde(rename_all = "camelCase")]
    ConfigMap {
        #[serde(flatten)]
        reference: LocalObjectReference,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        items: Vec<KeyToPath>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_mode: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        optional: Option<bool>,
    },
    Projected {

    },
    // TODO: rest of the volume types
}

type TerminationMessagePolicy = String;
type PullPolicy = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub working_dir: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<EnvFromSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_devices: Vec<VolumeDevice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<Probe>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<Probe>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub termination_message_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_message_policy: Option<TerminationMessagePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<PullPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<SecurityContext>,
    #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
    pub stdin: bool,
    #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
    pub stdin_once: bool,
    #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
    pub tty: bool,
}

type Protocol = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerPort {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    pub container_port: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub protocol: Protocol,
    #[serde(rename = "hostIP", default, skip_serializing_if = "String::is_empty")]
    pub host_ip: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct EnvFromSource {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub prefix: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map_ref: Option<ConfigMapEnvSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretEnvSource>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigMapEnvSource {
    #[serde(flatten)]
    pub reference: LocalObjectReference,
    #[serde(default)]
    pub optional: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SecretEnvSource {
    #[serde(flatten)]
    pub reference: LocalObjectReference,
    #[serde(default)]
    pub optional: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalObjectReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct EnvVar {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<EnvVarSource>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub enum EnvVarSource {
    FieldRef(ObjectFieldSelector),
    ResourceFieldRef(ResourceFieldSelector),
    ConfigMapKeyRef(ConfigMapKeySelector),
    SecretKeyRef(SecretKeySelector),
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectFieldSelector {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    pub field_path: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceFieldSelector {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_name: String,
    pub resource: String,
    // TODO: another API?
    // pub divisor: Option<resource::Quantity>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigMapKeySelector {
    #[serde(flatten)]
    pub reference: LocalObjectReference,
    pub key: String,
    #[serde(default)]
    pub optional: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SecretKeySelector {
    #[serde(flatten)]
    pub reference: LocalObjectReference,
    pub key: String,
    #[serde(default)]
    pub optional: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceRequirements {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub limits: resource::ResourceList,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub requests: resource::ResourceList,
}

type MountPropagationMode = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeMount {
    pub name: String,
    #[serde(default, skip_serializing_if = "::std::ops::Not::not")]
    pub read_only: bool,
    pub mount_path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mount_propagation: MountPropagationMode,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeDevice {
    pub name: String,
    pub device_path: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Probe {
    #[serde(flatten)]
    pub handler: Handler,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
}

type UriScheme = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub enum Handler {
    Exec {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        command: Vec<String>,
    },
    HttpGet {
        #[serde(default, skip_serializing_if = "String::is_empty")]
        path: String,
        port: IntOrString,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        host: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        scheme: UriScheme,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        http_headers: Vec<HttpHeader>,
    },
    TcpSocket {
        port: IntOrString,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        host: String,
    },
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct HttpHeader {
    name: String,
    value: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Lifecycle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_start: Option<Handler>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<Handler>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<SeLinuxOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalaltion: Option<bool>,
}

type Capability = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<Capability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drop: Vec<Capability>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SeLinuxOptions {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub role: String,
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub selinux_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub level: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodSecurityContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<SeLinuxOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_groups: Option<i64>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Affinity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<NodeAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_affinity: Option<PodAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: Option<PodAntiAffinity>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: Option<NodeSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredSchedulingTerm>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeSelector {
    pub node_selector_terms: Vec<NodeSelectorTerm>,
}


#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeSelectorTerm {
    pub match_expressions: Vec<NodeSelectorRequirement>,
}

type NodeSelectorOperator = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeSelectorRequirement {
    pub key: String,
    pub operator: NodeSelectorOperator,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PreferredSchedulingTerm {
    pub weight: i32,
    pub preference: NodeSelectorTerm,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodAffinity {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodAntiAffinity {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<meta::v1::LabelSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
    pub topology_key: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WeightedPodAffinityTerm {
    pub weight: i32,
    pub pod_affinity_term: PodAffinityTerm,
}

type TolerationOperator = String;
type TaintEffect = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Toleration {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operation: TolerationOperator,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effect: TaintEffect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAlias {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hostnames: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodDnsConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nameservers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<PodDnsConfigOption>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct PodDnsConfigOption {
    // TODO: the comment on this field says it is required but has `omitempty`???
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PodTemplateSpec {
    #[serde(default)]
    pub metadata: meta::v1::ObjectMeta,
    #[serde(default)]
    pub spec: PodSpec,
}
