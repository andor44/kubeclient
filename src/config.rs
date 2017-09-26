use reqwest::Certificate;
use utils;
use pem_parser;
use std::fmt;

#[derive(Debug)]
pub enum ClientConfig {
    InCluster,
    External {
        api_url: String,
        auth_info: AuthConfig,
        ca: Option<Certificate>,
    }
}

#[derive(Debug)]
pub enum KubeconfigParseError {
    MissingContext(String),
    MissingUser(String),
    MissingCluster(String),
    MissingFile(String),
    InvalidCertificate(String),
}

impl ClientConfig {
    // XXX: jfc, this sucks
    pub fn from_kubeconfig(config: kubeconfig::Config, 
                           context: Option<String>) -> Result<ClientConfig, KubeconfigParseError> {
        // Take current_context from the config file, or if the user specified one prefer that
        let context_name = context.unwrap_or_else(|| config.current_context.clone());

        // Find the desired context
        let context = &config.contexts.iter()
                                      .find(|context| context.name == context_name)
                                      .ok_or_else(|| KubeconfigParseError::MissingContext(context_name))?.context;
        // Find the cluster referenced in the context
        let cluster = &config.clusters.iter()
                                      .find(|cluster| cluster.name == context.cluster)
                                      .ok_or_else(|| KubeconfigParseError::MissingCluster(context.cluster.clone()))?.cluster;
        // Find the user referenced in the context
        let user = &config.users.iter()
                                .find(|user| user.name == context.user)
                                .ok_or_else(|| KubeconfigParseError::MissingUser(context.user.clone()))?.user;
        // If the cluster specified a CA cert retrieve that here
        let ca = if let Some(ref _ca_data) = cluster.certificate_authority_data {
            // CA specified inline as base64 PEM
            unimplemented!("Inline CA data not implemented yet");
        } else if let Some(ca_path) = cluster.certificate_authority.as_ref() {
            // Path given to a certificate file
            let ca_pem = utils::read_file(ca_path).map_err(|_| KubeconfigParseError::MissingFile(ca_path.clone()))?;
            Some(Certificate::from_der(&pem_parser::pem_to_der(String::from_utf8_lossy(&ca_pem).as_ref()))
                             .map_err(|_| KubeconfigParseError::InvalidCertificate(ca_path.clone()))?)
        } else { None };
        Ok(ClientConfig::External {
            api_url: cluster.server.clone(),
            auth_info: AuthConfig::from_kubeconfig_user(user),
            ca: ca,
        })
    }
}

#[derive(Clone)]
pub enum AuthConfig {
    Token(String),
    ClientCertificate,
    BasicAuth,
}

impl fmt::Debug for AuthConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            AuthConfig::Token(_) => write!(f, "Token authentication"),
            _ => unimplemented!("Other auth methods are not yet implemented"),
        }
    }
}

impl AuthConfig {
    pub fn from_kubeconfig_user(user: &kubeconfig::AuthInfo) -> AuthConfig {
        // TODO: other auth methods from kubeconfig files
        if let Some(ref token) = user.token {
            return AuthConfig::Token(token.clone());
        }
        unimplemented!("Reading other auth methods from kubeconfigs is not implemented yet");
    }
}

pub mod kubeconfig {
    use std::path::Path;
    use std::fs::File;
    use serde_yaml;

    pub fn read_config<T: AsRef<Path>>(path: T) -> Config {
        // TODO: error handling
        let file = File::open(path).expect("unable to open file");
        serde_yaml::from_reader(file).unwrap()
    }

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub kind: String,
        #[serde(rename = "apiVersion")]
        pub api_version: String,
        pub preferences: Preferences,
        pub clusters: Vec<NamedCluster>,
        pub users: Vec<NamedAuthInfo>,
        pub contexts: Vec<NamedContext>,
        #[serde(rename = "current-context")]
        pub current_context: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Preferences {
        pub colors: Option<bool>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Cluster {
        pub server: String,
        #[serde(rename = "insecure-skip-tls-verify")]
        pub insecure_skip_tls_verify: Option<bool>,
        #[serde(rename = "certificate-authority")]
        pub certificate_authority: Option<String>,
        #[serde(rename = "certificate-authority-data")]
        pub certificate_authority_data: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NamedCluster {
        pub name: String,
        pub cluster: Cluster,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AuthInfo {
        // Client certificate authentication
        #[serde(rename = "client-certificate")]
        pub client_certificate: Option<String>,
        #[serde(rename = "client-certificate-data")]
        pub client_certificate_data: Option<String>,
        #[serde(rename = "client-key")]
        pub client_key: Option<String>,
        #[serde(rename = "client-key-data")]
        pub client_key_data: Option<String>,

        // Token auth
        pub token: Option<String>,
        #[serde(rename = "tokenFile")]
        pub token_file: Option<String>,

        // Impersonation
        #[serde(rename = "as")]
        pub impersonate: Option<String>,
        #[serde(rename = "as-groups")]
        pub impersonate_groups: Option<Vec<String>>,

        // Basic auth
        pub username: Option<String>,
        pub password: Option<String>,

        // TODO: authprovider plugins
    }

    #[derive(Serialize, Deserialize)]
    pub struct NamedAuthInfo {
        pub name: String,
        pub user: AuthInfo,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Context {
        pub cluster: String,
        pub user: String,
        pub namespace: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NamedContext {
        pub name: String,
        pub context: Context,
    }
}
