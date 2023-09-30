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

//!
//! The build script demonstrating [`kusabira`].
//!

use kusabira::KusabiraError;
use kusabira::builder::{Config as MldBuilderConfig};
use kusabira::hooks::cc::warnings_into_errors;
use std::convert::From;
use std::fmt::{Display, Error as FmtError, Formatter};
use system_deps::{Config as SystemDepsConfig, Error as SystemDepsError};

///
/// Build the library and
/// [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding file.
///
fn main() -> Result<(), MainError>
{
	MldBuilderConfig::default()
		.lib_name("hello_world_c")
		.c_source_file("src/hello_world_export_to_rust.h")
		.add_c_source_files([
			"src/hello_world_c_*.c",
			"src/unixcw_libcw_demo.c"]
			.into_iter())
		.cc_build_hook(warnings_into_errors)
		.add_cc_build_hook(|build| {build.std("c11")})
		.bindgen_builder_hook(|builder| {builder.generate_block(true)})
		.add_bindgen_builder_hook(|builder| {builder.generate_comments(true)})
		.add_bindgen_builder_hook(|builder| {builder.newtype_enum("cw_return_values")})
		.build()?;

	SystemDepsConfig::new().probe()?;

	Ok(())
}

///
/// The integrated error data.
///
#[derive(Debug)]
enum MainError
{
	KusabiraError(KusabiraError),
	SystemDepsError(SystemDepsError),
}

impl From<KusabiraError> for MainError
{
	fn from(value: KusabiraError) -> Self
	{
		MainError::KusabiraError(value)
	}
}

impl From<SystemDepsError> for MainError
{
	fn from(value: SystemDepsError) -> Self
	{
		MainError::SystemDepsError(value)
	}
}

impl Display for MainError
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError>
	{
		match self {
			MainError::KusabiraError(err) => write!(f, "MainError: {}", err),
			MainError::SystemDepsError(err) => write!(f, "MainError: {}", err),
		}
	}
}
