use std::env;
use pem_parser;
use serde_json;
use reqwest;
use std::io::Error as IoError;
use serde::Serialize;
use serde_json::Error as JsonError;
use reqwest::{Method, Certificate, Client, Result as HttpResult, Response, Error as HttpError};
use reqwest::header::{Authorization, Bearer};

use utils;
use api::KubeKind;
use config::{ClientConfig, AuthConfig};

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

fn join_host_port(host: &str, port: &str) -> String {
    // handle IPv6 addresses
    if host.contains(':') || host.contains('%') {
        format!("https://[{}]:{}", host, port)
    } else {
        format!("https://{}:{}", host, port)
    }
}

#[derive(Debug)]
pub enum RequestError {
    TransportError(HttpError),
    HttpError(Response),
    SerdeError(JsonError),
    MiscError,
}

type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug)]
pub enum ClientInitError {
    EnvVarError(String, ::std::env::VarError),
    IoError(IoError),
    InvalidCert(HttpError),
    ClientBuildingError(HttpError),
}

impl KubeClient {
    pub fn new(config: ClientConfig) -> Result<KubeClient, ClientInitError> {
        let mut builder = Client::builder().map_err(ClientInitError::ClientBuildingError)?;

        match config {
            ClientConfig::InCluster => {
                let err_mapper = |var_name| |error| ClientInitError::EnvVarError(var_name, error);
                // TODO: convert ca.crt from PEM to DER
                let host = env::var(INCLUSTER_API_HOST_NAME).map_err(err_mapper(INCLUSTER_API_HOST_NAME.to_string()))?;
                let port = env::var(INCLUSTER_API_PORT_NAME).map_err(err_mapper(INCLUSTER_API_PORT_NAME.to_string()))?;
                
                let token_file_contents = &utils::read_file(INCLUSTER_TOKEN_FILE).map_err(ClientInitError::IoError)?;
                let token = String::from_utf8_lossy(token_file_contents).into_owned();

                let ca_file = utils::read_file(INCLUSTER_CA_FILE).expect("unable to read k8s ca file");
                // This is guaranteed to be PEM-encoded, therefore valid UTF8
                let ca_file = String::from_utf8(ca_file).expect("invalid PEM data in k8s CA file?");
                let ca = Certificate::from_der(&pem_parser::pem_to_der(&ca_file))
                                     .map_err(ClientInitError::InvalidCert)?;

                KubeClient::new(ClientConfig::External { 
                    auth_info: AuthConfig::Token(token),
                    api_url: join_host_port(&host, &port),
                    ca: Some(ca),
                })
            },
            ClientConfig::External { api_url, auth_info, ca } => {
                if let Some(ca) = ca {
                    builder.add_root_certificate(ca).map_err(ClientInitError::ClientBuildingError)?;
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
    pub fn create_cluster_object<T: KubeKind>(&self, object: &T) -> RequestResult<T> {
        self.post_object(&format!("/api/v1/{}", T::KIND_NAME), object)
    }

    pub fn update_cluster_object<T: KubeKind>(&self, name: &str, object: &T) -> RequestResult<T> {
        self.put_object(&format!("/api/v1/{}/{}", T::KIND_NAME, name), object)
    }

    pub fn list_cluster_objects<T: KubeKind>(&self) -> RequestResult<T> {
        self.get_object(&format!("/api/v1/{}", T::KIND_NAME))
    }

    pub fn get_cluster_object<T: KubeKind>(&self, name: &str) -> RequestResult<T> {
        self.get_object(&format!("/api/v1/{}/{}", T::KIND_NAME, name))
    }

    pub fn delete_cluster_object<T: KubeKind>(&self, name: &str) -> RequestResult<T> {
        self.delete_object(&format!("/api/v1/{}/{}", T::KIND_NAME, name))
    }

    // Namepsaced methods
    pub fn create_namespaced_object<T: KubeKind>(&self, namespace: &str, object: &T) -> RequestResult<T> {
        self.post_object(&format!("/api/v1/namespace/{}/{}", namespace, T::KIND_NAME), object)
    }

    pub fn update_namespaced_object<T: KubeKind>(&self, namespace: &str, name: &str, object: &T) -> RequestResult<T> {
        self.put_object(&format!("/api/v1/namespace/{}/{}/{}", namespace, T::KIND_NAME, name), object)
    }

    pub fn get_namespaced_object<T: KubeKind>(&self, namespace: &str, name: &str) -> RequestResult<T> {
        self.get_object(&format!("/api/v1/namespaces/{}/{}/{}", namespace, T::KIND_NAME, name))
    }

    pub fn list_namespaced_objects<T: KubeKind>(&self, namespace: &str) -> RequestResult<T> {
        self.get_object(&format!("/api/v1/namespaces/{}/{}", namespace, T::KIND_NAME))
    }

    pub fn delete_namespaced_objects<T: KubeKind>(&self, namespace: &str, name: &str) -> RequestResult<T> {
        self.delete_object(&format!("/api/v1/namespaces/{}/{}/{}", namespace, T::KIND_NAME, name))
    }

    // Low level methods
    pub fn get_object<T: KubeKind>(&self, path: &str) -> Result<T, RequestError> {
        deserialize_api_response(self.request_path::<()>(Method::Get, path, None))
    }

    pub fn post_object<T: KubeKind, U: KubeKind>(&self, path: &str, object: &T) -> RequestResult<U> {
        deserialize_api_response(self.request_path(Method::Post, path, Some(object)))
    }

    pub fn put_object<T: KubeKind, U: KubeKind>(&self, path: &str, object: &T) -> RequestResult<U> {
        deserialize_api_response(self.request_path(Method::Put, path, Some(object)))
    }

    pub fn delete_object<T: KubeKind>(&self, path: &str) -> Result<T, RequestError> {
        deserialize_api_response(self.request_path::<()>(Method::Delete, path, None))
    }

    fn authorize_request(&self, request: &mut reqwest::RequestBuilder) {
        // TODO: add logic for different auth methods
        match self.auth_info {
            AuthConfig::Token(ref bearer) => request.header(Authorization(Bearer { token: bearer.clone() })),
            _ => unimplemented!(),
        };
    }

    fn request_path<T: Serialize>(&self, method: Method, path: &str, body: Option<&T>) -> HttpResult<Response> {
        let uri = format!("{}{}", self.api_url, path);
        let mut request = self.client.request(method, &uri).expect("unable to create RequestBuilder");
        self.authorize_request(&mut request);
        if let Some(body) = body {
            request.json(body)?;
        }
        request.send()
    }
}

fn deserialize_api_response<T: KubeKind>(response: HttpResult<Response>) -> RequestResult<T> {
    response.map_err(RequestError::TransportError)
            .and_then(|response| {
                if response.status().is_success() {
                    serde_json::from_reader(response).map_err(RequestError::SerdeError)
                } else {
                    Err(RequestError::HttpError(response))
                }
            })
}
