extern crate libc;
extern crate blis_sys;

use libc::*;

extern "C" {
    pub fn sgemm_(
        transa: *const c_char,
        transb: *const c_char,
        m: *const c_int,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *const c_float,
        ldb: *const c_int,
        beta: *const c_float,
        c: *mut c_float,
        ldc: *const c_int,
    );
}

#[test]
fn gemm() {
    let a = [1.0];
    let b = [2.0];
    let mut c = [12.0];

    unsafe {
        sgemm_(
            &(b'N' as i8),
            &(b'N' as i8),
            &1,
            &1,
            &1,
            &1.0,
            a.as_ptr(),
            &1,
            b.as_ptr(),
            &1,
            &0.0,
            c.as_mut_ptr(),
            &1,
        );
    }
    assert_eq!(&c, &[2.0]);
}
