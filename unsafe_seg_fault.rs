extern crate libc;

use libc::{size_t, malloc};
use std::mem;
use std::ptr;

fn main() {
    // How ugly it is to pretend Rust is unsafe C.
    unsafe {
        let mut orig: *mut int = malloc(mem::size_of::<int>() as size_t)
            as *mut int;
        ptr::write(&mut *orig, 5i);

        println!("{}", *orig);

        orig = ptr::null::<int>() as *mut int;

        // null pointer crash!
        println!("{}", *orig);
    }
}
