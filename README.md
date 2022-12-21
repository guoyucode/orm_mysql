# orm_mysql

Committed to building a high-performance, lightweight orm library; Try to achieve zero copy;
>致力于构建高性能、轻量级的orm库; 尽量做到零拷贝;

The function is being developed rapidly. Those who are interested can participate in the development together; 
>功能正在加紧开发中, 有兴趣的可以一起参与开发; 

Join us: WeChat: wx_essence, Telegram: tg_essence;
>加入我们: 微信: wx_essence, Telegram: tg_essence;


examples:

    let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
    let mut conn: mysql_async::Conn = pool.get_conn().await?;
    let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;
    
    // use connection
    let list = User::query(conn, "", None).await?;

    // use transaction
    let list = User::query(&mut tx, "", None).await?;
    
    #[derive(OrmMySql)]
    struct User {
        id: i64,
        name: String,
    }
