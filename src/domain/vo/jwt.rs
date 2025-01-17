/*
 * @Author: tzw
 * @Date: 2021-07-21 22:45:16
 * @LastEditors: tzw
 * @LastEditTime: 2021-10-12 00:13:44
 */
use crate::error::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::domain::vo::Keys;
use std::str::FromStr;

/// JWT 鉴权 Token结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    //账号id
    pub id: String,
    //账号
    // pub account: String,
    // //权限集合
    // pub permissions: Vec<String>,
    // //角色id集合
    // pub role_ids: Vec<String>,
    //过期时间
    pub exp: usize,
	pub iat: usize,
	pub jti: String,
	pub iss: String,
	pub sub: String,
	pub typ: String,
	pub azp: String,
	pub session_state: String,
	pub acr: String,
	// // "realm_access": {
	// // 	"roles": ["offline_access", "nurse", "uma_authorization", "user"]
	// // },
	// // "resource_access": {
	// // 	"tdnis": {
	// // 		"roles": ["nurseManager", "user"]
	// // 	}
	// // },
	pub scope: String,
	pub departmentName: String,
	// pub address: String,
	pub departmentCode: String,
	pub departmentId: String,
	pub roles: Vec<String>,
	pub groups: Vec<String>,
	pub dept: Vec<String>,
	pub preferred_username: String,
	pub given_name: String,
	pub userId: String,
	pub name: String,
	pub departmentAbstract: String,
}

impl JWTToken {
    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let validation = Validation {
            ..Validation::default()
        };
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                _ => return Err(Error::from("InvalidToken other errors")),
            },
        };
    }

    pub fn verify_with_keycloak(keycloak: &Keys ,token: &str) -> Result<JWTToken, Error>{
            let n=keycloak.keys[0].n.as_ref().unwrap();
            let e=keycloak.keys[0].e.as_ref().unwrap();
            let kty =keycloak.keys[0].kty.as_ref().unwrap();
            let alg=keycloak.keys[0].alg.as_ref().unwrap();
            
            let algorithm=  Algorithm::from_str(&alg).unwrap();
            
            let validation = Validation::new(algorithm);

            let jwt_token =decode::<JWTToken>(
                &token,
                &DecodingKey::from_rsa_components(n,e),
                &validation,
            );
            
            return match jwt_token  {
                Ok(c) => Ok(c.claims),
                Err(err) => match *err.kind() {
                    ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                    ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                    _ => return Err(Error::from("InvalidToken other errors")),
                },
            };
    }

}
