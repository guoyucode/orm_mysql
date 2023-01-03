use orm_mysql::{OrmMySql};
use orm_mysql::mysql::OrmMySqlTrait;

#[tokio::main]
async fn main() -> common_uu::IResult {
    let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
    let mut conn: mysql_async::Conn = pool.get_conn().await?;
    let v = UserData::default();
    
    // conn.exec("delete from users where user_id=?", &v.user_id).await?;
    let r = v.update(&mut conn).await?;
    println!("find count: {:?}", r);

    Ok(())
}

#[derive(OrmMySql, Default)]
#[orm_mysql(table_name=users)] // is not config: table_name => user_data
struct UserData {
    #[orm_mysql(id)]
    user_id: i64,
    username: String,
}




