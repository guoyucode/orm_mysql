# orm_mysql

Committed to building a high-performance, lightweight orm library; Try to achieve zero copy;
>致力于构建高性能、轻量级的orm库; 尽量做到零拷贝;

The function is being developed rapidly. Those who are interested can participate in the development together; 
>功能正在加紧开发中, 有兴趣的可以一起参与开发; 

Join us: WeChat: wx_essence, Telegram: tg_essence;
>加入我们: 微信: wx_essence, Telegram: tg_essence;

[examples mysql_all](https://github.com/guoyucode/orm_mysql/blob/main/examples/mysql_all.rs)<br>
[examples mysql_insert](https://github.com/guoyucode/orm_mysql/blob/main/examples/mysql_insert.rs)<br>
[examples mysql_query](https://github.com/guoyucode/orm_mysql/blob/main/examples/mysql_query.rs)<br>

examples:

    add Cargo.toml dependencies
    mysql_async = "0.31"
    orm_mysql = "*"

    
    use orm_mysql::mysql_async::prelude::*;
    use orm_mysql::mysql::OrmMySqlTrait;
    use orm_mysql::OrmMySql;

    #[tokio::main]
    async fn main() -> common_uu::IResult {
        let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
        let mut conn: mysql_async::Conn = pool.get_conn().await?;

        r"DROP TABLE if exists users_temp".ignore(&mut conn).await?;

        r"CREATE TABLE users_temp (
            user_id int   not null,
            username      varchar(128) null,
            username2      varchar(128) null,
            username3      varchar(128) null,
            username4      varchar(128) null,
            username5      varchar(128) null,
            username6      varchar(128) null,
            username7      varchar(128) null,
            username8      varchar(128) null,
            username9      varchar(128) null,
            username10      varchar(128) null,
            username11      varchar(128) null,
            username12      varchar(128) null,
            username13      varchar(128) null,
            username14      varchar(128) null,
            username15      varchar(128) null,
            float_v      float(8,2) null
        )"
        .ignore(&mut conn)
        .await?;

        // use conn insert 
        // let r: Option<i64>=conn.exec_first("insert into users_temp (user_id, username, username2)values(?,?,?)", (1, "11".to_string(), Some("111".to_string()))).await?;
        let mut user = UserData::default();
        user.user_id = 1;
        user.username = "11".to_string();
        user.username2 = Some("111".to_string());
        user.insert(&mut conn).await?;

        // use transaction
        let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;
        let mut user = UserData::default();
        user.user_id = 2;
        user.username = "22".to_string();
        user.username2 = Some("222".to_string());
        user.insert(&mut tx).await?;
        tx.commit().await?;

        // let r: Option<UserData> = conn.query_first("select * from users_temp").await?;
        let r: Option<UserData> = UserData::query_first(&mut conn, "where 1=1").await?;
        println!(r##"UserData::query_first {:?}"##, r);

        // let r: Vec<UserData> = conn.query("select * from users_temp").await?;
        let r: Vec<UserData> = UserData::query(&mut conn, "where 1=1", Some(99)).await?;
        println!("UserData::query: {:?}", r);

        Ok(())
    }

    // #[derive(Default, Debug)]
    #[derive(OrmMySql, Default, Debug)]
    #[orm_mysql(table_name=users_temp)] // is not config: table_name => user_data
    struct UserData {
        #[orm_mysql(id)]
        user_id: i64,
        username: String,
        username2: Option<String>,
        username3: Option<String>,
        username4: Option<String>,
        username5: Option<String>,
        username6: Option<String>,
        username7: Option<String>,
        username8: Option<String>,
        username9: Option<String>,
        username10: Option<String>,
        username11: Option<String>,
        username12: Option<String>,
        username13: String,
        username14: String,
        username15: String,
        float_v: f64,
        
        datetime_v: chrono::NaiveDateTime,
        datetime_opt: Option<chrono::NaiveDateTime>,

        date: chrono::NaiveDate,
        date_opt: Option<chrono::NaiveDate>,

        // not support; 暂时不支持
        // datetime_utc_v: Option<chrono::DateTime<chrono::Utc>>,
        // datetime_local_v: chrono::DateTime<chrono::Local>,
        // naive_date_v: time::PrimitiveDateTime,
        // naive_date_opt_v: Option<time::Date>,

    }
