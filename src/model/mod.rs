use dotenv::dotenv;
use std::env;

use lazy_static::lazy_static;
use sqlx::mysql::MySqlPoolOptions;
use std::sync::{Arc, Mutex};

// 引入 user model
pub mod user;

// 定义懒加载全局变量
lazy_static! {
    pub static ref DB_POOL: Arc<Mutex<Option<sqlx::MySqlPool>>> = Arc::new(Mutex::new(None));
}
// 连接数据库
async fn init_pool() -> Result<sqlx::MySqlPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    Ok(pool)
}

// 初始化连接池
pub async fn mysql_connect() {
    let pool = init_pool().await.unwrap();
    // 存储连接池到全局变量
    *DB_POOL.lock().unwrap() = Some(pool);
}

// 释放连接池
pub async fn mysql_disconnect() -> Result<(), sqlx::Error> {
    if let Some(pool) = DB_POOL.lock().unwrap().take() {
        pool.close().await;
    }
    Ok(())
}
