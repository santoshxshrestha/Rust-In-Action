#![allow(unused)]
use std::any::{type_name, type_name_of_val};
use std::path::Path;
use trash::os_limited::purge_all;
use trash::os_limited::{self, restore_all};

fn main() {
    let content_to_delete = trash::os_limited::list()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .name;
    let truthy = content_to_delete == std::ffi::OsStr::new("test.txt");
    println!("the content is {:#?}", content_to_delete);
    println!("the truthy is {}", truthy);
}
