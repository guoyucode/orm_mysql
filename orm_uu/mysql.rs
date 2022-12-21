use mysql_async::prelude::ToConnection;

#[async_trait::async_trait]
pub trait ORMr {
    async fn query<C>(
        comm: &mut C,
        where_sql: &str,
        limit: Option<usize>,
    ) -> common_uu::IResult<Vec<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    async fn query_one<C>(
        comm: &mut C,
        where_sql: &str,
    ) -> common_uu::IResult<Option<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;
}
