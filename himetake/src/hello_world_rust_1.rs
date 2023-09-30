//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

//
// Copyright 2023 Seigo Tanimura <seigo.tanimura@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//
// MIT License
//
// Copyright (c) 2023 Seigo Tanimura <seigo.tanimura@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#![deny(missing_docs)]

use std::convert::TryInto;
use std::ffi::CStr;

///
/// The Rust function called from [`super::imported::hello_world_c_1_fn`] in C
/// by the symbol.
///
/// Print a message with the given string embedded.
///
/// Return the length of the printed message.
///
/// # Panics
/// * The length of the printed message cannot be determined.
///
/// # Safety
/// * `msg` is NUL-terminated.
///
#[no_mangle]
pub extern "C" fn hello_world_rust_1_fn(msg: *const i8) -> i32
{
	let cstr = unsafe {
		// SAFETY: msg is NUL-terminated.
		CStr::from_ptr(msg)
	};
	let safe_cstr = String::from_utf8_lossy(cstr.to_bytes()).to_string();
	let str = format!("Hello world 1, printed in Rust, {safe_cstr}.");

	println!("{str}");

	str.len().try_into().unwrap()
}

#[cfg(test)]
mod tests
{

use super::*;

#[test]
fn test_hello_world_rust_1_fn()
{
	use std::ffi::CString;

	let msg_str = String::from("called for testing");
	let msg_cstr = CString::new(msg_str.clone().into_bytes())
		.expect("CString creation MUST succeed");
	let format = "Hello world 1, printed in Rust, .";

	let ret = hello_world_rust_1_fn(msg_cstr.into_raw());

	assert_eq!(ret,
		(format.len() + msg_str.len())
			.try_into()
			.expect("usize to i32 conversion MUST succeed"));
}

}
