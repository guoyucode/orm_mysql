use mysql_async::prelude::{Query, ToConnection, WithParams};
use orm_uu::{OrmMySql};
use orm_uu::mysql::ORMr;

#[tokio::main]
async fn main() -> common_uu::IResult {
    let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
    let mut conn: mysql_async::Conn = pool.get_conn().await?;
    let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;
    
    // use connection
    let list = User::query(conn, "", None).await?;

    // use transaction
    let list = User::query(&mut tx, "", None).await?;
    Ok(())
}

#[derive(OrmMySql)]
struct User {
    id: i64,
    name: String,
}




