#[async_trait::async_trait]
pub trait OrmMySqlTrait {
    async fn query<C>(
        comm: &mut C,
        where_sql: &str,
        limit: Option<usize>,
    ) -> common_uu::IResult<Vec<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    async fn query_first<C>(
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

    async fn update<C>(
        self,
        comm: &mut C,
    ) -> common_uu::IResult<Option<i64>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    async fn delete<C>(
        self,
        comm: &mut C,
    ) -> common_uu::IResult<Option<i64>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

}


pub mod con_value{
    pub trait ValueConv<T> {
        fn conv(&self) -> common_uu::IResult<T>;
    }
    
    impl ValueConv<isize> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<isize>{
            let v = match self.clone(){
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as isize,
                mysql_async::Value::UInt(v) => v as isize,
                mysql_async::Value::Float(v) => v as isize,
                mysql_async::Value::Double(v) => v as isize,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }

    
    impl ValueConv<usize> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<usize>{
            let v = match self.clone(){
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as usize,
                mysql_async::Value::UInt(v) => v as usize,
                mysql_async::Value::Float(v) => v as usize,
                mysql_async::Value::Double(v) => v as usize,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }

    impl ValueConv<u8> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<u8>{
            let v = match self.clone(){
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as u8,
                mysql_async::Value::UInt(v) => v as u8,
                mysql_async::Value::Float(v) => v as u8,
                mysql_async::Value::Double(v) => v as u8,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }
    
    impl ValueConv<i32> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<i32>{
            let v = match self.clone(){
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as i32,
                mysql_async::Value::UInt(v) => v as i32,
                mysql_async::Value::Float(v) => v as i32,
                mysql_async::Value::Double(v) => v as i32,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }
    
    impl ValueConv<i64> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<i64>{
            let v = match self.clone(){
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v,
                mysql_async::Value::UInt(v) => v as i64,
                mysql_async::Value::Float(v) => v as i64,
                mysql_async::Value::Double(v) => v as i64,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }
    
    impl ValueConv<String> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<String>{
            let v = match self.clone(){
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?,
                mysql_async::Value::Int(v) => v.to_string(),
                mysql_async::Value::UInt(v) => v.to_string(),
                mysql_async::Value::Float(v) => v.to_string(),
                mysql_async::Value::Double(v) => v.to_string(),
                other => format!("{:?}", other),
            };
            Ok(v)
        }
    }
    
    impl ValueConv<Option<String>> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<Option<String>>{
            let v = match self.clone() {
                mysql_async::Value::NULL => return Ok(None),
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?,
                mysql_async::Value::Int(v) => v.to_string(),
                mysql_async::Value::UInt(v) => v.to_string(),
                mysql_async::Value::Float(v) => v.to_string(),
                mysql_async::Value::Double(v) => v.to_string(),
                other => format!("{:?}", other),
            };
            Ok(Some(v))
        }
    }
    
}