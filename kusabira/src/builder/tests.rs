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

use bindgen::BindgenError;
use cc::Error as CcError;
use glob::Pattern;
use std::ffi::OsString;
use std::mem::discriminant;
use std::io::{Error as StdIoError, ErrorKind as StdIoErrorKind};
use std::path::MAIN_SEPARATOR;

use super::*;

pub mod busshi;

#[test]
fn test_default_contents()
{
	use std::env::set_var;

	let out_dir: PathBuf = [
		&String::from(MAIN_SEPARATOR),
		"tmp",
		"out_dir"
	]
		.into_iter()
		.collect();
	set_var("OUT_DIR", out_dir.to_str().unwrap());
	let config = Config::default();

	assert_eq!(config.out_dir, out_dir);
	assert_eq!(config.input_files.len(), 0);
	assert!(config.lib_name.is_none());
	// The ([`Config::cc_build_hook`],
	// [`Config::bindgen_builder_hook`] and
	// [`Config::glob_matchoptions_hook`]) hooks are not tested; traits
	// [`std::ops::FnOnce`] and [`std::ops::FnMut`] (and [`std::ops::Fn`])
	// do not implement [`std::cmp::Eq`].
}

#[test]
fn test_out_dir()
{
	let out_dir: PathBuf = [
		&String::from(MAIN_SEPARATOR),
		"tmp",
		"out_dir"
	]
		.into_iter()
		.collect();
	let config = Config::default()
		.out_dir(out_dir.as_ref());

	assert_eq!(config.out_dir.to_str().expect("non-string on left"),
		out_dir.to_str().expect("non-string on right"));
}

#[test]
fn test_source_file()
{
	let input_file = "hello_world_c_1_*.c";
	let config = Config::default();
	assert_eq!(config.input_files.len(), 0);

	let config = config.input_file(input_file);
	assert_eq!(config.input_files.len(), 1);
	assert_source_files(&config, [input_file].into_iter());
}

#[test]
fn test_source_files()
{
	let input_file = [
		"hello_world_c_1_*.c",
		"hello_world_c_2_*.c",
		];
	let config = Config::default();
	assert_eq!(config.input_files.len(), 0);

	let config = config.input_files(input_file.into_iter());
	assert_eq!(config.input_files.len(), input_file.len());
	assert_source_files(&config, input_file.into_iter());
}

#[test]
fn test_add_source_file()
{
	let header_file = "hello_world_c_exported.h";
	let input_file = "hello_world_c_1_*.c";
	let config = Config::default()
		.input_file(header_file);
	assert_eq!(config.input_files.len(), 1);

	let config = config.add_input_file(input_file);
	assert_eq!(config.input_files.len(), 2);
	assert_source_files(&config, [header_file].into_iter());
	assert_source_files(&config, [input_file].into_iter());
}

#[test]
fn test_add_source_files()
{
	let header_file = "hello_world_c_exported.h";
	let input_files = [
		"hello_world_c_1_*.c",
		"hello_world_c_2_*.c"
	];
	let config = Config::default()
		.input_file(header_file);
	assert_eq!(config.input_files.len(), 1);

	let config = config.add_input_files(input_files.into_iter());
	assert_eq!(config.input_files.len(), 1 + input_files.len());
	assert_source_files(&config, [header_file].into_iter());
	assert_source_files(&config, input_files.into_iter());
}

fn assert_source_files<'a, IT>(config: &Config, fn_iter: IT)
where IT: Iterator<Item = &'a str>
{
	for filename in fn_iter {
		assert_eq!(
			config
			.input_files.iter()
			.filter(|config_fn|
			{
				*config_fn == &filename
			})
			.count(), 1);
	}
}

#[test]
fn test_lib_name()
{
	let lib_name = "hello_world";
	let config = Config::default()
		.lib_name(lib_name);
	assert_eq!(config.lib_name, Some(lib_name));
}

#[test]
fn test_binding_ext()
{
	let binding_ext = "rs";
	let config = Config::default()
		.binding_ext(binding_ext);
	assert_eq!(config.binding_ext, binding_ext);
}

// The hook configuration methods are not covered as their own unit tests;
// refer to [`test_default_contents`] for the detail.

fn test_build_setup(mut config: Config, config_lib_name: bool) -> (Config, StdPathBuf)
{
	use busshi::glob::{GlobContextAccess, test_glob_with_setup};

	let mut glob_ctx = GlobContextAccess::default();
	test_glob_with_setup(&mut glob_ctx);

	let out_dir: StdPathBuf =
	[
		&String::from(MAIN_SEPARATOR),
		"tmp",
		"out_dir"
	].into_iter().collect();
	config = config.out_dir(out_dir.as_ref());

	if config_lib_name {
		config = config.lib_name("hello_world");
	}

	(config, out_dir)
}

#[test]
fn test_build_success_simple()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.[ch]");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert_eq!(build_results.lib_name, Some("hello_world".to_string()));
	assert_eq!(build_results.source_files.len(), 6);
	assert_eq!(build_results.header_bindings.len(), 2);
}

#[test]
fn test_build_success_with_hooks()
{
	use super::super::hooks::cc::warnings_into_errors;
	use super::super::hooks::glob::case_insensitive;

	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.[ch]")
		.cc_build_hook(warnings_into_errors)
		.add_cc_build_hook(|build| {build.std("c11")})
		.bindgen_builder_hook(|builder| {builder.generate_block(true)})
		.add_bindgen_builder_hook(|builder| {builder.generate_comments(true)})
		.glob_matchoptions_hook(case_insensitive)
		.add_glob_matchoptions_hook(|mut match_options|
		{
			match_options.require_literal_separator = true;
			match_options
		});
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert_eq!(build_results.lib_name, Some("hello_world".to_string()));
	assert_eq!(build_results.source_files.len(), 6);
	assert_eq!(build_results.header_bindings.len(), 2);
}

#[test]
fn test_build_fail_out_dir_not_dir()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.[ch]");
	config.out_dir.is_dir = false;
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_fail_nothing_built()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("nonexist/**/*.[ch]");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_fail_no_lib_name()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, false);

	config = config.input_file("src/**/*.[ch]");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_success_c_only()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.c");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert_eq!(build_results.lib_name, Some("hello_world".to_string()));
	assert_eq!(build_results.source_files.len(), 6);
	assert_eq!(build_results.header_bindings.len(), 0);
}

#[test]
fn test_build_success_cc_only()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.cc");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert_eq!(build_results.lib_name, Some("hello_world".to_string()));
	assert_eq!(build_results.source_files.len(), 2);
	assert_eq!(build_results.header_bindings.len(), 0);
}

#[test]
fn test_build_fail_cc_source_ext_deleted()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.cc")
		.delete_source_ext("cc");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_fail_cc_source_ext_deleted_double()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.cc")
		.delete_source_ext("cc")
		.delete_source_ext("cc");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_success_h_only()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.h");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert!(build_results.lib_name.is_none());
	assert_eq!(build_results.source_files.len(), 0);
	assert_eq!(build_results.header_bindings.len(), 2);
}

#[test]
fn test_build_success_hh_only()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.hh");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert!(build_results.lib_name.is_none());
	assert_eq!(build_results.source_files.len(), 0);
	assert_eq!(build_results.header_bindings.len(), 1);
}

#[test]
fn test_build_fail_hh_header_ext_deleted()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.hh")
		.delete_header_ext("hh");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_fail_hh_header_ext_deleted_double()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.hh")
		.delete_header_ext("hh")
		.delete_header_ext("hh");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_fail_unsupported_exts()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*_non_c_*");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_fail_txt_unsupported()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.txt");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_success_txt_source_ext_added()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.txt")
		.add_source_ext("txt");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert_eq!(build_results.lib_name, Some("hello_world".to_string()));
	assert_eq!(build_results.source_files.len(), 2);
	assert_eq!(build_results.header_bindings.len(), 0);
}

#[test]
fn test_build_success_txt_source_ext_added_double()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.txt")
		.add_source_ext("txt")
		.add_source_ext("txt");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert_eq!(build_results.lib_name, Some("hello_world".to_string()));
	assert_eq!(build_results.source_files.len(), 2);
	assert_eq!(build_results.header_bindings.len(), 0);
}

#[test]
fn test_build_fail_in_unsupported()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.in");
	let result = config.build();
	assert!(result.is_err());
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&MldError::MessageError("".to_string())));
}

#[test]
fn test_build_success_in_source_ext_added()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.in")
		.add_header_ext("in");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert!(build_results.lib_name.is_none());
	assert_eq!(build_results.source_files.len(), 0);
	assert_eq!(build_results.header_bindings.len(), 1);
}

#[test]
fn test_build_success_in_source_ext_added_double()
{
	let mut config = Config::default();
	let out_dir;

	(config, out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.in")
		.add_header_ext("in")
		.add_header_ext("in");
	let result = config.build();
	let build_results = result.expect("build MUST succeed");
	assert_eq!(build_results.out_dir, out_dir);
	assert!(build_results.lib_name.is_none());
	assert_eq!(build_results.source_files.len(), 0);
	assert_eq!(build_results.header_bindings.len(), 1);
}

#[test]
fn test_build_fail_cc_build_error()
{
	use busshi::cc_build::*;

	let mut cc_build_ctx = CcBuildContextAccess::default();

	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.[ch]");
	cc_build_ctx.emulate_error_set(true);
	let result = config.build();
	assert!(result.is_err());
	let err = MldError::from(CcError::from(
		StdIoError::new(
			StdIoErrorKind::Other,
			"emulated by mock"
		)));
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&err));
}

#[test]
fn test_build_fail_bindgen_build_generate_error()
{
	use busshi::bindgen_builder::*;

	let mut bindgen_builder_ctx = BindgenBuilderContextAccess::default();

	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.[ch]");
	bindgen_builder_ctx.emulate_generate_error_set(true);
	let result = config.build();
	assert!(result.is_err());
	let err = MldError::from(BindgenError::ClangDiagnostic(
		"emulated by mock".to_string()));
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&err));
}

#[test]
fn test_build_fail_bindgen_build_write_error()
{
	use busshi::bindgen_builder::*;

	let mut bindgen_builder_ctx = BindgenBuilderContextAccess::default();

	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/**/*.[ch]");
	bindgen_builder_ctx.emulate_write_error_set(true);
	let result = config.build();
	assert!(result.is_err());
	let err = MldError::from(StdIoError::new(
		StdIoErrorKind::Other, "emulated by mock"));
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&err));
}

#[test]
fn test_build_fail_glob_pattern_error()
{
	let mut config = Config::default();
	let _out_dir;

	(config, _out_dir) = test_build_setup(config, true);

	config = config.input_file("src/a**/*.[ch]");
	let result = config.build();
	assert!(result.is_err());
	let err = MldError::from(
		Pattern::new("a**")
		.err()
		.expect("pattern MUST be illegal"));
	assert_eq!(discriminant(&(result.err().expect("MUST be error"))),
		discriminant(&err));
}

#[test]
fn test_header_binding()
{
	let header_binding = HeaderBinding::from((
		StdPathBuf::from("hello_world_c_exported.h".to_string()),
		StdPathBuf::from("hello_world_c_exported.in".to_string())));

	println!("header_binding = {}.", header_binding);
	println!("header_binding = {:?}.", header_binding);
}

#[test]
fn test_build_results()
{
	let mut build_results = BuildResults::new();

	println!("build_results (1) = {}.", build_results);
	println!("build_results (1) = {:?}.", build_results);

	build_results.lib_name = Some("hello_world".to_string());
	build_results.source_files =
	[
		"hello_world_c_1.c",
		"hello_world_c_2.c",
	].into_iter()
		.map(|x| {StdPathBuf::from(x.to_string())})
		.collect();
	build_results.header_bindings =
	[
		("hello_world_c_exported_1.h", "hello_world_c_exported_1.in"),
		("hello_world_c_exported_2.h", "hello_world_c_exported_2.in"),
	].into_iter()
		.map(|x| -> HeaderBinding
		{
			let paths = (StdPathBuf::from(x.0.to_string()),
				StdPathBuf::from(x.1.to_string()));
			HeaderBinding::from(paths)
		}).collect();

	println!("build_results (2) = {}.", build_results);
	println!("build_results (2) = {:?}.", build_results);

	// std::process::ExitCode does not support PartialEq.
	build_results.report();
}

#[test]
fn test_file_type()
{
	let config = Config::default();

	for e in SOURCE_EXTS.iter() {
		let path_buf = StdPathBuf::from("hello_world_c_1.".to_string() + e);

		let ext = config.find_filetype(path_buf.extension());
		assert_eq!(ext, FileType::Source);
		println!("ext (1) = {}.", ext);
		println!("ext (1) = {:?}.", ext);
	}

	for e in HEADER_EXTS.iter() {
		let path_buf = StdPathBuf::from("hello_world_c_1.".to_string() + e);

		let ext = config.find_filetype(path_buf.extension());
		assert_eq!(ext, FileType::Header);
		println!("ext (2) = {}.", ext);
		println!("ext (2) = {:?}.", ext);
	}

	let path_buf = StdPathBuf::from("hello_world_c_1.txt".to_string());

	let ext = config.find_filetype(path_buf.extension());
	assert_eq!(ext, FileType::Unsupported("txt".to_string()));
	println!("ext (3) = {}.", ext);
	println!("ext (3) = {:?}.", ext);

	let mut path_buf = StdPathBuf::from("hello_world_c_1".to_string());

	let ext = config.find_filetype(path_buf.extension());
	assert_eq!(ext, FileType::Unsupported("".to_string()));
	println!("ext (4) = {}.", ext);
	println!("ext (4) = {:?}.", ext);

	path_buf.set_extension(invalid_path_str().as_os_str());

	let ext = config.find_filetype(path_buf.extension());
	assert_eq!(ext, FileType::Unsupported("".to_string()));
	println!("ext (4) = {}.", ext);
	println!("ext (4) = {:?}.", ext);
}

#[cfg(unix)]
pub fn invalid_path_str() -> OsString
{
	use std::os::unix::ffi::OsStrExt;

	let source = [b'b', b'i', 0x80, b'n'];
	let mut os_str = OsString::new();
	os_str.push(OsStr::from_bytes(&source));

	os_str
}

#[cfg(windows)]
pub fn invalid_path_str() -> OsString
{
	use std::os::windows::prelude::*;

	let source = [0x0062, 0x0069, 0xd800, 0x6e];

	OsString::from_wide(&source)
}
