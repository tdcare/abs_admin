use crate::config::app_config::ApplicationConfig;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
use crate::domain::vo::Keys;

pub mod mapper;

pub async fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let mut rbatis = Rbatis::new();
    //logic plugin 设置逻辑删除插件
    rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt(
        &config.logic_column,
        config.logic_deleted as i32,
        config.logic_un_deleted as i32,
    )));
    if config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    //连接数据库
    println!("[abs_admin] rbatis 开始 link database({})...",&config.database_url[0..config.database_url.find(":").unwrap_or(0)]);
    rbatis
        .link(&config.database_url)
        .await
        .expect("[abs_admin] rbatis link database fail!");
    println!("[abs_admin] rbatis 连接 database 成功!");
    return rbatis;
}

