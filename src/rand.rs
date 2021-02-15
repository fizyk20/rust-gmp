use std::mem::MaybeUninit;

use super::mpz::{mp_bitcnt_t, mpz_ptr, mpz_srcptr, mpz_struct, Mpz};
use libc::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct gmp_randstate_struct {
    _mp_seed: mpz_struct,
    _mp_alg: c_int,
    _mp_algdata: *const c_void,
}

pub type gmp_randstate_t = *mut gmp_randstate_struct;

#[link(name = "gmp")]
extern "C" {
    fn __gmp_randinit_default(state: gmp_randstate_t);
    fn __gmp_randinit_mt(state: gmp_randstate_t);
    fn __gmp_randinit_lc_2exp(
        state: gmp_randstate_t,
        a: mpz_srcptr,
        c: c_ulong,
        m2exp: mp_bitcnt_t,
    );
    fn __gmp_randinit_lc_2exp_size(state: gmp_randstate_t, size: mp_bitcnt_t);
    fn __gmp_randinit_set(state: gmp_randstate_t, op: *const gmp_randstate_struct);
    fn __gmp_randclear(state: gmp_randstate_t);
    fn __gmp_randseed(state: gmp_randstate_t, seed: mpz_srcptr);
    fn __gmp_randseed_ui(state: gmp_randstate_t, seed: c_ulong);
    fn __gmpz_urandomb(rop: mpz_ptr, state: gmp_randstate_t, n: mp_bitcnt_t);
    fn __gmpz_urandomm(rop: mpz_ptr, state: gmp_randstate_t, n: mpz_srcptr);
}

pub struct RandState {
    state: gmp_randstate_struct,
}

unsafe impl Send for RandState {}
unsafe impl Sync for RandState {}

impl Drop for RandState {
    fn drop(&mut self) {
        unsafe { __gmp_randclear(&mut self.state) }
    }
}

impl RandState {
    pub fn new() -> RandState {
        let mut state = MaybeUninit::uninit();
        unsafe {
            __gmp_randinit_default(state.as_mut_ptr());
            let state = state.assume_init();

            RandState { state }
        }
    }

    pub fn new_mt() -> RandState {
        let mut state = MaybeUninit::uninit();
        unsafe {
            __gmp_randinit_mt(state.as_mut_ptr());
            let state = state.assume_init();

            RandState { state }
        }
    }

    pub fn new_lc_2exp(a: Mpz, c: u64, m2exp: u64) -> RandState {
        let mut state = MaybeUninit::uninit();
        unsafe {
            __gmp_randinit_lc_2exp(
                state.as_mut_ptr(),
                a.inner(),
                c as c_ulong,
                m2exp as c_ulong,
            );
            let state = state.assume_init();

            RandState { state }
        }
    }

    pub fn new_lc_2exp_size(size: u64) -> RandState {
        let mut state = MaybeUninit::uninit();
        unsafe {
            __gmp_randinit_lc_2exp_size(state.as_mut_ptr(), size as c_ulong);
            let state = state.assume_init();
            RandState { state }
        }
    }

    pub fn seed(&mut self, seed: Mpz) {
        unsafe { __gmp_randseed(&mut self.state, seed.inner()) }
    }

    pub fn seed_ui(&mut self, seed: u64) {
        unsafe { __gmp_randseed_ui(&mut self.state, seed as c_ulong) }
    }

    /// Generate a uniform random integer in the range 0 to n-1, inclusive
    pub fn urandom(&mut self, n: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_urandomm(res.inner_mut(), &mut self.state, n.inner());
            res
        }
    }

    /// Generate a uniformly distributed random integer in the range 0 to 2^n−1, inclusive.
    pub fn urandom_2exp(&mut self, n: u64) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_urandomb(res.inner_mut(), &mut self.state, n as c_ulong);
            res
        }
    }
}

impl Clone for RandState {
    fn clone(&self) -> RandState {
        let mut state = MaybeUninit::uninit();
        unsafe {
            __gmp_randinit_set(state.as_mut_ptr(), &self.state);
            let state = state.assume_init();

            RandState { state }
        }
    }
}
