use common_uu::JsonV;
use mysql_async::{prelude::Queryable, Params};

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

    async fn query_first<C>(comm: &mut C, where_sql: &str) -> common_uu::IResult<Option<Self>>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    async fn insert<C>(self, comm: &mut C) -> common_uu::IResult<i64>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    fn insert_sql(&self) -> String
    where
        Self: Sized;

    async fn update<C>(self, comm: &mut C) -> common_uu::IResult<i64>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    async fn delete<C>(&self, comm: &mut C) -> common_uu::IResult<i64>
    where
        Self: Sized,
        C: mysql_async::prelude::Queryable + Send + Sync;

    fn delete_sql(&self) -> String
    where
        Self: Sized;

    fn where_id(&self) -> common_uu::JsonV
    where
        Self: Sized;
}

#[async_trait::async_trait]
pub trait OrmMySqlTraitConn<T: OrmMySqlTrait, C: mysql_async::prelude::Queryable + Send + Sync> {
    async fn query(&mut self, where_sql: &str, limit: Option<usize>) -> common_uu::IResult<Vec<T>>;
    async fn query_first(&mut self, where_sql: &str) -> common_uu::IResult<Option<T>>;
    async fn insert(&mut self, v: &T) -> common_uu::IResult<i64>;
    async fn insert_arr(&mut self, arr: &Vec<T>) -> common_uu::IResult<i64>;
    async fn update(&mut self, v: &T) -> common_uu::IResult<i64>;
    async fn delete(&mut self, v: &T) -> common_uu::IResult<i64>;
    async fn delete_arr(&mut self, arr: &Vec<T>) -> common_uu::IResult<i64>;
}

#[async_trait::async_trait]
impl<T: OrmMySqlTrait + Send + Sync + Clone + Into<Params>> OrmMySqlTraitConn<T, Self>
    for mysql_async::Conn
{
    async fn query(&mut self, where_sql: &str, limit: Option<usize>) -> common_uu::IResult<Vec<T>> {
        T::query(self, where_sql, limit).await
    }
    async fn query_first(&mut self, where_sql: &str) -> common_uu::IResult<Option<T>> {
        T::query_first(self, where_sql).await
    }
    async fn insert(&mut self, v: &T) -> common_uu::IResult<i64> {
        v.clone().insert(self).await
    }
    async fn insert_arr(&mut self, arr: &Vec<T>) -> common_uu::IResult<i64> {
        if arr.is_empty() {
            return Ok(0);
        }
        let sql = arr[0].insert_sql();
        self.exec_batch(sql, arr.clone()).await?;
        Ok(arr.len() as i64)
    }
    async fn update(&mut self, v: &T) -> common_uu::IResult<i64> {
        v.clone().update(self).await
    }
    async fn delete(&mut self, v: &T) -> common_uu::IResult<i64> {
        v.delete(self).await
    }
    async fn delete_arr(&mut self, arr: &Vec<T>) -> common_uu::IResult<i64> {
        if arr.is_empty() {
            return Ok(0);
        }
        let sql = arr[0].delete_sql();
        let params = arr.iter().map(|x| x.where_id().to_string()).collect::<Vec<_>>();
        // self.exec_batch(sql, params).await?;
        unimplemented!("delete_arr");
        Ok(arr.len() as i64)
    }
}

#[async_trait::async_trait]
impl<T: OrmMySqlTrait + Send + Sync + Clone + Into<Params>, 'a> OrmMySqlTraitConn<T, Self>
    for mysql_async::Transaction<'a>
{
    async fn query(&mut self, where_sql: &str, limit: Option<usize>) -> common_uu::IResult<Vec<T>> {
        T::query(self, where_sql, limit).await
    }
    async fn query_first(&mut self, where_sql: &str) -> common_uu::IResult<Option<T>> {
        T::query_first(self, where_sql).await
    }
    async fn insert(&mut self, v: &T) -> common_uu::IResult<i64> {
        v.clone().insert(self).await
    }
    async fn insert_arr(&mut self, arr: &Vec<T>) -> common_uu::IResult<i64> {
        if arr.is_empty() {
            return Ok(0);
        }
        let sql = arr[0].insert_sql();
        println!("insert_arr({}): {sql}", arr.len());
        self.exec_batch(sql, arr.clone()).await?;
        Ok(arr.len() as i64)
    }
    async fn update(&mut self, v: &T) -> common_uu::IResult<i64> {
        v.clone().update(self).await
    }
    async fn delete(&mut self, v: &T) -> common_uu::IResult<i64> {
        v.delete(self).await
    }
    async fn delete_arr(&mut self, arr: &Vec<T>) -> common_uu::IResult<i64> {
        if arr.is_empty() {
            return Ok(0);
        }
        let sql = arr[0].delete_sql();
        let params = arr.iter().map(|x| x.where_id()).collect::<Vec<_>>();
        // self.exec_batch(sql, params).await?;
        unimplemented!("delete_arr");
        Ok(arr.len() as i64)
    }
}

pub mod con_value {
    pub trait ValueConv<T> {
        fn conv(&self) -> common_uu::IResult<T>;
    }

    #[cfg(feature = "chrono")]
    impl ValueConv<chrono::NaiveTime> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<chrono::NaiveTime> {
            let v = match self.clone() {
                mysql_async::Value::Bytes(bytes) => {
                    let s = String::from_utf8(bytes)?;
                    let v = chrono::NaiveTime::parse_from_str(&s, "%H:%M:%S")?;
                    v
                }
                mysql_async::Value::Time(_is_negative, days, hours, minutes, seconds, micro) => {
                    let mut v = chrono::NaiveTime::from_hms(0, 0, 0);
                    v = v + chrono::Duration::days(days as i64);
                    v = v + chrono::Duration::hours(hours as i64);
                    v = v + chrono::Duration::minutes(minutes as i64);
                    v = v + chrono::Duration::seconds(seconds as i64);
                    v = v + chrono::Duration::microseconds(micro as i64);
                    // if is_negative{
                    //     v = v.checked_neg().unwrap();
                    // }
                    v
                }
                _ => {
                    return Err("mysql_async::Value::Time")?;
                }
            };
            Ok(v)
        }
    }
    #[cfg(feature = "chrono")]
    impl ValueConv<Option<chrono::NaiveTime>> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<Option<chrono::NaiveTime>> {
            let v = match self.clone() {
                mysql_async::Value::NULL => return Ok(None),
                other => other.conv(),
            }?;
            Ok(Some(v))
        }
    }

    #[cfg(feature = "chrono")]
    impl ValueConv<chrono::NaiveDateTime> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<chrono::NaiveDateTime> {
            let v = match self.clone() {
                mysql_async::Value::Bytes(bytes) => {
                    let s = String::from_utf8(bytes)?;
                    chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                }
                mysql_async::Value::Date(year, month, day, hour, minutes, seconds, micro) => {
                    chrono::NaiveDateTime::parse_from_str(
                        &format!("{year}-{month}-{day} {hour}:{minutes}:{seconds}.{micro:0>6}"),
                        "%Y-%m-%d %H:%M:%S.f6",
                    )
                }
                _ => {
                    return Err("mysql_async::Value::Date")?;
                }
            }?;
            Ok(v)
        }
    }

    #[cfg(feature = "chrono")]
    impl ValueConv<Option<chrono::NaiveDateTime>> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<Option<chrono::NaiveDateTime>> {
            let v = match self.clone() {
                mysql_async::Value::NULL => return Ok(None),
                other => other.conv(),
            }?;
            Ok(Some(v))
        }
    }

    #[cfg(feature = "chrono")]
    impl ValueConv<chrono::NaiveDate> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<chrono::NaiveDate> {
            let v = match self.clone() {
                mysql_async::Value::Bytes(bytes) => {
                    let s = String::from_utf8(bytes)?;
                    chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                }
                mysql_async::Value::Date(year, month, day, hour, minutes, seconds, micro) => {
                    chrono::NaiveDate::parse_from_str(&format!("{year}-{month}-{day}"), "%Y-%m-%d")
                }
                mysql_async::Value::Time(_is_negative, days, hours, minutes, seconds, micro) => {
                    let mut v = chrono::NaiveDate::from_ymd(1970, 1, 1);
                    v = v + chrono::Duration::days(days as i64);
                    Ok(v)
                }
                _ => {
                    return Err("mysql_async deser tos chrono::NaiveDate")?;
                }
            }?;
            Ok(v)
        }
    }
    #[cfg(feature = "chrono")]
    impl ValueConv<Option<chrono::NaiveDate>> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<Option<chrono::NaiveDate>> {
            let v = match self.clone() {
                mysql_async::Value::NULL => return Ok(None),
                other => other.conv(),
            }?;
            Ok(Some(v))
        }
    }

    #[cfg(feature = "chrono")]
    impl ValueConv<chrono::DateTime<chrono::Local>> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<chrono::DateTime<chrono::Local>> {
            let v = match self.clone() {
                mysql_async::Value::Int(timestament) => {
                    use chrono::TimeZone;
                    let dt = chrono::Local.timestamp(timestament as i64, 0);
                    dt
                }
                mysql_async::Value::UInt(timestament) => {
                    use chrono::TimeZone;
                    let dt = chrono::Local.timestamp(timestament as i64, 0);
                    dt
                }
                mysql_async::Value::Time(_is_negative, days, hours, minutes, seconds, micro) => {
                    let mut v = chrono::NaiveTime::from_hms(0, 0, 0);
                    v = v + chrono::Duration::days(days as i64);
                    v = v + chrono::Duration::hours(hours as i64);
                    v = v + chrono::Duration::minutes(minutes as i64);
                    v = v + chrono::Duration::seconds(seconds as i64);
                    v = v + chrono::Duration::microseconds(micro as i64);
                    // if is_negative{
                    //     v = v.checked_neg().unwrap();
                    // }
                    chrono::Local::now().date().and_time(v).unwrap_or_default()
                }
                mysql_async::Value::Date(year, month, day, hour, minutes, seconds, micro) => {
                    use chrono::TimeZone;
                    let dt = chrono::Local
                        .ymd(year as i32, month as u32, day as u32)
                        .and_hms_micro(hour as u32, minutes as u32, seconds as u32, micro as u32);
                    dt
                }
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }

    #[cfg(feature = "chrono")]
    impl ValueConv<Option<chrono::DateTime<chrono::Local>>> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<Option<chrono::DateTime<chrono::Local>>> {
            let v = match self.clone() {
                mysql_async::Value::NULL => {
                    return Ok(None);
                }
                other => other.conv()?,
            };
            Ok(v)
        }
    }

    #[cfg(feature = "chrono")]
    impl ValueConv<Option<chrono::DateTime<chrono::Utc>>> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<Option<chrono::DateTime<chrono::Utc>>> {
            let v = match self.clone() {
                mysql_async::Value::NULL => {
                    return Ok(None);
                }
                other => other.conv()?,
            };
            Ok(v)
        }
    }

    impl ValueConv<f64> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<f64> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0.0,
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as f64,
                mysql_async::Value::UInt(v) => v as f64,
                mysql_async::Value::Float(v) => v as f64,
                mysql_async::Value::Double(v) => v,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }

    impl ValueConv<f32> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<f32> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0.0,
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as f32,
                mysql_async::Value::UInt(v) => v as f32,
                mysql_async::Value::Float(v) => v,
                mysql_async::Value::Double(v) => v as f32,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }

    impl ValueConv<isize> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<isize> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0,
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

    impl ValueConv<u64> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<u64> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0,
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as u64,
                mysql_async::Value::UInt(v) => v as u64,
                mysql_async::Value::Float(v) => v as u64,
                mysql_async::Value::Double(v) => v as u64,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }

    impl ValueConv<u32> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<u32> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0,
                mysql_async::Value::Bytes(v) => String::from_utf8(v)?.parse()?,
                mysql_async::Value::Int(v) => v as u32,
                mysql_async::Value::UInt(v) => v as u32,
                mysql_async::Value::Float(v) => v as u32,
                mysql_async::Value::Double(v) => v as u32,
                other => Err(format!("ValueConv Error: {:?}", other))?,
            };
            Ok(v)
        }
    }

    impl ValueConv<usize> for mysql_async::Value {
        fn conv(&self) -> common_uu::IResult<usize> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0,
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
        fn conv(&self) -> common_uu::IResult<u8> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0,
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
        fn conv(&self) -> common_uu::IResult<i32> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0,
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
        fn conv(&self) -> common_uu::IResult<i64> {
            let v = match self.clone() {
                mysql_async::Value::NULL => 0,
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
        fn conv(&self) -> common_uu::IResult<String> {
            let v = match self.clone() {
                mysql_async::Value::NULL => "".into(),
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
        fn conv(&self) -> common_uu::IResult<Option<String>> {
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
