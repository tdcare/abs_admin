use rbatis::rbatis::Rbatis;
use async_std::task;
use hyper::*;

mod cache_service;
mod mem_service;
mod redis_service;
mod sys_config_service;
mod sys_dict_service;
mod sys_res_service;
mod sys_role_res_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_user_role_service;
mod sys_user_service;

pub use crate::config::app_config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
pub use redis_service::*;
pub use sys_config_service::*;
pub use sys_dict_service::*;
pub use sys_res_service::*;
pub use sys_role_res_service::*;
pub use sys_role_service::*;
pub use sys_sms_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;
use crate::domain::vo::Keys;


pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
    pub sys_res_service: SysResService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_res_service: SysRoleResService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_dict_service: SysDictService,
    pub keycloak_keys: Keys,
}

impl Default for ServiceContext {
     fn default() -> Self {
        let config = ApplicationConfig::default();
        match config.cache_type.as_str() {
            "mem" => {
                println!("[abs_admin] cache_type: mem");
            }
            "redis" => {
                println!("[abs_admin] cache_type: redis");
            }
            e => {
                panic!("[abs_admin] unsupport cache type of {}", e);
            }
        }


        ServiceContext {
            keycloak_keys: task::block_on(async{
                get_keycloak_keys(&config).await
            }),
            rbatis: rbatis::core::runtime::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            cache_service: CacheService::new(&config),
            sys_res_service: SysResService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_res_service: SysRoleResService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_service: SysDictService {},

            config,

        }
    }
}
pub async fn get_keycloak_keys(config: &ApplicationConfig)->Keys{
    println!("[abs_admin] 开始访问keycloak");
    let client = Client::new();

    let res = client.get(Uri::from_static("http://122.9.125.181/auth/realms/tdcare/protocol/openid-connect/certs")).await.unwrap();

    println!("status: {}", res.status());

    let buf = hyper::body::to_bytes(res).await.unwrap();

    println!("body: {:?}", buf);
    let  keys= serde_json::from_slice::<Keys>(&buf).unwrap();

    println!("[abs_admin] 取到了keycloak certs");
    return keys;
}
lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
