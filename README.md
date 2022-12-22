# orm_mysql

Committed to building a high-performance, lightweight orm library; Try to achieve zero copy;
>致力于构建高性能、轻量级的orm库; 尽量做到零拷贝;

The function is being developed rapidly. Those who are interested can participate in the development together; 
>功能正在加紧开发中, 有兴趣的可以一起参与开发; 

Join us: WeChat: wx_essence, Telegram: tg_essence;
>加入我们: 微信: wx_essence, Telegram: tg_essence;

[examples mysql_all](https://github.com/guoyucode/orm_mysql/blob/main/examples/mysql_all.rs)
[examples mysql_insert](https://github.com/guoyucode/orm_mysql/blob/main/examples/mysql_insert.rs)
[examples mysql_query](https://github.com/guoyucode/orm_mysql/blob/main/examples/mysql_query.rs)

examples:

    add Cargo.toml dependencies
    mysql_async = "0.31"
    orm_mysql = "*"

    fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
        let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
        let mut conn: mysql_async::Conn = pool.get_conn().await?;
        let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;

        // use connection
        let query_first = UserData::query_first(&mut conn, "where 1 != 1", None).await?;
        // sql: select user_id,username from users where 1 != 1 limit 1
        println!("find: {:?}", query_first);

        // use transaction
        let query_first = UserData::query_first(&mut tx, "where 1=1", Some(1000)).await?;
        // sql: select user_id,username from users where 1=1 limit 1
        println!("find: {:?}", query_first);

        // query_list
        let list = UserData::query(&mut tx, "where 1=1").await?;
        // sql: select user_id,username from users where 1=1 limit 1
        println!("find count: {:?}", list);

        // insert into data
        let user = UserData{user_id: 5, username:"123".to_string()};
        user.insert(&mut conn).await?;

        // use transaction insert into data
        let user = UserData{user_id: 5, username:"123".to_string()};
        user.insert(&mut tx).await?;
        tx.commit().await?;

        Ok(())
    }

    #[derive(OrmMySql)]
    #[orm_mysql(table_name=users)] // is not config: table_name => user_data
    struct UserData {
        user_id: i64,
        username: String,
    }