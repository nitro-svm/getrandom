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
#[allow(dead_code)]
pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    extern "Rust" { // Call SP1's version
        fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error>;
    }

    unsafe {
        __getrandom_v03_custom(dest.as_mut_ptr() as *mut u8, dest.len())
    }
}

#[allow(dead_code)]
pub fn inner_u32() -> Result<u32, Error> {
    let mut buf = [MaybeUninit::<u8>::uninit(); 4];
    fill_inner(&mut buf)?;
    // Safe because fill_inner initialized the bytes
    let buf: [u8; 4] = unsafe { core::mem::transmute(buf) };
    Ok(u32::from_ne_bytes(buf))
}

#[allow(dead_code)]
pub fn inner_u64() -> Result<u64, Error> {
    let mut buf = [MaybeUninit::<u8>::uninit(); 8];
    fill_inner(&mut buf)?;
    // Safe because fill_inner initialized the bytes
    let buf: [u8; 8] = unsafe { core::mem::transmute(buf) };
    Ok(u64::from_ne_bytes(buf))
}
