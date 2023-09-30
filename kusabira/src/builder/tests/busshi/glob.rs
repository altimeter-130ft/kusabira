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

use glob::{MatchOptions, Pattern as GlobPattern, PatternError};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::convert::AsRef;
use std::default::Default;
use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::{Error as StdIoError, ErrorKind as StdIoErrorKind};
use std::iter::{FromIterator, Iterator};
use std::path::{Path, PathBuf};
use std::thread_local;

pub trait GlobContext
where Self: Default
{
	fn paths_clear(&mut self) -> &mut Self;
	fn paths_push<T>(&mut self, path: T)
	where T: AsRef<Path>;
	fn paths_push_from_iter<T, IT>(&mut self, iter: IT)
	where T: AsRef<Path>, IT: Iterator<Item = T>;
}

#[derive(Debug)]
struct GlobContextTLS
{
	paths: Vec<PathBuf>,
}

impl Default for GlobContextTLS
{
	fn default() -> Self
	{
		GlobContextTLS {
			paths: Vec::new(),
		}
	}
}

impl GlobContext for GlobContextTLS
{
	fn paths_clear(&mut self) -> &mut Self
	{
		self.paths.clear();
		self
	}

	fn paths_push<T>(&mut self, path: T)
	where T: AsRef<Path>
	{
		let mut path_buf = PathBuf::new();
		path_buf.push(path);
		self.paths.push(path_buf);
	}

	fn paths_push_from_iter<T, IT>(&mut self, mut iter: IT)
	where T: AsRef<Path>, IT: Iterator<Item = T>
	{
		while let Some(path) = iter.next() {
			self.paths_push(path);
		}
	}
}

thread_local!
{
	static GLOB_CONTEXT_TLS: RefCell<GlobContextTLS> =
		RefCell::new(GlobContextTLS::default());
}

#[derive(Debug)]
pub struct GlobContextAccess
{
}

impl Default for GlobContextAccess
{
	fn default() -> Self
	{
		GlobContextAccess {}
	}
}

impl GlobContext for GlobContextAccess
{
	fn paths_clear(&mut self) -> &mut Self
	{
		GLOB_CONTEXT_TLS.with(|ctx|
		{
			ctx.borrow_mut().paths_clear();
		});
		self
	}

	fn paths_push<T>(&mut self, path: T)
	where T: AsRef<Path>
	{
		GLOB_CONTEXT_TLS.with(move |ctx|
		{
			ctx.borrow_mut().paths_push(path);
		});
	}

	fn paths_push_from_iter<T, IT>(&mut self, iter: IT)
	where T: AsRef<Path>, IT: Iterator<Item = T>
	{
		GLOB_CONTEXT_TLS.with(move |ctx|
		{
			ctx.borrow_mut().paths_push_from_iter(iter);
		});
	}
}

pub type GlobResult = Result<PathBuf, GlobError>;

#[derive(Debug)]
pub struct Paths
{
	results: VecDeque<GlobResult>,
}

impl Iterator for Paths
{
	type Item = GlobResult;
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.results.is_empty() {
			None
		} else {
			Some(self.results.pop_front().expect("results MUST NOT be empty"))
		}
	}
}

impl FromIterator<GlobResult> for Paths
{
	fn from_iter<T>(iter: T) -> Self
	where T: IntoIterator<Item = GlobResult>
	{
		let mut paths = Paths {
			results: VecDeque::new(),
		};
		for result in iter.into_iter() {
			paths.results.push_back(result);
		}
		paths
	}
}

#[derive(Debug)]
pub struct GlobError
{
	path: PathBuf,
	error: StdIoError,
}

impl GlobError
{
	pub fn new(path: PathBuf, error: StdIoError) -> GlobError
	{
		GlobError {
			path: path,
			error: error,
		}
	}

	pub fn path(&self) -> &Path
	{
		&self.path
	}

	pub fn error(&self) -> &StdIoError
	{
		&self.error
	}

	pub fn into_error(self) -> StdIoError
	{
		self.error
	}
}

impl Display for GlobError
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError>
	{
		write!(f, "GlobError: (path: {}, error: {})", self.path.display(), self.error)
	}
}

impl StdError for GlobError
{
}

pub fn glob_with(pattern: &str, options: MatchOptions) -> Result<Paths, PatternError>
{
	let pattern = GlobPattern::new(pattern)?;
	let mut matches = GLOB_CONTEXT_TLS.with(|ctx|
	{
		let matches: Vec<PathBuf> = ctx.borrow().paths.iter().filter(|path|
		{
			pattern.matches_path_with(path, options)
		})
		.map(|path| {path.clone()})
		.collect();
		matches
	});
	matches.sort();
	let results = matches.into_iter().map(|path| {Ok(path)});
	Ok(Paths::from_iter(results))
}

pub fn test_glob_with_setup(glob_ctx: &mut GlobContextAccess)
{
	let glob_subject_paths = [
		"src/hello_world_c_1.c",
		"src/hello_world_c_2.c",
		"src/sub/hello_world_c_3.c",
		"src/sub/hello_world_c_4.c",
		"src/hello_world_C_5.c",
		"src/hello_world_C_6.c",
		"src/hello_world_cc_7.cc",
		"src/hello_world_cc_8.cc",
		"src/hello_world_cpp_9.cpp",
		"src/hello_world_cpp_10.cpp",
		"src/hello_world_cxx_11.cxx",
		"src/hello_world_cxx_12.cxx",
		"src/hello_world_s_13.s",
		"src/hello_world_s_14.s",
		"src/hello_world_export_to_rust.h",
		"src/sub/hello_world_internal.h",
		"src/sub/hello_world_internal_hh.hh",
		"src/sub/hello_world_internal_hpp.hpp",
		"src/sub/hello_world_internal_hxx.hxx",
		"src/hello_world_non_c_101",
		"src/hello_world_non_c_102.txt",
		"src/hello_world_non_c_103.txt",
		"src/hello_world_non_c_104.in",
		"tests/hello_world_c_7.c",
		"tests/hello_world_c_8.c",
		"tests/sub/hello_world_tests.h",
	];

	glob_ctx.paths_push_from_iter(glob_subject_paths.iter());
	GLOB_CONTEXT_TLS.with(move |ctx|
	{
		assert_eq!(ctx.borrow_mut().paths.len(), glob_subject_paths.len());
	});
}

mod tests {

use super::*;

#[test]
fn test_struct_glob_context()
{
	let mut glob_ctx = GlobContextAccess::default();
	GLOB_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().paths.len(), 0);
	});

	glob_ctx.paths_push("src/hello_world_export_to_rust.h");
	GLOB_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().paths.len(), 1);
	});
	glob_ctx.paths_push_from_iter(
		[
			"src/hello_world_c_1.c",
			"src/hello_world_c_2.c",
		]
		.iter());
	GLOB_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().paths.len(), 3);
	});

	glob_ctx.paths_clear();
	GLOB_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().paths.len(), 0);
	});

	GLOB_CONTEXT_TLS.with(|ctx|
	{
		println!("glob_ctx (TLS) = {:?}.", ctx.borrow());
	});
	println!("glob_ctx (Access) = {:?}.", glob_ctx);
}

#[test]
fn test_struct_paths()
{
	let path_results = [
		Ok(PathBuf::from(String::from("src/hello_world_c_1.c"))),
		Ok(PathBuf::from(String::from("src/hello_world_c_2.c"))),
		Err(GlobError::new(
			PathBuf::from(String::from("src/hello_world_c_1.c")),
			StdIoError::new(StdIoErrorKind::Other, "test error"))),
		Err(GlobError::new(
			PathBuf::from(String::from("src/hello_world_c_2.c")),
			StdIoError::new(StdIoErrorKind::Other, "another test error"))),
	];
	let paths = Paths::from_iter(path_results.into_iter());
	assert_eq!(paths.results.len(), 4);
	assert_eq!(paths.results.iter().filter(|r| {r.is_ok()}).count(), 2);
	assert_eq!(paths.results.iter().filter(|r| {r.is_err()}).count(), 2);

	println!("paths = {:?}.", paths);
}

#[test]
fn test_glob_with()
{
	let mut glob_ctx = GlobContextAccess::default();

	test_glob_with_setup(&mut glob_ctx);

	let mut match_options = MatchOptions::new();

	let paths = glob_with("src/*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 6);

	let paths = glob_with("src/*.[ch]", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 8);

	let paths = glob_with("tests/*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 2);

	let paths = glob_with("tests/*.[ch]", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 3);

	let paths = glob_with("**/*.[ch]", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 11);

	let paths = glob_with("src/sub/*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 2);

	let paths = glob_with("src/sub/**/*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 2);

	let paths = glob_with("src/**/hello_world_C_*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 2);

	let paths = glob_with("src/sub/**/hello_world_C_*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 0);

	match_options.case_sensitive = false;

	let paths = glob_with("src/hello_world_C_*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 4);

	match_options.case_sensitive = true;

	let paths = glob_with("src/hello_world_C_*.c", match_options).expect("glob_with() MUST succeed");
	assert_eq!(paths.results.len(), 2);
}

#[test]
fn test_glob_error()
{
	let err = GlobError::new(
		PathBuf::from("."),
		StdIoError::new(
			StdIoErrorKind::Other,
			"emulated by mock"));

	assert_eq!(err.path(), PathBuf::from("."));
	assert_eq!(err.error().kind(), StdIoErrorKind::Other);

	println!("glob_error = {}.", err);
	println!("glob_error = {:?}.", err);

	let std_io_err = err.into_error();
	assert_eq!(std_io_err.kind(), StdIoErrorKind::Other);
}

}
