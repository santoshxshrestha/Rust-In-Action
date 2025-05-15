#![allow(unused)]
use std::fs::write;
use std::{slice, vec};

fn main() {
    //here const is used because that is the only way to create a raw pointer
    let raw_p: *const u32 = &525;
    let another: *mut u32 = &mut 42;

    unsafe {
        another.write(100);
        assert!(*another==100);
        
        assert!(*raw_p == 525);
    }
    {
        let some_vector = vec![1,2,3,4,5,6];
        let pointer = some_vector.as_ptr();
        let length = some_vector.len();

        unsafe {
            let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

            assert_eq!(some_vector.as_slice(), my_slice);
        }
    }
}
