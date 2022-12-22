#[macro_use]
extern crate orm_mysql;
#[macro_use]
extern crate log;


#[derive(ScyllaDBQuery)]
struct Demo1{
    id: i64,
    name: String,
}
impl Demo1{
    pub fn table_name() -> String{
        "todo input database tabale name".into()
    }
}


fn main() {
    
}