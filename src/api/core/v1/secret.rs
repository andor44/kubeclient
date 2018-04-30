use std::fmt;
use std::collections::HashMap;

use base64;
use serde::{Serialize, Serializer};
use serde::de::{self, Visitor, Deserialize, Deserializer};

use api;
use super::{API_GROUP, API_VERSION};


// Secret
type SecretType = String;

pub struct SecretData(pub Vec<u8>);

// XXX: base64 is better than printing a byte array, but should this perhaps just obscure it
// completely?
impl fmt::Debug for SecretData {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", base64::encode(&self.0))
    }
}

impl Serialize for SecretData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&base64::encode(&self.0))
    }
}

struct SecretDataVisitor;

impl<'de> Visitor<'de> for SecretDataVisitor {
    type Value = SecretData;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("a base64 encoded string of the underlying data")
    }

    fn visit_str<E>(self, value: &str) -> Result<SecretData, E>
        where E: de::Error 
    {
        base64::decode(value).map(SecretData).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for SecretData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(SecretDataVisitor)
    }
}

// struct SecretDat
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Secret {
    pub metadata: api::meta::ObjectMeta,
    pub status: Option<api::core::NamespaceStatus>,
    // TODO: stringData is a key that is used for reading only, never writing
    // therefore it is never in the output. Should this library care about fields like that?
    // pub string_data: Option<Vec<String>>
    
    pub data: HashMap<String, SecretData>,
    #[serde(rename = "type")]
    pub secret_type: Option<SecretType>,
}

kube_kind!(Secret, SecretList, "secrets");