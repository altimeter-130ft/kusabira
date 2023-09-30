//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

//
// Copyright 2023 Seigo Tanimura <seigo.tanimura@gmail.com> and contributors.
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
// Copyright (c) 2023 Seigo Tanimura <seigo.tanimura@gmail.com> and contributors.
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
use std::ffi::{CStr, CString};

///
/// The Rust function called from [`super::imported::hello_world_c_2_fn`] in C
/// by the symbol.
///
/// Call `callback` with a locally created string.
///
/// Print the message with the callback address and its return value.
///
/// Return the length of the printed message.
///
/// # Panics
/// * The creation of the string for `callback` fails.
/// * The length of the printed message cannot be determined.
///
/// # Safety
/// * `msg_cstr` is NUL-terminated.
///
#[no_mangle]
pub extern "C" fn hello_world_rust_2_fn(
	callback: Option<unsafe extern "C" fn(msg: *const i8) -> i32>)
	-> i32
{
	let callback_fn = callback.unwrap();

	let msg = String::from("from Rust");
	let msg_cstr = CString::new(msg).unwrap();

	let ret = unsafe {
		// SAFETY: msg_cstr is NUL-terminated.
		callback_fn(msg_cstr.as_ptr())
	};

	let str = format!(
		"Hello world 2, printed in Rust, callback_fn = {:?}, ret = {ret}.",
		callback_fn);

	println!("{str}");

	str.len().try_into().unwrap()
}

///
/// The Rust callback function passed to
/// [`super::imported::hello_world_c_2_fn`] in C.
///
/// Print the message with the given string embedded.
///
/// Return the length of the printed message.
///
/// # Safety
/// * `msg` is NUL-terminated.
///
/// # Notes
/// A closure cannot be passed to C because it does not support the
/// `extern "C"` qualifier.
///
#[no_mangle]
pub extern "C" fn hello_world_rust_2_callback(msg: *const i8) -> i32
{
	let cstr = unsafe {
		// SAFETY: msg is NUL-terminated.
		CStr::from_ptr(msg)
	};
	let safe_cstr = String::from_utf8_lossy(cstr.to_bytes())
		.to_string();
	let str = format!(
		"Hello world 2 callback, printed in Rust, {safe_cstr}.");

	println!("{str}");

	str.len().try_into().unwrap()
}

#[cfg(test)]
mod tests
{

use super::*;

#[test]
fn test_hello_world_rust_2_fn_success()
{
	let format = "Hello world 2, printed in Rust, callback_fn = , ret = .";

	let callback = hello_world_rust_2_callback;

	let ret = hello_world_rust_2_fn(Some(callback));

	let right_len: i32 = format.len()
			.try_into()
			.expect("usize to i32 conversion MUST succeed");
	println!("ret = {}, right_len = {}.", ret, right_len);

	assert!(ret >
		format.len()
			.try_into()
			.expect("usize to i32 conversion MUST succeed"));
}

#[test]
#[should_panic]
fn test_hello_world_rust_2_fn_fail_null_callback()
{
	hello_world_rust_2_fn(None);
}

#[test]
fn test_hello_world_rust_2_callback()
{
	use std::ffi::CString;

	let msg_str = String::from("called for testing");
	let msg_cstr = CString::new(msg_str.clone().into_bytes())
		.expect("CString creation MUST succeed");
	let format = "Hello world 2 callback, printed in Rust, .";

	let ret = hello_world_rust_2_callback(msg_cstr.into_raw());

	assert_eq!(ret,
		(format.len() + msg_str.len())
			.try_into()
			.expect("usize to i32 conversion MUST succeed"));
}

}
