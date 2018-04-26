use std::collections::HashMap;


type ExtraValue = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, ExtraValue>,
}
