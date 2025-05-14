use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
        // The printf function from libc
        fn printf(format: *const c_char, ...) -> c_int;
        // The strlen function from libc
        fn strlen(s: *const c_char) -> usize;
    }

fn call_c_printf_example() {
    // Create a C-compatible string
    let message = CString::new("Hello from Rust calling C's printf!\n").unwrap();
    let welcome = CString::new("Welcome 'C' in Rust programming! \n").unwrap();

    // Call C's printf (unsafe because we're calling an external C function)
    unsafe {
        printf(message.as_ptr());
        printf(welcome.as_ptr());
    }
}

fn call_c_strlen_example() {
    let message = CString::new("Testing C's strlen").unwrap();

    // Call C's strlen
    let length = unsafe {
        strlen(message.as_ptr())
    };

    println!("Length calculated by C's strlen: {}", length);
}

fn main() {
    call_c_printf_example();
    call_c_strlen_example();
}

