use proc_macro::TokenStream;

mod mysql;
mod utils;


/// plase see ./examples/xxxx.rs
#[proc_macro_derive(OrmMySql, attributes(orm_mysql))]
pub fn mysql_query(input: TokenStream) -> TokenStream{
    mysql::db_query(input)
}


#[test]
fn test() {}
