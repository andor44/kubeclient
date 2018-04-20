use std::collections::HashMap;

mod group_version;
pub use self::group_version::*;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectMeta {
    pub name: String,
    pub creation_timestamp: Option<String>,
    pub resource_version: Option<String>,
    pub self_link: Option<String>,
    pub uid: Option<String>,
    pub annotations: Option<HashMap<String, String>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMeta {
    pub resource_version: String,
    pub self_link: String,
}
