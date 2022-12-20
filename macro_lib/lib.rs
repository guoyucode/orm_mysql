use proc_macro::TokenStream;

mod redis;
mod scylla;

/// 生成方法
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
