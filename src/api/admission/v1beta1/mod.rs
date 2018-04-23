use uuid::Uuid;

use api;


type Operation = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdmissionReview<T> {
    #[serde(flatten)]
    pub type_meta: api::meta::v1::TypeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AdmissionRequest<T>>,
    // TODO: impl
    // pub response: Option<AdmissionRequest<T>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdmissionRequest<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uuid>,
    pub kind: api::meta::v1::GroupVersionKind,
    pub resource: api::meta::v1::GroupVersionResource,
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
