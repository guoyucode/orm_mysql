use mysql_async::prelude::{Query, ToConnection, WithParams};

#[tokio::main]
async fn main() -> common_uu::IResult {
    let ref pool: mysql_async::Pool =
        mysql_async::Pool::new("mysql://用户名:密码@ip:端口号/数据库名");
    let mut conn: mysql_async::Conn = pool.get_conn().await?;
    let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;
    let list = "select id, name from user "
        .with(())
        .map(&mut tx, |(id, name)| User { id, name })
        .await?;
    Ok(())
}

struct User {
    id: i64,
    name: String,
}


#[async_trait::async_trait]
impl orm_uu::mysql::ORMr for User {
    async fn query<'a, 't: 'a, C>(
        comm: C,
        where_sql: &'a str,
        limit: Option<usize>,
    ) -> common_uu::IResult<Vec<Self>>
    where
        Self: Sized,
        C: ToConnection<'a, 't> + 'a,
    {
        let r = where_sql
            .with(())
            .map(comm, |(id, name)| Self { id, name })
            .await?;
        Ok(r)
    }

    async fn query_one<'a, 't: 'a, C>(
        comm: C,
        where_sql: &'a str,
    ) -> common_uu::IResult<Option<Self>>
    where
        Self: Sized,
        C: ToConnection<'a, 't> + 'a,
    {
        let mut r = Self::query(comm, where_sql, Some(1)).await?;
        match r.len(){
            0 => return Ok(None),
            1 => return Ok(Some(r.remove(0))),
            _ => return Err(format!("'{where_sql}' find more row data!"))?,
        }
    }
}


