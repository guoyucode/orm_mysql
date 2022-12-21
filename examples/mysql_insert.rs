use mysql_async::prelude::{Query, ToConnection, WithParams};
use orm_uu::{OrmMySql};
use orm_uu::mysql::ORMr;
use mysql_async::prelude::Queryable;

#[tokio::main]
async fn main() -> common_uu::IResult {
    let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
    let mut conn: mysql_async::Conn = pool.get_conn().await?;
    let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;
    
    // use connection
    let sql = "insert into users (user_id, username)values(:user_id, :username)";
    sql.with(vec![].iter().map(|UserData{ user_id, username }| params! {
        "user_id" => user_id,
        "username" => username,
    })).batch(&mut conn);

    Ok(())
}

#[derive(OrmMySql)]
#[orm_mysql(table_name=users)] // is not config: table_name => user_data
struct UserData {
    user_id: i64,
    username: String,
}




