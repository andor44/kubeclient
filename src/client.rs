use std::env;
use pem_parser;
use serde_json;
use reqwest;
use serde::Serialize;
use serde_json::Error as JsonError;
use reqwest::{Method, Certificate, Client, Result as HttpResult, Response, Error as HttpError};
use reqwest::header::{Authorization, Bearer};

use utils;
use api::KubeKind;

const INCLUSTER_CA_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt";
const INCLUSTER_TOKEN_FILE: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";
const INCLUSTER_API_HOST_NAME: &str = "KUBERNETES_SERVICE_HOST";
const INCLUSTER_API_PORT_NAME: &str = "KUBERNETES_SERVICE_PORT";

pub enum KubeClientConfig {
    InCluster,
    External {
        api_url: String,
        auth_info: KubeAuthConfig,
        ca: Certificate,
    }
}

#[derive(Clone)]
pub enum KubeAuthConfig {
    Token(Bearer),
    ClientCertificate,
    BasicAuth,
}

#[derive(Clone)]
pub struct KubeClient {
    auth_info: KubeAuthConfig,
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
    HttpError(HttpError),
    SerdeError(JsonError),
    MiscError,
}

type RequestResult<T> = Result<T, RequestError>;

impl KubeClient {
    pub fn new(config: KubeClientConfig) -> Option<KubeClient> {
        // TODO: proper error handling
        let mut builder = Client::builder().expect("can't initialize client builder?");

        match config {
            KubeClientConfig::InCluster => {
                // TODO: convert ca.crt from PEM to DER
                let host = env::var(INCLUSTER_API_HOST_NAME).expect("invalid k8s service host?");
                let port = env::var(INCLUSTER_API_PORT_NAME).expect("invalid k8s service port?");
                
                let token_file_contents = &utils::read_file(INCLUSTER_TOKEN_FILE).expect("unable to read k8s token file");
                let token = String::from_utf8_lossy(token_file_contents).into_owned();

                let ca_file = utils::read_file(INCLUSTER_CA_FILE).expect("unable to read k8s ca file");
                // This is guaranteed to be PEM-encoded, therefore valid UTF8
                let ca_file = String::from_utf8(ca_file).expect("invalid PEM data in k8s CA file?");
                let ca = Certificate::from_der(&pem_parser::pem_to_der(&ca_file));

                KubeClient::new(KubeClientConfig::External { 
                    auth_info: KubeAuthConfig::Token(Bearer { token }),
                    api_url: join_host_port(&host, &port),
                    ca: ca.expect("k8s CA file contains no valid cert?"),
                })
            },
            KubeClientConfig::External { api_url, auth_info, ca } => {
                let client = builder.add_root_certificate(ca)
                                    .expect("supplied CA is not valid?")
                                    .build()
                                    .expect("unable to build client");
                Some(KubeClient {
                    api_url, auth_info, client
                })
            }
        }
    }

    // TODO: more deduplication?
    // problem is, macros can't generate idents, so can't generate the methods with it

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
            KubeAuthConfig::Token(ref bearer) => request.header(Authorization(bearer.clone())),
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
    response.map_err(RequestError::HttpError)
            .and_then(|response| serde_json::from_reader(response).map_err(RequestError::SerdeError))
}
