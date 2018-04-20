use std::collections::HashMap;
use uuid::Uuid;


type ExtraValue = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub username: Option<String>,
    pub uid: Option<Uuid>,
    pub groups: Vec<String>,
    pub extra: HashMap<String, ExtraValue>,
}
