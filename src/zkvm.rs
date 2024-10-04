// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A dummy implementation for unsupported targets which always fails
use crate::Error;

pub fn getrandom_inner(s: &mut [u8]) -> Result<(), Error> {
    unsafe { sp1_zkvm::syscalls::sys_rand(s.as_mut_ptr(), s.len()) };
    Ok(())
}
