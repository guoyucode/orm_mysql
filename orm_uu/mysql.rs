use mysql_async::prelude::ToConnection;

#[async_trait::async_trait]
pub trait ORMr {
    async fn query<'a, 't: 'a, C>(
        comm: C,
        where_sql: &'a str,
        limit: Option<usize>,
    ) -> common_uu::IResult<Vec<Self>>
    where
        Self: Sized,
        C: ToConnection<'a, 't> + 'a;

    async fn query_one<'a, 't: 'a, C>(
        comm: C,
        where_sql: &'a str,
    ) -> common_uu::IResult<Option<Self>>
    where
        Self: Sized,
        C: ToConnection<'a, 't> + 'a;
}
