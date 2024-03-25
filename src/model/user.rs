use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, MySql, Row, Transaction};
use std::clone::Clone;
// 引入全局变量
use super::DB_POOL;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub enable: i8,
    #[allow(non_snake_case)]
    pub createTime: DateTime<Utc>,
    #[allow(non_snake_case)]
    pub updateTime: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: String::default(),
            password: String::default(),
            enable: 0,
            createTime: Utc::now(),
            updateTime: Utc::now(),
        }
    }
}

// 新增用户（需要加事务，所以 pool 从外面传进来）
pub async fn add_user_by_transaction(
    pool: &mut Transaction<'_, MySql>,
    data: User,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let insert_sql = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.username)
        .bind(&data.password)
        .bind(&data.enable)
        .bind(&data.createTime)
        .bind(&data.updateTime)
        .execute(pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

// 删除记录-通过 id
pub async fn delete_by_id(id: i64) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query("delete from user where id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 0 }
    Ok(result)
}

// 更新记录-通过 id
pub async fn update_username_by_id(
    username: String,
    id: i64,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query("update user set username = ? where id = ?")
        .bind(&username)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

// 查询一条记录-通过 id
pub async fn find_one_by_id(id: i64) -> Result<Option<User>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query_as::<_, User>("SELECT * FROM user where id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    Ok(result)
}

// 查询多条记录
pub async fn fetch_all() -> Result<Vec<User>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query_as::<_, User>("SELECT * FROM `user`")
        .fetch_all(&pool)
        .await?;
    Ok(result)
}

#[derive(Debug)]
pub struct PageReq {
    #[allow(non_snake_case)]
    pub pageNo: Option<i64>,
    //  可传：默认1
    #[allow(non_snake_case)]
    pub pageSize: Option<i64>,
    //  可传：默认10
    pub username: Option<String>,
    //  可传
    pub enable: Option<i64>, //  可传
}

// 动态绑定条件查询所有
pub async fn fetch_all_by_dynamic_parameter_req(req: PageReq) -> Result<Vec<User>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    // 构建 SQL 查询语句
    let mut sql_str = "SELECT * FROM `user`".to_string();
    let mut params: Vec<String> = Vec::new();
    if req.enable.is_some() || req.username.is_some() {
        sql_str.push_str(" WHERE");
        let mut conditions: Vec<String> = Vec::new();
        if let Some(enable) = req.enable {
            conditions.push(" enable=? ".to_string());
            params.push((&enable).to_string());
        }
        if let Some(name) = req.username.as_ref() {
            conditions.push(" `username` like ? ".to_string());
            params.push(format!("%{}%", name));
        }
        sql_str.push_str(&conditions.join(" AND"));
    }
    sql_str.push_str(" order by id desc LIMIT ? OFFSET ? ");
    let limit = req.pageSize.unwrap_or(10);
    let offset = (req.pageNo.unwrap_or(1) - 1) * 10;

    let query_builder = sqlx::query(&sql_str);
    let mut with_params = query_builder;
    for par in &params {
        with_params = with_params.bind(par);
    }
    with_params = with_params.bind(limit).bind(offset);
    // you can debug : Open comments below
    // print!("sql={:?},",sql_str);
    let result = with_params.fetch_all(&pool).await?;
    let mut list: Vec<User> = Vec::new();
    for row in result {
        let l = User {
            // 从数据库行中提取信息并创建 Profile 对象
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            enable: row.get("enable"),
            createTime: row.get("createTime"),
            updateTime: row.get("updateTime"),
        };
        list.push(l);
    }
    Ok(list)
}
