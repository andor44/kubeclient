#[derive(Serialize, Deserialize, Debug)]
pub struct GroupVersionResource {
    pub group: String,
    pub version: String,
    pub resource: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupVersionKind {
    pub group: String,
    pub version: String,
    pub kind: String,
}
