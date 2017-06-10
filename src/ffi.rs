use mpz::*;
use std::{mem, slice, str};
use libc::{c_char, strlen};

type AllocFunc = extern "C" fn(usize) -> *mut c_char;
type ReallocFunc = extern "C" fn(*mut c_char, usize, usize) -> *mut c_char;
type FreeFunc = extern "C" fn(*mut c_char, usize);

#[link(name = "gmp")]
extern "C" {
    pub fn __gmpz_fdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    pub fn __gmpz_cdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    pub fn __gmp_get_memory_functions(alloc: *mut AllocFunc, relloc: *mut ReallocFunc, free: *mut FreeFunc);
}

pub struct GString(*mut u8, usize);

impl GString {
    pub unsafe fn from_raw(raw: *mut c_char) -> GString {
        GString(raw as *mut u8, strlen(raw) as usize + 1)
    }

    pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
        let bytes: &[u8] = unsafe { slice::from_raw_parts(self.0, self.1) };
        str::from_utf8(&bytes[..bytes.len() - 1])
    }
}

impl Drop for GString {
    fn drop(&mut self) {
        use std::ptr::null_mut;
        unsafe {
            let mut free_func: FreeFunc = mem::uninitialized();
            __gmp_get_memory_functions(null_mut(), null_mut(), &mut free_func);
            free_func(self.0 as *mut c_char, self.1);
        }
    }
}
