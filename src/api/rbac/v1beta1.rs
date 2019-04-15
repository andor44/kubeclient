use crate::apimachinery::apis::meta;

use super::API_GROUP;
pub const API_VERSION: &str = "v1beta1";


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterRole {
    pub metadata: meta::v1::ObjectMeta,
    pub rules: Vec<PolicyRule>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PolicyRule {
    pub verbs: Vec<String>,
    pub api_groups: Option<Vec<String>>,
    pub resources: Option<Vec<String>>,
    pub resource_names: Option<Vec<String>>,
    pub non_resource_urls: Option<Vec<String>>,
}

kube_kind!(ClusterRole, ClusterRoleList, "clusterroles");

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClusterRoleBinding {
    pub metadata: meta::v1::ObjectMeta,
    pub subjects: Vec<Subject>,
    pub role_ref: RoleRef,
}

kube_kind!(ClusterRoleBinding, ClusterRoleBindingList, "clusterrolebindings");

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RoleBinding {
    pub metadata: meta::v1::ObjectMeta,
    pub subjects: Vec<Subject>,
    pub role_ref: RoleRef,
}

kube_kind!(RoleBinding, RoleBindingList, "rolebindings");

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Subject {
    pub kind: String,
    pub api_group: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RoleRef {
    pub api_group: String,
    pub kind: String,
    pub name: String,
}
