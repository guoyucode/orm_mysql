use proc_macro::TokenStream;

mod redis;
mod scylla;
mod mysql;

/* 
async fn query( comm: C, where_sql: &str, limit: Option<usize>) -> common_uu::IResult<Vec<Self>>
async fn query_one( comm: C, where_sql: &str) -> common_uu::IResult<Option<Self>>
*/
/// 
#[proc_macro_derive(OrmMySql, attributes(orm_mysql))]
pub fn mysql_query(input: TokenStream) -> TokenStream{
    mysql::db_query(input)
}


/// 查询方法
/// db_query<T: ToString>(session: &Arc<scylla::Session>, where_sql: String, where_in_vars: &Vec<T>, limit_v: Option<isize>) -> R<Vec<Self>>
#[proc_macro_derive(ScyllaDBQuery)]
pub fn db_query(input: TokenStream) -> TokenStream{
    scylla::db_query(input)
}

#[proc_macro_derive(RedisHget)]
pub fn cache_query_macro(input: TokenStream) -> TokenStream {
    redis::cache_query_macro(input)
}

#[proc_macro_derive(RedisZrange)]
pub fn cache_query_zrange_macro(input: TokenStream) -> TokenStream{
    redis::cache_query_zrange_macro(input)
}

#[test]
fn test() {}
