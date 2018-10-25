#[serde(untagged)]
#[derive(Serialize, Deserialize, Debug)]
pub enum IntOrString {
    Int(i64),
    String(String),
}
