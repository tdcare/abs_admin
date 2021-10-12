/*
 * @Author: tzw
 * @Date: 2021-07-21 22:45:16
 * @LastEditors: tzw
 * @LastEditTime: 2021-10-12 15:59:26
 */

use serde::{Deserialize, Serialize};
use hyper::*;
use crate::config::{self, app_config::ApplicationConfig};

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

pub async fn get_keycloak_keys(config: &ApplicationConfig)->Keys{
    println!("[abs_admin] 开始访问keycloak");
    let client = Client::new();
    let c=&config.keycloak_auth_server_certs;
    
    let res = client.get(<Uri as std::str::FromStr>::from_str(&c).unwrap()).await.unwrap();

    // println!("status: {}", res.status());

    let buf = hyper::body::to_bytes(res).await.unwrap();

    // println!("body: {:?}", buf);
    let  keys= serde_json::from_slice::<Keys>(&buf).unwrap();

    println!("[abs_admin] 取到了keycloak certs");
    return keys;
}
