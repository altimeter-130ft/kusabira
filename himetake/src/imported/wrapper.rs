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

use std::error::Error as ErrorTrait;
use std::fmt::{Display, Error as FmtError, Formatter};

use super::cw_return_values;

///
/// The unit structure representing an error out of the
/// [`unixcw`](https://sourceforge.net/projects/unixcw/) APIs.
///
#[derive(Debug)]
pub struct LibCwError {}

impl Display for LibCwError
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError>
	{
		write!(f, "LibCwError")
	}
}

impl ErrorTrait for LibCwError
{
}

///
/// The Rustified type of the
/// [`unixcw`](https://sourceforge.net/projects/unixcw/) API return value.
///
pub type LibCwResult = Result<(), LibCwError>;

impl Into<LibCwResult> for cw_return_values
{
	fn into(self) -> Result<(), LibCwError>
	{
		if 0 == self.0 {
			Err(LibCwError {})
		} else {
			Ok(())
		}
	}
}

#[cfg(test)]
mod tests
{
	use std::ffi::CString;
	use std::thread::{JoinHandle, spawn};

	use super::*;
	use super::super::unixcw_libcw_demo_1;

	#[test]
	fn test_lib_cw_error()
	{
		let lib_cw_error = LibCwError {};

		println!("lib_cw_error = {}.", lib_cw_error);
		println!("lib_cw_error = {:?}.", lib_cw_error);
	}

	#[test]
	fn test_lib_cw_result()
	{
		let lib_cw_result_ok: LibCwResult  = cw_return_values(1).into();
		let lib_cw_result_err: LibCwResult = cw_return_values(0).into();

		assert!(lib_cw_result_ok.is_ok());
		assert!(lib_cw_result_err.is_err());
	}

	#[test]
	fn test_lib_cw_demo_1_parallel()
	{
		let thread_num = 2;
		let mut threads = Vec::<JoinHandle<LibCwResult>>::new();

		for i in 0..thread_num {
			let thread_id = i;
			threads.push(
				spawn(move || {
					println!("thread {thread_id} started.");

					let msg_cstr = CString::new(format!("thread {thread_id}")
						.into_bytes())
						.unwrap();

					// SAFETY: msg_cstr is NUL-terminated.
					let ret = unsafe {
						unixcw_libcw_demo_1(msg_cstr.as_ptr()).into()
					};

					println!("thread {thread_id} finished.");

					ret
				})
			);
		}

		let mut threads_into_iter = threads.into_iter();
		while let Some(join_handle) = threads_into_iter.next() {
			join_handle.join()
				.expect("test thread MUST terminate successfully")
				.expect("libcw demo 1 MUST succeed");
		}
	}
}
