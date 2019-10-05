/*
* Copyright 2019 Comcast Cable Communications Management, LLC
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* SPDX-License-Identifier: Apache-2.0
*/

mod mbuf;
mod mempool;
mod port;

pub use mbuf::*;
pub use mempool::*;
pub use port::*;

use crate::ffi;
use crate::ffi::{AsStr, ToResult};
use failure::{Error, Fail};
use std::ffi::CString;
use std::os::raw;

/// An error generated by libdpdk
#[derive(Debug, Fail)]
#[fail(display = "{}", _0)]
pub struct DpdkError(String);

impl DpdkError {
    #[inline]
    pub(crate) fn new() -> Self {
        let msg = unsafe { ffi::rte_strerror(ffi::_rte_errno()) };
        DpdkError(msg.as_str().into())
    }
}

pub fn eal_init(args: &[&str]) -> Result<u32, Error> {
    unsafe {
        let len = args.len() as raw::c_int;
        let mut args = args
            .iter()
            .map(|&s| CString::from_vec_unchecked(s.into()).into_raw())
            .collect::<Vec<*mut raw::c_char>>();

        ffi::rte_eal_init(len, args.as_mut_ptr()).to_result()
    }
}