use libc;

fn main(){
    // should not crash
    unsafe {
        assert_eq!(libc::KERNEL_VERSION(6, 0, 0), 393216);
    }
}