use std::{env, fmt};
use std::error::Error as StdError;
use std::io::Error as IoError;

use serde_json;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Error as JsonError;
use reqwest::{
    Method,
    Certificate,
    Client,
    Result as HttpResult,
    Response,
    Error as HttpError,
    RequestBuilder,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use crate::utils;
use crate::api::KubeKind;
use crate::config::{ClientConfig, AuthConfig};

// the name and location for in-cluster runtime configuration parameters
const INCLUSTER_CA_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt";
const INCLUSTER_TOKEN_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";
const INCLUSTER_API_HOST_NAME: &str = "KUBERNETES_SERVICE_HOST";
const INCLUSTER_API_PORT_NAME: &str = "KUBERNETES_SERVICE_PORT";

#[derive(Clone)]
pub struct KubeClient {
    auth_info: AuthConfig,
    api_url: String,
    client: Client,
}

// produce a base HTTP URI from the given host and port
fn join_host_port(host: &str, port: &str) -> String {
    // handle IPv6 addresses
    if host.contains(':') || host.contains('%') {
        format!("https://[{}]:{}", host, port)
    } else {
        format!("https://{}:{}", host, port)
    }
}

/// Error type used to represent errors that can occur during calls to the Kubernetes API
#[derive(Debug)]
pub enum RequestError {
    /// A low-level HTTP error
    TransportError(HttpError),
    /// A successful request was made but it did not contain the expected response
    HttpError(Response),
    /// The response could not be deserialized
    SerdeError(JsonError),
    /// Other misc. error
    MiscError,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for RequestError {
    fn description(&self) -> &str {
        match self {
            RequestError::TransportError(_) =>
                "There was an error during the HTTP request",
            RequestError::HttpError(_) =>
                "Received an unexpected response from the Kubernetes API",
            RequestError::SerdeError(_) =>
                "Error during deserialization of response body",
            RequestError::MiscError =>
                "Unknown, miscellaneous error (shouldn't happen)",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match self {
            RequestError::TransportError(error) => Some(error as &StdError),
            RequestError::HttpError(_) => None,
            RequestError::SerdeError(error) => Some(error as &StdError),
            RequestError::MiscError => None,
        }
    }
}

pub type RequestResult<T> = Result<T, RequestError>;

/// Errors that can occur during client initialization
#[derive(Debug)]
pub enum ClientInitError {
    /// The env var that was being looked up and the reason the lookup failed
    EnvVarError(String, ::std::env::VarError),
    /// IO error while attempting to read a file
    IoError(IoError),
    /// Certificate parsing error
    InvalidCert(HttpError),
    /// Low-level HTTP client-building error
    ClientBuildingError(HttpError),
}

impl KubeClient {
    pub fn new(config: ClientConfig) -> Result<KubeClient, ClientInitError> {
        let mut builder = Client::builder();

        match config {
            ClientConfig::InCluster => {
                let err_mapper = |var_name| |error| ClientInitError::EnvVarError(var_name, error);
                // TODO: convert ca.crt from PEM to DER
                let host = env::var(INCLUSTER_API_HOST_NAME).map_err(err_mapper(INCLUSTER_API_HOST_NAME.to_string()))?;
                let port = env::var(INCLUSTER_API_PORT_NAME).map_err(err_mapper(INCLUSTER_API_PORT_NAME.to_string()))?;
                
                let token_file_contents = &utils::read_file(INCLUSTER_TOKEN_FILE).map_err(ClientInitError::IoError)?;
                let token = String::from_utf8_lossy(token_file_contents).into_owned();

                let ca_file = utils::read_file(INCLUSTER_CA_FILE)
                                    .map_err(ClientInitError::IoError)?;
                let ca = Certificate::from_pem(&ca_file)
                                     .map_err(ClientInitError::InvalidCert)?;

                KubeClient::new(ClientConfig::External { 
                    auth_info: AuthConfig::Token(token),
                    api_url: join_host_port(&host, &port),
                    ca: Some(ca),
                })
            },
            ClientConfig::External { api_url, auth_info, ca } => {
                if let Some(ca) = ca {
                    debug!("Adding CA cert");
                    builder = builder.add_root_certificate(ca);
                }
                let client = builder.build().map_err(ClientInitError::ClientBuildingError)?;
                Ok(KubeClient {
                    api_url, auth_info, client
                })
            }
        }
    }

    // TODO: more deduplication?
    // problem is, macros can't generate idents, so can't generate the methods with it

    // XXX: The create methods assume that the same type is returned from the API servers as the
    // one that's sent to it. I'm not sure if that's necessarily true, but since default type
    // parameters are being phased out (? https://github.com/rust-lang/rust/issues/36887) we can't
    // write <Input: KubeKind, Output = Input> so that the result defaults to the input, and since
    // the general case is the same output as input it's easier to restrict this for now.

    // Cluster methods
    pub fn create_cluster_resource<T: KubeKind>(&self, resource: &T) -> RequestResult<T>
    {
        self.post_object(&produce_path::<T>(None, None), resource)
    }

    pub fn replace_cluster_resource<T: KubeKind>(&self, name: &str, resource: &T) -> RequestResult<T> {
        self.put_object(&produce_path::<T>(None, Some(name)), resource)
    }

    pub fn patch_cluster_resource<T: KubeKind>(&self, name: &str, resource: &T) -> RequestResult<T> {
        self.patch_object(&produce_path::<T>(None, Some(name)), resource)
    }

    pub fn list_cluster_resource<T: KubeKind>(&self) -> RequestResult<T::List> {
        self.get_object(&produce_path::<T>(None, None))
    }

    pub fn get_cluster_resource<T: KubeKind>(&self, name: &str) -> RequestResult<T> {
        self.get_object(&produce_path::<T>(None, Some(name)))
    }

    pub fn delete_cluster_resource<T: KubeKind>(&self, name: &str) -> RequestResult<T> {
        self.delete_object(&produce_path::<T>(None, Some(name)))
    }

    // Namepsaced methods
    pub fn create_namespaced_resource<T: KubeKind>(&self, namespace: &str, resource: &T) -> RequestResult<T> {
        self.post_object(&produce_path::<T>(Some(namespace), None), resource)
    }

    pub fn replace_namespaced_resource<T: KubeKind>(&self, namespace: &str, name: &str, resource: &T) -> RequestResult<T> {
        self.put_object(&produce_path::<T>(Some(namespace), Some(name)), resource)
    }

    pub fn patch_namespaced_resource<T: KubeKind>(&self, namespace: &str, name: &str, resource: &T) -> RequestResult<T> {
        self.patch_object(&produce_path::<T>(Some(namespace), Some(name)), resource)
    }

    pub fn get_namespaced_resource<T: KubeKind>(&self, namespace: &str, name: &str) -> RequestResult<T> {
        self.get_object(&produce_path::<T>(Some(namespace), Some(name)))
    }

    pub fn list_namespaced_resource<T: KubeKind>(&self, namespace: &str) -> RequestResult<T::List> {
        self.get_object(&produce_path::<T>(Some(namespace), None))
    }

    pub fn delete_namespaced_resource<T: KubeKind>(&self, namespace: &str, name: &str) -> RequestResult<T> {
        self.delete_object(&produce_path::<T>(Some(namespace), Some(name)))
    }

    // Low level methods
    pub fn get_object<T: DeserializeOwned>(&self, path: &str) -> RequestResult<T> {
        deserialize_api_response(self.request_path::<()>(Method::GET, path, None))
    }

    pub fn post_object<T: Serialize, U: DeserializeOwned>(&self, path: &str, object: &T) -> RequestResult<U> {
        deserialize_api_response(self.request_path(Method::POST, path, Some(object)))
    }

    pub fn put_object<T: Serialize, U: DeserializeOwned>(&self, path: &str, object: &T) -> RequestResult<U> {
        deserialize_api_response(self.request_path(Method::PUT, path, Some(object)))
    }

    pub fn patch_object<T: Serialize, U: DeserializeOwned>(&self, path: &str, object: &T) -> RequestResult<U> {
        deserialize_api_response(self.request_path(Method::PATCH, path, Some(object)))
    }

    pub fn delete_object<T: DeserializeOwned>(&self, path: &str) -> RequestResult<T> {
        deserialize_api_response(self.request_path::<()>(Method::DELETE, path, None))
    }

    fn authorize_request(&self, request: RequestBuilder) -> RequestBuilder {
        // TODO: add logic for different auth methods
        match self.auth_info {
            AuthConfig::Token(ref bearer) =>
                request.header(AUTHORIZATION, format!("Bearer {}", bearer.clone())),
            _ => unimplemented!(),
        }
    }

    fn request_path<T: Serialize>(&self, method: Method, path: &str, body: Option<&T>) -> HttpResult<Response> {
        let uri = format!("{}{}", self.api_url, path);
        let is_patching = method == Method::PATCH;
        let mut request = self.authorize_request(self.client.request(method, &uri));
        if let Some(body) = body {
            request = request.json(body);
        }
        // TODO: this is very heavy handed, there's probably a more flexible way to do this
        if is_patching {
            request = request.header(CONTENT_TYPE, "application/strategic-merge-patch+json");
        }
        request.send()
    }
}

// TODO: how should we deal with subresources?
fn produce_path<T: KubeKind>(namespace: Option<&str>, resource: Option<&str>) -> String {
    // First parameter is API path, which consists of:
    // /api/<version> for core
    // /apis/<api group>/<version> for everything else
    // Second parameter is namespacing information
    // Third is kind name
    // Finally, an optional object name
    format!("/{api}/{namespace}{kind}/{object}",
            api = if T::API_GROUP == "core" {
                format!("api/{}", T::API_VERSION)
            } else {
                format!("apis/{}/{}", T::API_GROUP, T::API_VERSION)
            },
            namespace = if let Some(namespace) = namespace {
                format!("namespaces/{}/", namespace)
            } else {
                "".to_string()
            },
            object = resource.unwrap_or(""),
            kind = T::KIND_NAME)
}

fn deserialize_api_response<T: DeserializeOwned>(response: HttpResult<Response>) -> RequestResult<T> {
    response.map_err(RequestError::TransportError)
            .and_then(|response| {
                if response.status().is_success() {
                    serde_json::from_reader(response).map_err(RequestError::SerdeError)
                } else {
                    Err(RequestError::HttpError(response))
                }
            })
}
