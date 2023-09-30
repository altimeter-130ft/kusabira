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

use bindgen::BindgenError;
use cc::Error as CcError;
use glob::PatternError;
use std::convert::From;
use std::error::Error as ErrorTrait;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::Error as StdIoError;

///
/// The error wrapper covering all backends and [`kusabira`](crate).
///
#[derive(Debug)]
pub enum Error {
	///
	/// An error data by [`bindgen::Builder::generate`].
	///
	/// # Example
	/// ```
	/// use bindgen::BindgenError;
	/// use kusabira::error::Error;
	///
	/// let err = Error::from(BindgenError::ClangDiagnostic("sample error".to_string()));
	/// match err {
	/// 	Error::BindgenError(bindgen_err) => match bindgen_err {
	/// 		BindgenError::ClangDiagnostic(msg) => assert_eq!(msg, "sample error"),
	/// 		_ => unreachable!("unexpected BindgenError variant"),
	/// 	},
	/// 	_ => unreachable!("unexpected Error variant"),
	/// };
	/// ```
	///
	BindgenError(BindgenError),
	///
	/// An error data by [`cc::Build::try_compile`].
	///
	/// # Example
	/// ```
	/// use cc::Build;
	/// use kusabira::error::Error;
	///
	/// let build = Build::default();
	/// let try_compile_result = build.try_compile("nonbuildable");
	/// assert!(try_compile_result.is_err());
	/// let err = Error::from(try_compile_result.err().expect("successful build"));
	/// match err {
	/// 	Error::CcError(cc_err) => {
	/// 		();
	/// 	},
	/// 	_ => unreachable!("unexpected Error variant"),
	/// };
	/// ```
	///
	CcError(CcError),
	///
	/// An error data by [`bindgen::Bindings::write_to_file`].
	///
	/// # Example
	/// ```
	/// use std::io::{Error as StdIoError, ErrorKind as StdIoErrorKind};
	/// use kusabira::error::Error;
	///
	/// let err = Error::from(StdIoError::new(StdIoErrorKind::Other, "sample error"));
	/// match err {
	/// 	Error::StdIoError(stdio_err) => {
	/// 		assert_eq!(stdio_err.kind(), StdIoErrorKind::Other);
	/// 	},
	/// 	_ => unreachable!("unexpected Error variant"),
	/// };
	/// ```
	///
	StdIoError(StdIoError),
	///
	/// An error data by [`glob::glob_with`].
	///
	/// # Example
	/// ```
	/// use glob::PatternError;
	/// use kusabira::error::Error;
	///
	/// let err = Error::from(PatternError{pos: 42, msg: "sample error"});
	/// match err {
	/// 	Error::PatternError(pattern_err) => {
	/// 		assert_eq!(pattern_err.pos, 42);
	/// 		assert_eq!(pattern_err.msg, "sample error");
	/// 	},
	/// 	_ => unreachable!("unexpected Error variant"),
	/// };
	/// ```
	///
	PatternError(PatternError),
	///
	/// An error message by [`super::builder::Config::build`].
	///
	/// # Example
	/// ```
	/// use kusabira::error::Error;
	///
	/// let err = Error::from("sample error");
	/// match err {
	/// 	Error::MessageError(msg_err) => assert_eq!(msg_err, "sample error"),
	/// 	_ => unreachable!("unexpected Error variant"),
	/// };
	/// ```
	///
	/// ```
	/// use kusabira::error::Error;
	///
	/// let err = Error::from("sample error".to_string());
	/// match err {
	/// 	Error::MessageError(msg_err) => assert_eq!(msg_err, "sample error"),
	/// 	_ => unreachable!("unexpected Error variant"),
	/// };
	/// ```
	///
	MessageError(String)
}

impl From<BindgenError> for Error {
	fn from(err: BindgenError) -> Self
	{
		Error::BindgenError(err)
	}
}

impl From<CcError> for Error {
	fn from(err: CcError) -> Self
	{
		Error::CcError(err)
	}
}

impl From<StdIoError> for Error {
	fn from(err: StdIoError) -> Self
	{
		Error::StdIoError(err)
	}
}

impl From<PatternError> for Error {
	fn from(err: PatternError) -> Self
	{
		Error::PatternError(err)
	}
}

impl From<String> for Error {
	fn from(err: String) -> Self
	{
		Error::MessageError(err)
	}
}

impl From<&str> for Error {
	fn from(err: &str) -> Self
	{
		Error::MessageError(String::from(err))
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError>
	{
		match self {
			Error::BindgenError(err) => write!(f, "BindgenError: {}", err),
			Error::CcError(err) => write!(f, "CcError: {}", err),
			Error::StdIoError(err) => write!(f, "StdIoError: {}", err),
			Error::PatternError(err) => write!(f, "PatternError: {}", err),
			Error::MessageError(err) => write!(f, "MessageError: {}", err),
		}
	}
}

impl ErrorTrait for Error {
}

#[cfg(test)]
mod tests {

use bindgen::BindgenError;
use cc::Error as CcError;
use glob::Pattern;
use std::io::{Error as StdIoError, ErrorKind as StdIoErrorKind};

use super::*;

#[test]
fn test_from_bindgen_error()
{
	let err = Error::from(BindgenError::ClangDiagnostic(
		"emulated by mock".to_string()));

	println!("err = {}.", err);
	println!("err = {:?}.", err);
}

#[test]
fn test_from_cc_error()
{
	let err = Error::from(CcError::from(
		StdIoError::new(
			StdIoErrorKind::Other,
			"emulated by mock"
		)));

	println!("err = {}.", err);
	println!("err = {:?}.", err);
}

#[test]
fn test_from_std_io_error()
{
	let err = Error::from(StdIoError::new(
		StdIoErrorKind::Other, "emulated by mock"));

	println!("err = {}.", err);
	println!("err = {:?}.", err);
}

#[test]
fn test_from_pattern_error()
{
	let err = Error::from(
		Pattern::new("a**")
		.err()
		.expect("pattern MUST be illegal"));

	println!("err = {}.", err);
	println!("err = {:?}.", err);
}

#[test]
fn test_from_string_error()
{
	let err = Error::from("emulated by mock".to_string());

	println!("err = {}.", err);
	println!("err = {:?}.", err);
}

#[test]
fn test_from_str_error()
{
	let err = Error::from("emulated by mock");

	println!("err = {}.", err);
	println!("err = {:?}.", err);
}

}
