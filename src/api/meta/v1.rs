use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectMeta {
    pub name: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "resourceVersion")]
    pub resource_version: Option<String>,
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    pub uid: Option<String>,
    pub annotations: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMeta {
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
}
