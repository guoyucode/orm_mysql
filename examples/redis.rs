#[macro_use]
extern crate orm_mysql;
#[macro_use]
extern crate log;


#[derive(RedisHget)]
struct Demo2{
    r#type: i64,
    name: String,
}

#[derive(RedisZrange)]
struct Demo3{
    id: i64,
    name: String,
}


fn main() {
    
}