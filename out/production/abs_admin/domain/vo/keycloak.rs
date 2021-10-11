
use serde::{Deserialize, Serialize};

///keycloak 秘钥
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keys {
    pub keys: Vec<Key>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    pub kid: Option<String>,
    pub kty: Option<String>,
    pub alg: Option<String>,
    #[serde(rename="use")]
    pub use1: Option<String>,
    pub n: Option<String>,
    pub e: Option<String>,
    pub x5c: Vec<String>,
    pub x5t: Option<String>,
    #[serde(rename="x5t#S256")]
    pub x5t_s256:Option<String>,
}

