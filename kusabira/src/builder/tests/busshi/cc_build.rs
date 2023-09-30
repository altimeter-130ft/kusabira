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

use cc::{Build as CcBuild, Error as CcError};
use std::cell::RefCell;
use std::convert::AsRef;
use std::default::Default;
use std::env;
use std::io::{Error as StdIoError, ErrorKind as StdIoErrorKind};
use std::path::{Path, PathBuf};
use std::thread_local;
use std::vec::Vec;

use super::super::ENV_KEY_OUT_DIR;

pub trait CcBuildContext
where Self: Default
{
	fn emulate_error_set(&mut self, emulate_error: bool) -> &mut Self;
	fn emulate_error_get(&self) -> bool;
}

#[derive(Debug)]
struct CcBuildContextTLS
{
	emulate_error: bool,
}

impl Default for CcBuildContextTLS
{
	fn default() -> Self
	{
		CcBuildContextTLS {
			emulate_error: false,
		}
	}
}

impl CcBuildContext for CcBuildContextTLS
{
	fn emulate_error_set(&mut self, emulate_error: bool) -> &mut Self
	{
		self.emulate_error = emulate_error;
		self
	}

	fn emulate_error_get(&self) -> bool
	{
		self.emulate_error
	}
}

thread_local!
{
	static CC_BUILD_CONTEXT_TLS: RefCell<CcBuildContextTLS> =
		RefCell::new(CcBuildContextTLS::default());
}

#[derive(Debug)]
pub struct CcBuildContextAccess
{
}

impl Default for CcBuildContextAccess
{
	fn default() -> Self
	{
		CcBuildContextAccess {}
	}
}

impl CcBuildContext for CcBuildContextAccess
{
	fn emulate_error_set(&mut self, emulate_error: bool) -> &mut Self
	{
		CC_BUILD_CONTEXT_TLS.with(|ctx|
		{
			ctx.borrow_mut().emulate_error_set(emulate_error);
		});
		self
	}

	fn emulate_error_get(&self) -> bool
	{
		CC_BUILD_CONTEXT_TLS.with(|ctx|
		{
			ctx.borrow().emulate_error_get()
		})
	}
}

#[derive(Clone, Debug)]
pub struct Build
{
	build: CcBuild,
	out_dir: PathBuf,
	files: Vec<PathBuf>,
	warnings: bool,
	extra_warnings: bool,
	warnings_into_errors: bool,
	std: Option<String>,
}

impl Build
{
	pub fn out_dir<P: AsRef<Path>>(&mut self, out_dir: P) -> &mut Build
	{
		self.out_dir.clear();
		self.out_dir.push(out_dir);
		self.build.out_dir(&self.out_dir);
		self
	}

	pub fn file<P: AsRef<Path>>(&mut self, p: P) -> &mut Build
	{
		let mut path_buf = PathBuf::new();
		path_buf.push(p);

		self.build.file(&path_buf);
		self.files.push(path_buf);
		self
	}

	pub fn warnings(&mut self, b: bool) -> &mut Build
	{
		self.build.warnings(b);
		self.warnings = b;
		self
	}

	pub fn extra_warnings(&mut self, b: bool) -> &mut Build
	{
		self.build.extra_warnings(b);
		self.extra_warnings = b;
		self
	}

	pub fn warnings_into_errors(&mut self, b: bool) -> &mut Build
	{
		self.build.warnings_into_errors(b);
		self.warnings_into_errors = b;
		self
	}

	pub fn std(&mut self, std: &str) -> &mut Build
	{
		self.build.std(std);
		self.std = Some(String::from(std));
		self
	}

	pub fn try_compile(&self, _output: &str) -> Result<(), CcError>
	{
		let cc_build_ctx = CcBuildContextAccess::default();
		let emulate_error = cc_build_ctx.emulate_error_get();

		if emulate_error {
			Err(CcError::from(
				StdIoError::new(
					StdIoErrorKind::Other,
					"emulated by mock"
				)))
		} else {
			Ok(())
		}
	}
}

impl Default for Build
{
	fn default() -> Self
	{
		Build {
			build: CcBuild::default(),
			out_dir: PathBuf::from(env::var(&ENV_KEY_OUT_DIR).unwrap_or(".".to_string())),
			files: Vec::new(),
			warnings: false,
			extra_warnings: false,
			warnings_into_errors: false,
			std: None,
		}
	}
}

mod tests {

use super::*;

#[test]
fn test_struct_cc_build_context()
{
	let mut cc_build_ctx = CcBuildContextAccess::default();
	CC_BUILD_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().emulate_error, false);
	});
	assert_eq!(cc_build_ctx.emulate_error_get(), false);

	cc_build_ctx.emulate_error_set(true);
	CC_BUILD_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().emulate_error, true);
	});
	assert_eq!(cc_build_ctx.emulate_error_get(), true);

	cc_build_ctx.emulate_error_set(false);
	CC_BUILD_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().emulate_error, false);
	});
	assert_eq!(cc_build_ctx.emulate_error_get(), false);

	CC_BUILD_CONTEXT_TLS.with(|ctx|
	{
		println!("cc_build_ctx (TLS) = {:?}.", ctx.borrow());
	});
	println!("cc_build_ctx (Access) = {:?}.", cc_build_ctx);
}

#[test]
fn test_struct_build()
{
	let mut build = Build::default();

	build.out_dir(".");
	assert_eq!(build.out_dir.to_str(), Some("."));

	let files = ["hello_world_c_1.c", "hello_world_exported_to_rust.h"];
	let mut path_buf = PathBuf::new();
	for filename in files {
		path_buf.clear();
		path_buf.push(filename);
		build.file(&path_buf);
	}
	for filename in files {
		path_buf.clear();
		path_buf.push(filename);
		assert_eq!(build.files.iter().filter(|&fname| {*fname == path_buf}).count(), 1);
	}

	assert_eq!(build.warnings, false);
	assert_eq!(build.extra_warnings, false);
	assert_eq!(build.warnings_into_errors, false);
	build.warnings(true);
	build.extra_warnings(true);
	build.warnings_into_errors(true);
	assert_eq!(build.warnings, true);
	assert_eq!(build.extra_warnings, true);
	assert_eq!(build.warnings_into_errors, true);

	let build_cloned = build.clone();
	println!("build_cloned = {:?}.", build_cloned);
}

}
