use serde::de::DeserializeOwned;
use serde::Serialize;

macro_rules! kube_kind {
    ( $typ:ty, $list_name:ident, $name: expr ) => {
        impl KubeKind for $typ {
            const KIND_NAME: &'static str = $name;
            const API_GROUP: &'static str = API_GROUP;
            const API_VERSION: &'static str = API_VERSION;

            type List = $list_name;
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct $list_name {
            #[serde(rename = "apiVersion")]
            pub api_version: String,
            pub items: Vec<$typ>,
            pub kind: String,
            pub metadata: meta::ListMeta,
        }
    }
}

pub mod meta;
pub mod core;
pub mod rbac;


/// Trait used to represent types used in the Kubernetes API
/// Generally you should use the `kube_kind!` macro to implement this.
/// To do this, your module representing an API will need to define the following 2 variables:
/// `API_GROUP: &'static str`
/// `API_VERSION: &'static str`
pub trait KubeKind: DeserializeOwned + Serialize {
    const KIND_NAME: &'static str;
    const API_GROUP: &'static str;
    const API_VERSION: &'static str;

    type List: DeserializeOwned = Vec<Box<Self>>;
}
