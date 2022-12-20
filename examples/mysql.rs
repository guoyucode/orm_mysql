#[tokio::main]
async fn main () -> common_uu::IResult{
    let pool: mysql_async::Pool = mysql_async::Pool::new("mysql://用户名:密码@ip:端口号/数据库名");
    let conn: mysql_async::Conn = pool.get_conn().await?;
    let tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;
    Ok(())
}