use libc::c_int;

extern "C" {
    fn stackblur(buffer: *mut u8, x: c_int, y: c_int, w: c_int, h: c_int, r: c_int, n: c_int);
}

pub fn blur(buffer: &mut [u8], w: c_int, h: c_int, r: c_int) {
    assert!(r > 0);
    unsafe {
        stackblur(buffer.as_mut_ptr(), 0, 0, w, h, r, num_cpus::get() as i32);
    }
}
