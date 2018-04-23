use serde::de::DeserializeOwned;
use serde::Serialize;

///! NOTE: Go uses `omitempty` for many fields, i.e. if the string is empty the field will be left
///! out of the serialized version. Serde's "default" feature maps to this behavior but I'm not 100%
///! convinced this is the right way to go.
///! While this way maps closest to the way the Go code is written it is not ideal. The k8s Go API
///! is a mess, because in some places `string` is used, where the empty string is treated as the
///! absence of value, whereas in other places `*string` is used, which is more akin to
///! Option<String> in Rust.
///! To match the behavior the best go's `string` is mapped to `String` and `*string` to
///! `Option<String>`.
///! Other types will be treated on a case-by-case basis as described here:
///! * Uuid - represented as Option<Uuid> because only the API server generated these, we're not
///! supposed to supply them, and using `Default` would result in a zeroed-out UUID.

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

pub mod admission;
pub mod authentication;
pub mod core;
pub mod meta;
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

    type List: DeserializeOwned = Vec<Self>;
}
