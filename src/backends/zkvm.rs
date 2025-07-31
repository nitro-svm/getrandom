// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! zkVM implementation using custom backend
use crate::Error;
use core::mem::MaybeUninit;

// Main function for getrandom 0.3.3 API - note the MaybeUninit
pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    unsafe {
        __getrandom_v03_custom(dest.as_mut_ptr() as *mut u8, dest.len())
    }
}

pub fn inner_u32() -> Result<u32, Error> {
    let mut buf = [MaybeUninit::<u8>::uninit(); 4];
    fill_inner(&mut buf)?;
    // Safe because fill_inner initialized the bytes
    let buf: [u8; 4] = unsafe { core::mem::transmute(buf) };
    Ok(u32::from_ne_bytes(buf))
}

pub fn inner_u64() -> Result<u64, Error> {
    let mut buf = [MaybeUninit::<u8>::uninit(); 8];
    fill_inner(&mut buf)?;
    // Safe because fill_inner initialized the bytes
    let buf: [u8; 8] = unsafe { core::mem::transmute(buf) };
    Ok(u64::from_ne_bytes(buf))
}

// Custom backend function - to be overridden by user implementations
#[no_mangle]
pub unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), Error> {
    // Should be overridden by the correct sp1 implementation. Previous version was:
    // unsafe { sp1_zkvm::syscalls::sys_rand(s.as_mut_ptr(), s.len()) };

    for i in 0..len {
        *dest.add(i) = (i as u8).wrapping_mul(17).wrapping_add(42);
    }
    Ok(())
}
