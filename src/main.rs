use chrono::Utc;

mod model;

use crate::model::user::User;
use crate::model::DB_POOL;

#[tokio::main]
async fn main() {
    // Initializing a database connection
    let _ = model::mysql_connect().await;

    example_fetch_all().await;
    // example_add_user_by_transaction().await;
    // example_delete_by_id().await;

    // Close database connection
    let _ = model::mysql_disconnect().await;
}

async fn example_add_user_by_transaction() {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            println!("\nErr is  \n开启事务失败:{:?}\n", err);
            return;
        }
    };
    // 初始化用户结构体
    let user_data = User {
        id: 0,
        username: "your username".to_string(),
        password: "your password".to_string(),
        enable: 1,
        createTime: Utc::now(),
        updateTime: Utc::now(),
    };
    match model::user::add_user_by_transaction(&mut tx, user_data).await {
        Ok(res) => {
            println!("\n add res is \n{:?}\n", res)
        }
        Err(err) => {
            println!("\n add Err is  \n{:?}\n", err);
            if let Err(rollback_err) = tx.rollback().await {
                println!("\n tx rollback Err is  \n事务提交失败:{:?}\n", rollback_err);
                return;
            }
            return;
        }
    }
    if let Err(commit_err) = tx.commit().await {
        println!("\n tx commit Err is  \n事务提交失败:{:?}\n", commit_err);
        return;
    }
    println!("add new user is succession");
    /* println
    add res is
    MySqlQueryResult { rows_affected: 1, last_insert_id: 20 }
    add new user is succession
    */
}

async fn example_delete_by_id() {
    match model::user::delete_by_id(18).await {
        Ok(res) => {
            println!("\n res is \n{:?}\n", res)
        }
        Err(err) => {
            println!("\nErr is  \n{:?}\n", err)
        }
    }
    /* println
    res is
    MySqlQueryResult { rows_affected: 1, last_insert_id: 0 }
    */
}

async fn example_update_username_by_id() {
    match model::user::update_username_by_id("your new name".to_string(), 17).await {
        Ok(res) => {
            println!("\n res is \n{:?}\n", res)
        }
        Err(err) => {
            println!("\nErr is  \n{:?}\n", err)
        }
    }
    /* println
    res is
    MySqlQueryResult { rows_affected: 1, last_insert_id: 0 }
    */
}

async fn example_find_one_by_id() {
    match model::user::find_one_by_id(5).await {
        Ok(res) => {
            println!("\n res is \n{:?}\n", res)
        }
        Err(err) => {
            println!("\nErr is  \n{:?}\n", err)
        }
    }
    /* println
    res is
    Some(User { id: 5, username: "xxx", password: "e10adc3949ba59abbe56e057f20f883e", enable: 1, createTime: 2024-01-30T06:23:31.185782Z, updateTime: 2024-03-13T16:55:48.078618Z })
    */
}

async fn example_fetch_all() {
    match model::user::fetch_all().await {
        Ok(res) => {
            println!("\n res is \n{:?}\n", res)
        }
        Err(err) => {
            println!("\nErr is  \n{:?}\n", err)
        }
    }
    /* println
    res is
    [
        User { id: 5, username: "xxx", password: "e10adc3949ba59abbe56e057f20f883e", enable: 1, createTime: 2024-01-30T06:23:31.185782Z, updateTime: 2024-03-13T16:55:48.078618Z },
        User { id: 7, username: "John6", password: "e10adc3949ba59abbe56e057f20f883e", enable: 1, createTime: 2024-01-30T06:27:11.161173Z, updateTime: 2024-01-30T06:27:11.161182Z },
        User { id: 9, username: "John9", password: "e10adc3949ba59abbe56e057f20f883e", enable: 1, createTime: 2024-01-30T06:27:26.990481Z, updateTime: 2024-03-22T06:47:35.992691Z },
        User { id: 17, username: "rrrr", password: "e10adc3949ba59abbe56e057f20f883e", enable: 1, createTime: 2024-03-22T07:07:46.194102Z, updateTime: 2024-03-22T07:07:46.194103Z }
    ]
    */
}

async fn example_fetch_all_by_dynamic_parameter_req() {
    // test params : query all
    // let req = model::user::PageReq {
    //     pageNo:None,
    //     pageSize:None,
    //     username:None,
    //     enable:None,
    // };
    // test params : query where enable
    let req = model::user::PageReq {
        pageNo: None,
        pageSize: None,
        username: None,
        enable: Some(1),
    };
    // test params : query where enable and like username
    let req = model::user::PageReq {
        pageNo: None,
        pageSize: None,
        username: Some("username".to_string()),
        enable: Some(1),
    };
    match model::user::fetch_all_by_dynamic_parameter_req(req).await {
        Ok(res) => {
            println!("\n res is \n{:?}\n", res)
        }
        Err(err) => {
            println!("\nErr is  \n{:?}\n", err)
        }
    }
    /* println
    res is
    [
        [User { id: 20, username: "your username1", password: "your password", enable: 1, createTime: 2024-03-25T07:24:54.136330Z, updateTime: 2024-03-25T07:24:54.136332Z }]
    ]
    */
}
