use mysql_async::prelude::ToConnection;

#[async_trait::async_trait]
pub trait OrmMySqlTrait {
    async fn query_list<C>(
        comm: &mut C,
        where_sql: &str,
        limit: Option<usize>,
    ) -> common_uu::IResult<Vec<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    async fn query<C>(
        comm: &mut C,
        where_sql: &str,
    ) -> common_uu::IResult<Option<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    async fn insert<C>(
        self,
        comm: &mut C,
    ) -> common_uu::IResult<Option<i64>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;
}
