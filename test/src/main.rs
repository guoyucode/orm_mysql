#[macro_use]
extern crate orm_uu;
#[macro_use]
extern crate log;


#[derive(ScyllaDBQuery)]
struct Demo{
    id: i64,
    name: String,
}
impl Demo{
    pub fn table_name() -> String{
        "todo input database tabale name".into()
    }
}

fn main() {
    
}