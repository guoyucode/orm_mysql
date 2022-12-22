use std::{fmt::Display};

use common_uu::JsonVExentd;


pub trait ConvData<T> {
    fn conv_data(self) -> T;
}
impl ConvData<i64> for &Option<String>  {
    fn conv_data(self) -> i64 {
        match self{
            Some(v) => v.parse::<i64>().unwrap_or_default(),
            None => Default::default(),
        }
    }
}

impl ConvData<Option<i64>> for &Option<String>  {
    fn conv_data(self) -> Option<i64> {
        let this = match self{
            None => return None,
            Some(v) => v,
        };
        match this.parse::<i64>(){
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
impl ConvData<i32> for &Option<String>  {
    fn conv_data(self) -> i32 {
        let this = match self{
            None => return Default::default(),
            Some(v) => v,
        };
        this.parse::<i32>().unwrap_or_default()
    }
}
impl ConvData<Option<i32>> for &Option<String> {
    fn conv_data(self) -> Option<i32> {
        let this = match self{
            None => return Default::default(),
            Some(v) => v,
        };
        match this.parse::<i32>(){
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
impl ConvData<Option<isize>> for &Option<String> {
    fn conv_data(self) -> Option<isize> {
        let this = match self{
            None => return Default::default(),
            Some(v) => v,
        };
        match this.parse::<isize>(){
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
impl ConvData<isize> for &Option<String> {
    fn conv_data(self) -> isize {
        let this = match self{
            None => return Default::default(),
            Some(v) => v,
        };
        match this.parse::<isize>(){
            Ok(v) => v,
            Err(_) => Default::default(),
        }
    }
}







impl ConvData<String> for &Option<String>  {
    fn conv_data(self) -> String {
        let this = match self{
            None => return "".into(),
            Some(v) => v.to_string(),
        };
        this
    }
}

impl ConvData<Option<String>> for &Option<String>  {
    fn conv_data(self) -> Option<String> {
        self.clone()
    }
}
impl ConvData<Option<bool>> for &Option<String>  {
    fn conv_data(self) -> Option<bool> {
        let this = match self{
            None => return Default::default(),
            Some(v) => v,
        };
        match this.parse::<bool>(){
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

impl ConvData<usize> for &str  {
    fn conv_data(self) -> usize {
        self.parse::<usize>().unwrap_or_default()
    }
}

pub struct VecInto<T: ToString>(pub Vec<T>);
impl<'a, T: Display + Clone> From<&Vec<T>> for VecInto<T>{
    fn from(v: &Vec<T>) -> Self {
        VecInto(v.clone())
    }
}
impl<'a, T: Display> From<Vec<T>> for VecInto<T>{
    fn from(v: Vec<T>) -> Self {
        VecInto(v)
    }
}
impl<T: ToString + Clone> From<&[T]> for VecInto<T>{
    fn from(v: &[T]) -> Self {
        let v = v.to_vec();
        VecInto(v)
    }
}
static EMPTY_VEC: Vec<String> = vec![];
impl<'a> From<()> for VecInto<String> {
    fn from(_v: ()) -> Self {
        Self(EMPTY_VEC.clone())
    }
}

pub fn is_empty<T: serde::Serialize>(s: &T) -> bool {
    let obj = json!(s).as_object2().unwrap_or_default();
    for (_, v) in obj {
        if !v.is_null() {
            return false;
        }
    }
    true
}