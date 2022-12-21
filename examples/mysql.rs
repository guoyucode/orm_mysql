use mysql_async::prelude::{Query, ToConnection, WithParams};
use orm_uu::{OrmMySql};
use orm_uu::mysql::ORMr;
// use mysql_async::prelude::Queryable;

#[tokio::main]
async fn main() -> common_uu::IResult {
    let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
    let mut conn: mysql_async::Conn = pool.get_conn().await?;
    let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;
    
    // use connection
    let list = UserData::query(&mut conn, "where 1 != 1", None).await?;
    // sql: select user_id,username from users where 1 != 1
    println!("find count: {}", list.len());

    // use transaction
    let list = UserData::query(&mut tx, "where 1=1", Some(1000)).await?;
    // sql: select user_id,username from users where 1=1 limit 1000
    println!("find count: {}", list.len());
    Ok(())
}

#[derive(OrmMySql)]
#[orm_mysql(table_name=users)] // is not config: table_name => user_data
struct UserData {
    user_id: i64,
    username: String,
}




