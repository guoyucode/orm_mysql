use orm_mysql::mysql::OrmMySqlTrait;
use orm_mysql::OrmMySql;

#[tokio::main]
async fn main() -> common_uu::IResult {
    let ref pool: mysql_async::Pool = mysql_async::Pool::new("mysql://username:pwd@ip:port/db_name");
    let mut conn: mysql_async::Conn = pool.get_conn().await?;
    let mut tx = pool.start_transaction(mysql_async::TxOpts::new()).await?;

    // use connection
    let user = UserData{user_id: 5, username:"123".to_string()};
    user.insert(&mut conn).await?;

    // use transaction
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

/* gen code:
#[async_trait::async_trait]
impl OrmMySqlTrait for UserData {
    async fn query_list<C>(
        comm: &mut C,
        where_sql: &str,
        limit: Option<usize>,
    ) -> common_uu::IResult<Vec<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync,
    {
        todo!()
    }

    async fn query<C>(comm: &mut C, where_sql: &str) -> common_uu::IResult<Option<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync,
    {
        todo!()
    }

    async fn insert<C>(self, conn: &mut C) -> common_uu::IResult<Option<i64>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync
    {
        let mut params = vec![self].into_iter().map(|Self{user_id, username}| (user_id, username)).collect::<Vec<_>>();
        let r: Option<(i64, )> = conn.exec_first("insert into users (user_id, username)values(?, ?)", params.remove(0)).await?;
        let r = r.map(|v|v.0);
        println!("r: {:?}", r);
        Ok(r)
    }
}
 */