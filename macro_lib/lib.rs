use proc_macro::TokenStream;

mod mysql;
mod utils;

/* 
async fn query( comm: C, where_sql: &str, limit: Option<usize>) -> common_uu::IResult<Vec<Self>>
async fn query_one( comm: C, where_sql: &str) -> common_uu::IResult<Option<Self>>
*/
/// 
#[proc_macro_derive(OrmMySql, attributes(orm_mysql))]
pub fn mysql_query(input: TokenStream) -> TokenStream{
    mysql::db_query(input)
}


#[test]
fn test() {}
