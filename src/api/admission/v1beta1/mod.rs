use uuid::Uuid;

use api;


type Operation = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdmissionRequest<T> {
    // TODO: not string, UUID
    pub uid: Uuid,
    pub kind: String,//api::meta::v1::GroupVersionKind,
    pub resource: api::meta::v1::GroupVersionResource,
    pub subresource: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub operation: Operation,
    pub user_info: api::authentication::v1::UserInfo,
    pub object: Option<Option<T>>,
    pub old_object: Option<Option<T>>,
}
