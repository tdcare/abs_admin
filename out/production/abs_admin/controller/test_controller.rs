use actix_web::{web, HttpRequest, Responder};

use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserRolePageDTO};
use crate::domain::vo::{JWTToken, RespVO,Keys};
use crate::service::CONTEXT;
use std::collections::HashMap;
use serde_json::Value;


/// 用户信息
pub async fn info(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("access_token");

    let resp = reqwest::get("http://122.9.125.181/auth/realms/tdcare/protocol/openid-connect/certs")
        .await
        .unwrap()
         .json::<Keys>()
       // .text()
        .await
        .unwrap()
        ;
    // let jsonString=resp.clone();
    // let json= serde_json::from_slice::<Keys>(&jsonString.as_bytes());
    // let v:Value =serde_json::from_str(&jsonString).unwrap();
    // let user=&v["keys"][0]["use"];
    // let x5c=&v["keys"][0]["x5c"][0];
    // let s=user.as_str().unwrap();
    // let x5c_string=x5c.as_str().unwrap();
    // .json::<HashMap<String, String>>();
     return RespVO::from(&resp).resp_json();

    // match token {
    //     Some(token) => {
    //         let token = token.to_str().unwrap_or("");
    //         let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    //         if token.is_err() {
    //             return RespVO::from_result(&token).resp_json();
    //         }
    //         let user_data = CONTEXT
    //             .sys_user_service
    //             .get_user_info_by_token(&token.unwrap())
    //             .await;
    //         return RespVO::from_result(&user_data).resp_json();
    //     }
    //     _ => {
    //         return RespVO::<String>::from_error_info("access_token is empty!", "").resp_json();
    //     }
    // }
}

