use uuid::Uuid;

use apimachinery::apis::meta;
use api;


type Operation = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AdmissionReview<T> {
    #[serde(flatten)]
    pub type_meta: meta::v1::TypeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AdmissionRequest<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<AdmissionResponse>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AdmissionRequest<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uuid>,
    pub kind: meta::v1::GroupVersionKind,
    pub resource: meta::v1::GroupVersionResource,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subresource: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    pub operation: Operation,
    pub user_info: api::authentication::v1::UserInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_object: Option<T>,
}

type PatchType = String;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AdmissionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uuid>,
    pub allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<meta::v1::Status>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub patch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_type: Option<PatchType>,
}
