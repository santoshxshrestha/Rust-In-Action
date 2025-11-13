use std::{any::type_name_of_val, process::Output};

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    return a + b;
}

pub trait MeroTrait {
    fn use_this(&self) -> String;
}

impl MeroTrait for i32 {
    fn use_this(&self) -> String {
        return format!("i32: {}", self);
    }
}
impl MeroTrait for String {
    fn use_this(&self) -> String {
        return format!("String: {}", self);
    }
}
impl MeroTrait for i64 {
    fn use_this(&self) -> String {
        return format!("i64: {}", self);
    }
}

fn do_some_thing<T>(content: T) -> String
where
    T: MeroTrait<Output = String>,
{
    return type_name_of_val(&content).to_string();
}
fn main() {}
