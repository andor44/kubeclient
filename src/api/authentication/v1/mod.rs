use std::collections::HashMap;


type ExtraValue = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub username: Option<String>,
    pub uid: Option<String>,
    pub groups: Vec<String>,
    pub extra: HashMap<String, ExtraValue>,
}
