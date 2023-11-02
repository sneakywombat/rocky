use librocksdb_sys;
use std::os::raw::c_void;
use std::ptr;

pub fn main() {
    print!("hello ");
    let p: *mut c_void = ptr::null_mut();
    unsafe {librocksdb_sys::rocksdb_free(p);}
    print!("world\n");
}
