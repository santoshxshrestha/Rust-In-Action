#![allow(unused)]
use tokio::join;
use tokio::time::Duration;
use tokio::time::sleep;

pub fn add(first: i32, second: i32) -> i32 {
    return first + second;
}
