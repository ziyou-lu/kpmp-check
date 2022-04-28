use crate::config::Config;
use rbatis::plugin::logic_delete::{RbatisLogicDeletePlugin};
use rbatis::rbatis::Rbatis;
use crate::CONTEXT;

pub mod mapper;

pub fn init_rbatis(config: &Config) -> Rbatis {
    let mut rbatis = Rbatis::new();
    //设置逻辑删除插件
    rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt(
        &config.db.logic_column,
        config.db.logic_deleted as i32,
        config.db.logic_un_deleted as i32,
    )));
    //连接数据库
    println!("rbatis link database success!");
    return rbatis;
}
