# orm_mysql

Committed to building a high-performance, lightweight orm library; Try to achieve zero copy;
>致力于构建高性能、轻量级的orm库; 尽量做到零拷贝;

The function is being developed rapidly. Those who are interested can participate in the development together; 
>功能正在加紧开发中, 有兴趣的可以一起参与开发; 

Join us: WeChat: wx_essence, Telegram: tg_essence;
>加入我们: 微信: wx_essence, Telegram: tg_essence;


examples:

    fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
        let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
        let mut conn: mysql_async::Conn = pool.get_conn().await?;
        let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;

        // use connection
        let list = UserData::query_list(&mut conn, "where 1 != 1", None).await?;
        // sql: select user_id,username from users where 1 != 1
        println!("find count: {}", list.len());

        // use transaction
        let list = UserData::query_list(&mut tx, "where 1=1", Some(1000)).await?;
        // sql: select user_id,username from users where 1=1 limit 1000
        println!("find count: {}", list.len());

        // query_one
        let one = UserData::query(&mut tx, "where 1=1").await?;
        // sql: select user_id,username from users where 1=1 limit 1
        println!("find count: {:?}", one.is_some());

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