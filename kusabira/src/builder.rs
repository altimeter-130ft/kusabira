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
//! [`builder::Config`] holds the builder configuration.  It is created with the
//! default parameters by [`builder::Config::default`].
//!
//! The builder is then configured by the methods on [`builder::Config`].  Both
//! the C source files (`*.c`) and header files (`*.h`) can be configured
//! together.  At least one C source or header file MUST be configured.
//!
//! The backends can also be configured via [`builder::Config`].  In order to
//! make the [`kusabira`](crate) API robust against the changes on the backend
//! configuration design, they are configured by the _hooks_, namely the
//! user-supplied function or closure that receives the backend-dependent
//! configuration, modifies it and returns it.
//!
//! Finally, the builder is executed by [`builder::Config::build`], which
//! builds the library and Rust binding files altogether.
//! [`builder::Config::build`] checks the extention of each C source file;
//! the `*.c` files are passed together to [`cc::Build::try_compile`] to build
//! a single library, while the `*.h` files are passed in the one-by-one manner
//! to [`bindgen::Builder::generate`] and [`bindgen::Bindings::write_to_file`].
//! Both of these behaviors reflect the API design of the backends.
//!
//! The glob expansion by [`glob`] is supported on the C source files.  This
//! happens during the execution of [`builder::Config::build`].
//!
//! # RECOMMENDED C Source File Configurtion
//! * C source files (`*.c`)
//!
//!   Configure all of them together, possibly by the glob.  They are all
//!   compiled into a single library, which is then linked automatically with
//!   the Rust object files by [`Cargo`](https://doc.rust-lang.org/cargo/).
//!
//! * C header files (`*.h`)
//!
//!   Create a single C header file that `#include`s all of the C header files
//!   exported to the Rust source files.  Configure only this C header file to
//!   [`builder::Config`].  On the Rust side, `include!` the generated binding
//!   file in a source file.
//!
//!   This configuration is recommended because
//!   [`bindgen::Builder::generate`] requires a C header file completely
//!   pre-processable and compilable on its own, while most C header files
//!   assume some other header files `#include`d before them.
//!
//!   Mind that the identifier scope of Rust is different from C.  In Rust, you
//!   `include!` a binding file somewhere in a module, and the identifiers in
//!   it are visible anywhere in the module.
//!
//!   Also note that the generated binding file SHOULD NOT be compiled
//!   directly.  [`rustc`](https://doc.rust-lang.org/rustc/) assumes the
//!   certain source file hierarchy to define the modules, which requires some
//!   tricks to follow.  In addition,
//!   [`Cargo`](https://doc.rust-lang.org/cargo/) may exhibit an unexpected
//!   behavior if the build script emits any files into the source directory.
//!
//!   A good practice in a large-scaled project is to `include!` the binding
//!   file into a dedicated module, and `use` the required items only to avoid
//!   flooding a module by many unused identifiers.  As of version 0.68.1,
//!   [`bindgen`] adds `pub` to every bound identifier.
//!
//!   Alternatively, you can also create multiple C header files and configure
//!   all of them, as long as each header file pre-processes and compiles on
//!   its own.  This is a good option if you multiple features implemented in
//!   C and each Rust module does not require all C features.
//!
//! # Internal Design Notes
//! ## Path Storage
//! The output directory is stored as [`std::path::PathBuf`] because it MUST
//! point to a valid directory path upon building.
//!
//! In contrary, the C source glob patterns are represented by
//! [`std::string::String`].  A glob pattern do not necessarily have to be a
//! valid path, so [`std::path::PathBuf`] is too restrictive.  [`glob`] deals
//! with a pattern in the same way.
//!
//! ## Hook Addition
//!
//! The following methods on [`builder::Config`] add a new hook to the existing
//! one in the configuration:
//!
//! * [`builder::Config::add_cc_build_hook`]
//! * [`builder::Config::add_bindgen_builder_hook`]
//! * [`builder::Config::add_glob_matchoptions_hook`]
//!
//! Internally, these methods create a new hook closure in which the input
//! parameter is first passed to the existing hook, and its result is then
//! passed to the new hook.  Below is the pseudocode of the internal hook
//! closure created by [`builder::Config::add_cc_build_hook`]:
//! ```
//! use cc::Build;
//! use std::ops::FnOnce;
//!
//! let existing_hook: Box<dyn FnOnce(&mut Build) -> &mut Build>
//! 	= Box::new(|build: &mut Build|
//! 		{
//! 			// Work on build.
//! 			build
//! 		});
//! let added_hook: Box<dyn FnOnce(&mut Build) -> &mut Build>
//! 	= Box::new(|build: &mut Build|
//! 		{
//! 			// Another work on build.
//! 			build
//! 		});
//!
//! let new_hook: Box<dyn FnOnce(&mut Build) -> &mut Build>
//! 	= Box::new(
//! 		move |build: &mut Build|
//! 		{
//! 			added_hook(existing_hook(build))
//! 		});
//! ```
//!
//! The other methods create the internal hook closure in the same way except
//! for the parameter and return type, and the hook function trait.
//!

#![deny(missing_docs)]

#[cfg(not(test))]
use bindgen::Builder;
#[cfg(test)]
use tests::busshi::bindgen_builder::Builder;
use bindgen::CargoCallbacks;
#[cfg(not(test))]
use cc::Build;
#[cfg(test)]
use tests::busshi::cc_build::Build;
#[cfg(not(test))]
use glob::glob_with;
#[cfg(test)]
use tests::busshi::glob::glob_with;
use glob::MatchOptions;
use std::boxed::Box;
use std::cell::RefCell;
use std::convert::AsRef;
use std::env;
use std::ffi::OsStr;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::iter::Iterator;
use std::path::Path;
#[cfg(not(test))]
use std::path::PathBuf;
#[cfg(test)]
use tests::busshi::std_path_path_buf::PathBuf;
use std::path::PathBuf as StdPathBuf;
use std::process::{ExitCode, Termination};

use super::error::Error as MldError;
use super::hooks::bindgen::reflect as reflect_bindgen;
use super::hooks::cc::reflect as reflect_cc;
use super::hooks::glob::reflect as reflect_glob;

// This `use` is required for the document to link to
// `system_deps::Config::probe`.
#[cfg(doc)]
use system_deps::Config as SystemDepsConfig;

#[cfg(test)]
/// The tests for [`builder`].
pub mod tests;

/// The environment variable name [`Cargo`](https://doc.rust-lang.org/cargo/)
/// configures the output directory.
pub static ENV_KEY_OUT_DIR: &str = "OUT_DIR";

///
/// The configuration parameters, as well as the entry to the builder engine.
///
/// All of the configuration methods return `self` by the value, so the method
/// calls can be chained.
///
pub struct Config<'a>
{
	out_dir: PathBuf,
	c_source_files: Vec<&'a str>,
	lib_name: Option<&'a str>,
	cc_build_hook: RefCell<Box<dyn FnOnce(&mut Build) -> &mut Build + 'a>>,
	bindgen_builder_hook: RefCell<Box<dyn FnMut(Builder) -> Builder + 'a>>,
	glob_matchoptions_hook: RefCell<Box<dyn FnOnce(MatchOptions) -> MatchOptions + 'a>>,
}

impl<'a> Default for Config<'a>
{
	///
	/// Generate the configuration with the default parameters.
	///
	/// # Default Parameters
	/// * *Output Directory*: The value of environment variable `OUT_DIR` if
	///    defined, the current directory (`.`) otherwise.
	/// * *C Source Files*: None.
	/// * *Library Name*: None.
	/// * *[`cc::Build`] Configuration Hook*: [`super::hooks::cc::reflect`].
	/// * *[`bindgen::Builder`] Configuration Hook*: [`super::hooks::bindgen::reflect`].
	/// * *[`glob::MatchOptions`] Configuration Hook*: [`super::hooks::glob::reflect`].
	///
	/// # Example
	/// ```
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default();
	/// ```
	///
	fn default() -> Config<'a>
	{
		Config {
			out_dir: PathBuf::from(env::var(&ENV_KEY_OUT_DIR).unwrap_or(".".to_string())),
			c_source_files: Vec::new(),
			lib_name: None,
			cc_build_hook: RefCell::new(Box::new(reflect_cc)),
			bindgen_builder_hook: RefCell::new(Box::new(reflect_bindgen)),
			glob_matchoptions_hook: RefCell::new(Box::new(reflect_glob)),
		}
	}
}

impl<'a> Config<'a>
{
	///
	/// Set the output directory of the generated object, library and Rust
	/// files.
	///
	/// The output directory MUST be available before calling
	/// [`Config::build`].
	///
	/// # Caveat
	/// This SHOULD NOT be called upon the build by
	/// [`Cargo`](https://doc.rust-lang.org/cargo/).  The primary purpose of
	/// this method is for the tests and
	/// non-[`Cargo`](https://doc.rust-lang.org/cargo/) usages.
	///
	/// # Example
	/// ```
	/// use std::path::{PathBuf, MAIN_SEPARATOR};
	/// use kusabira::builder::Config;
	///
	/// let out_dir: PathBuf = [&String::from(MAIN_SEPARATOR), "tmp", "out_dir"]
	/// 	.iter()
	/// 	.collect();
	/// let config = Config::default()
	/// 	.out_dir(&out_dir);
	/// ```
	///
	pub fn out_dir(mut self, out_dir: &Path) -> Self
	{
		self.out_dir.clear();
		self.out_dir.push(out_dir);
		self
	}

	///
	/// Set a single C source file.
	///
	/// The filename MAY be a [`glob`] pattern.
	///
	/// Any existing C source files are removed from the configuration.
	///
	/// # Example
	/// ```
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.c_source_file("hello_world_c_*.c");
	/// ```
	///
	pub fn c_source_file(mut self, c_filename: &'a str) -> Self
	{
		self.c_source_files.clear();
		self.add_c_source_file(c_filename)
	}

	///
	/// Set either a single or multiple C source files via an iterator.
	///
	/// The filename MAY be a [`glob`] pattern.
	///
	/// Any existing C source files are removed from the configuration.
	///
	/// # Example
	/// ```
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.c_source_files(
	/// 		["hello_world_c_1_*.c", "hello_world_c_2_*.c"]
	/// 		.into_iter());
	/// ```
	///
	pub fn c_source_files<IT>(mut self, c_filename_iter: IT) -> Self
		where IT: Iterator<Item = &'a str>
	{
		self.c_source_files.clear();
		self.add_c_source_files(c_filename_iter)
	}

	///
	/// Add a single C source file.
	///
	/// The filename MAY be a [`glob`] pattern.
	///
	/// Any existing C source files are preserved in the configuration.
	///
	/// # Example
	/// ```
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.c_source_file("hello_world_c_exported.h")
	/// 	.add_c_source_file("hello_world_c_*.c");
	/// ```
	///
	pub fn add_c_source_file(mut self, c_filename: &'a str) -> Self
	{
		self.c_source_files.push(c_filename);
		self
	}

	///
	/// Add either a single or multiple C source files via an iterator.
	///
	/// The filename MAY be a [`glob`] pattern.
	///
	/// Any existing C source files are preserved in the configuration.
	///
	/// # Example
	/// ```
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.c_source_file("hello_world_c_exported.h")
	/// 	.add_c_source_files(
	/// 		["hello_world_c_1_*.c", "hello_world_c_2_*.c"]
	/// 		.into_iter());
	/// ```
	///
	pub fn add_c_source_files<IT>(mut self, c_filename_iter: IT) -> Self
		where IT: Iterator<Item = &'a str>
	{
		for c_filename in c_filename_iter {
			self.c_source_files.push(c_filename);
		}
		self
	}

	///
	/// Set the output library name.
	///
	/// This library named is passed to [`cc::Build::try_compile`].
	///
	/// Refer to [`cc::Build::compile`] for the convention upon the library
	/// name.
	///
	/// # Example
	/// ```
	/// use kusabira::builder::Config;
	///
	/// let lib_name = "test_lib";
	/// let config = Config::default()
	/// 	.lib_name(lib_name);
	/// ```
	///
	pub fn lib_name(mut self, lib_name: &'a str) -> Self
	{
		self.lib_name = Some(lib_name);
		self
	}

	///
	/// Set the hook to configure [`cc::Build`].
	///
	/// The configured hook is called only once during the execution of
	/// [`Config::build`].
	///
	/// The configured hook replaces the old one.
	///
	/// The parameter type of the hook is aligned to the configuration methods
	/// of [`cc::Build`], which receives and returns `&mut cc::Build`.
	///
	/// # Example
	/// ```
	/// use cc::Build;
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.cc_build_hook(|build: &mut Build| {build});
	/// ```
	///
	/// The [`FnOnce`] trait is sufficient on the hook, so it MAY consume any
	/// external data.
	/// ```
	/// use cc::Build;
	/// use kusabira::builder::Config;
	///
	/// let consumed = String::from("c11");
	/// let mut consumer = vec![];
	/// let _config = Config::default()
	/// 	.cc_build_hook(
	/// 		|build: &mut Build|
	/// 		{
	/// 			build.std(&consumed);
	/// 			consumer.push(consumed);
	/// 			build
	/// 		});
	/// ```
	///
	pub fn cc_build_hook<CcBuildHook>(
		mut self,
		cc_build_hook: CcBuildHook)
		-> Self
		where CcBuildHook: FnOnce(&mut Build) -> &mut Build + 'a
	{
		self.cc_build_hook = RefCell::new(Box::new(cc_build_hook));
		self
	}

	///
	/// Add a new hook to configure [`cc::Build`].
	///
	/// The configured hook is called only once during the execution of
	/// [`Config::build`].
	///
	/// Refer to [the Hook Addition section](super::builder#hook-addition) for
	/// the detail of the hook generated by this method.
	///
	/// The parameter type of the hook is aligned to the configuration methods
	/// of [`cc::Build`], which receives and returns `&mut cc::Build`.
	///
	/// # Example
	/// ```
	/// use cc::Build;
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.add_cc_build_hook(|build: &mut Build| {build});
	/// ```
	///
	/// Refer to the example on [`Config::cc_build_hook`] for the usage of a
	/// hook with only the [`FnOnce`] trait.
	///
	pub fn add_cc_build_hook<CcBuildHook>(
		mut self,
		cc_build_hook: CcBuildHook)
		-> Self
		where CcBuildHook: FnOnce(&mut Build) -> &mut Build + 'a
	{
		let cc_build_hook_fn = (self.cc_build_hook)
			.replace(Box::new(reflect_cc));
		self.cc_build_hook = RefCell::new(Box::new(move |build: &mut Build| {
			cc_build_hook(cc_build_hook_fn(build))
		}));
		self
	}

	///
	/// Set the hook to configure [`bindgen::Builder`].
	///
	/// The configured hook is called for each configured C header file during
	/// the execution of [`Config::build`].
	///
	/// The configured hook replaces the old one.
	///
	/// The parameter type of the hook is aligned to the configuration methods
	/// of [`bindgen::Builder`], which receives and returns `bindgen::Builder`.
	///
	/// # Example
	/// ```
	/// use bindgen::Builder;
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.bindgen_builder_hook(|builder: Builder| {builder});
	/// ```
	///
	/// Mind that the configuration methods of [`bindgen::Builder`] tend to
	/// consume `self`, which is returned as a separate value, as well as
	/// the parameters.  Implement your hooks so that it does not miss the
	/// [`FnMut`] trait and do not forget to receive the returned
	/// [`bindgen::Builder`] data.
	///
	/// A hook below compiles and works because it does not consume
	/// `rustified_enum`, so the hook has the [`FnMut`] trait:
	/// ```
	/// use bindgen::Builder;
	/// use kusabira::builder::Config;
	///
	/// let rustified_enum = String::from("rust_style_enum");
	/// let _config = Config::default()
	/// 	.bindgen_builder_hook(
	/// 		|builder: Builder|
	/// 		{
	/// 			let builder = builder.rustified_enum(&rustified_enum);
	/// 			builder
	/// 		});
	/// ```
	///
	/// The following hook consumes `rustified_enum`, so it does not have the
	/// [`FnMut`] trait and causes a compilation error:
	/// ```compile_fail
	/// use bindgen::Builder;
	/// use kusabira::builder::Config;
	///
	/// let rustified_enum = String::from("rust_style_enum");
	/// let _config = Config::default()
	/// 	.bindgen_builder_hook(
	/// 		|builder: Builder|
	/// 		{
	/// 			let builder = builder.rustified_enum(rustified_enum);
	/// 			// rustified_enum is lost!
	/// 			builder
	/// 		});
	/// ```
	///
	/// A hook also fails to compile if the return value from a
	/// [`bindgen::Builder`] configuration method is not received nor returned
	/// from the hook:
	/// ```compile_fail
	/// use bindgen::Builder;
	/// use kusabira::builder::Config;
	///
	/// let rustified_enum = String::from("rust_style_enum");
	/// let _config = Config::default()
	/// 	.bindgen_builder_hook(
	/// 		|builder: Builder|
	/// 		{
	/// 			builder.rustified_enum(&rustified_enum);
	/// 			// builder is lost!
	/// 			builder
	/// 		});
	/// ```
	///
	pub fn bindgen_builder_hook<BindgenBuildHook>(
		mut self,
		bindgen_builder_hook: BindgenBuildHook)
		-> Self
		where BindgenBuildHook: FnMut(Builder) -> Builder + 'a
	{
		self.bindgen_builder_hook = RefCell::new(Box::new(bindgen_builder_hook));
		self
	}

	///
	/// Add a new hook to configure [`bindgen::Builder`].
	///
	/// The configured hook is called for each configured C header file during
	/// the execution of [`Config::build`].
	///
	/// Refer to [the Hook Addition section](super::builder#hook-addition) for
	/// the detail of the hook generated by this method.
	///
	/// The parameter type of the hook is aligned to the configuration methods
	/// of [`bindgen::Builder`], which receives and returns `bindgen::Builder`.
	///
	/// Refer to the example on [`Config::bindgen_builder_hook`] for the
	/// limitation due to the hook being bound to [`FnMut`].
	///
	/// # Example
	/// ```
	/// use bindgen::Builder;
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.add_bindgen_builder_hook(|builder: Builder| {builder});
	/// ```
	///
	pub fn add_bindgen_builder_hook<BindgenBuildHook>(
		mut self,
		bindgen_builder_hook: BindgenBuildHook)
		-> Self
		where BindgenBuildHook: FnMut(Builder) -> Builder + 'a
	{
		let mut bindgen_builder_hook_ = bindgen_builder_hook;
		let mut bindgen_builder_hook_fn = self.bindgen_builder_hook
			.replace(Box::new(reflect_bindgen));
		self.bindgen_builder_hook = RefCell::new(Box::new(move |builder: Builder| {
			bindgen_builder_hook_(bindgen_builder_hook_fn(builder))
		}));
		self
	}

	///
	/// Set the hook to configure [`glob::MatchOptions`].
	///
	/// The configured hook is called only once during the execution of
	/// [`Config::build`].
	///
	/// The configured hook replaces the old one.
	///
	/// Since the members of [`glob::MatchOptions`] are all `pub`, the
	/// parameter type of the hook may be either `MatchOptions` or
	/// `&mut MatchOptions`.  This implementation opts to the first.
	///
	/// # Example
	/// ```
	/// use glob::MatchOptions;
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.glob_matchoptions_hook(|match_options: MatchOptions| {match_options});
	/// ```
	///
	/// The [`FnOnce`] trait is sufficient on the hook, so it MAY consume any
	/// external data.
	/// ```
	/// use glob::MatchOptions;
	/// use kusabira::builder::Config;
	///
	/// let case_sensitive = false;
	/// let consumed = String::from("dummy");
	/// let mut consumer = vec![];
	/// let _config = Config::default()
	/// 	.glob_matchoptions_hook(
	/// 		|mut match_options: MatchOptions|
	/// 		{
	/// 			match_options.case_sensitive = case_sensitive;
	/// 			consumer.push(consumed);
	/// 			match_options
	/// 		});
	/// ```
	///
	pub fn glob_matchoptions_hook<GlobMatchOptionsHook>(
		mut self,
		glob_matchoptions_hook: GlobMatchOptionsHook)
		-> Self
		where GlobMatchOptionsHook: FnOnce(MatchOptions) -> MatchOptions + 'a
	{
		self.glob_matchoptions_hook = RefCell::new(Box::new(glob_matchoptions_hook));
		self
	}

	///
	/// Add a new hook to configure [`glob::MatchOptions`].
	///
	/// The configured hook is called only once during the execution of
	/// [`Config::build`].
	///
	/// Refer to [the Hook Addition section](super::builder#hook-addition) for
	/// the detail of the hook generated by this method.
	///
	/// Since the members of [`glob::MatchOptions`] are all `pub`, the
	/// parameter type of the hook may be either `MatchOptions` or
	/// `&mut MatchOptions`.  This implementation opts to the first.
	///
	/// # Example
	/// ```
	/// use glob::MatchOptions;
	/// use kusabira::builder::Config;
	///
	/// let config = Config::default()
	/// 	.add_glob_matchoptions_hook(|match_options: MatchOptions| {match_options});
	/// ```
	///
	/// Refer to the example on [`Config::glob_matchoptions_hook`] for the
	/// usage of a hook with only the [`FnOnce`] trait.
	///
	pub fn add_glob_matchoptions_hook<GlobMatchOptionsHook>(
		mut self,
		glob_matchoptions_hook: GlobMatchOptionsHook)
		-> Self
		where GlobMatchOptionsHook: FnOnce(MatchOptions) -> MatchOptions + 'a
	{
		let glob_matchoptions_hook_fn = (self.glob_matchoptions_hook)
			.replace(Box::new(reflect_glob));
		self.glob_matchoptions_hook = RefCell::new(Box::new(move |match_options: MatchOptions| {
			glob_matchoptions_hook(glob_matchoptions_hook_fn(match_options))
		}));
		self
	}

	///
	/// Build the library and/or the
	/// [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding files as
	/// configured.
	///
	/// Also, emit the [`Cargo`](https://doc.rust-lang.org/cargo/) metadata to
	/// [`std::io::Stdout`].
	///
	/// `self` is consumed.
	///
	/// If you have to integrate any system libraries, do so after building the
	/// internal library by [`Config::build`]; the `-l` flags are added to the
	/// linker in the order reported to
	/// [`Cargo`](https://doc.rust-lang.org/cargo/), which means that the
	/// system libraries MUST be configured after the internal ones.
	///
	/// # Covered `Cargo` Metadata
	/// * The linkage to the generated library. ([`cc`])
	/// * The dependency on the external C header files. ([`bindgen`])
	/// * The dependency on the configured C source and header files.
	///   ([`kusabira`](crate))
	///
	/// # Panics
	/// * The unwrap of [`Result<T, E>`] fails because of a logical error.
	///
	/// # Errors
	/// * `self` is misconfigured.
	/// * Any of the backends ([`cc`], [`bindgen`] and [`glob`]) fails.
	///
	/// # Example
	/// Below is the build script excerpt of
	/// [`himetake`](../../himetake/index.html) version 0.1.0, modified
	/// slightly for the better understanding:
	/// ```no_run
	/// use kusabira::builder::Config as MldBuilderConfig;
	/// use kusabira::hooks::cc::warnings_into_errors;
	/// use system_deps::Config as SystemDepsConfig;
	///
	///	MldBuilderConfig::default()
	/// 	.lib_name("hello_world_c")
	/// 	.c_source_file("src/hello_world_export_to_rust.h")
	/// 	.add_c_source_files([
	/// 		"src/hello_world_c_*.c",
	/// 		"src/unixcw_libcw_demo.c"]
	/// 	.into_iter())
	/// 	.cc_build_hook(warnings_into_errors)
	/// 	.add_cc_build_hook(|build| {build.std("c11")})
	/// 	.bindgen_builder_hook(|builder| {builder.generate_block(true)})
	/// 	.add_bindgen_builder_hook(|builder| {builder.generate_comments(true)})
	/// 	.add_bindgen_builder_hook(|builder| {builder.newtype_enum("cw_return_values")})
	/// 	.build()
	/// 	.expect("the internal library build MUST succeed");
	///
	/// SystemDepsConfig::new()
	/// 	.probe()
	/// 	.expect("the system library integration MUST succeed");
	/// ```
	///
	/// [`system_deps::Config::probe`] after [`Config::build`] integrates the
	/// system libraries configured in the
	/// [`Cargo`](https://doc.rust-lang.org/cargo/) manifest so that they
	/// resolve the symbols required by the internal library.
	///
	// XXX the link to `himetake` is not via the Rust name in order to avoid
	// the cyclic package dependency.
	pub fn build(self)
		-> Result<BuildResults, MldError>
	{
		let mut results = BuildResults::new();
		let mut built_something = false;

		if !self.out_dir.is_dir() {
			return Err(
				MldError::from(
					format!("output directory {} MUST be created before calling Config::build",
						self.out_dir.to_str().expect("output directory path string MUST be valid"))));
		}
		results.out_dir = StdPathBuf::new();
		results.out_dir.push(self.out_dir.clone());

		let mut build = Build::default();
		let cc_build_hook_fn = (self.cc_build_hook)
			.replace(Box::new(reflect_cc));
		// Allow the override by the mock.
		build.out_dir::<&Path>(self.out_dir.as_ref());
		cc_build_hook_fn(&mut build);

		let glob_matchoptions = MatchOptions::new();
		let glob_matchoptions_hook_fn = (self.glob_matchoptions_hook)
			.replace(Box::new(reflect_glob));
		let glob_matchoptions = glob_matchoptions_hook_fn(glob_matchoptions);

		let mut c_source_found = false;

		for c_fn_glob in &self.c_source_files {
			for c_source_pathbuf in glob_with(c_fn_glob, glob_matchoptions)?
				.filter_map(Result::ok) {
				let c_filename = c_source_pathbuf
					.to_str()
					.expect("globbed path MUST make a valid string");

				let ext = FileExtension::from(c_source_pathbuf.as_path());
				match ext {
					FileExtension::C | FileExtension::H => {
						println!("cargo:rerun-if-changed={c_filename}");
					},
					FileExtension::Unsupported(_) => {
						eprintln!("Ignoring non-C-source file {c_filename}.");
					},
				}
				match ext {
					FileExtension::C => {
						c_source_found = true;
						build.file(c_source_pathbuf.as_path());
						results.c_source_files.push(c_source_pathbuf);
					},
					FileExtension::H => {
						let mut binding_pathbuf = self.out_dir
							.clone()
							.join(c_source_pathbuf.file_name()
								.expect("C source file PathBuf MUST make a valid string"));
						binding_pathbuf.set_extension("in");
						let builder = Builder::default()
							.header(c_filename)
							.parse_callbacks(Box::new(CargoCallbacks));
						let builder = (self.
							bindgen_builder_hook
							.borrow_mut())
							(builder);
						let bindings = builder.generate()?;
						bindings.write_to_file(&binding_pathbuf)?;
						built_something = true;
						results.c_header_rust_binding_files.push(
							HeaderBinding::new(c_source_pathbuf, binding_pathbuf));
					},
					FileExtension::Unsupported(_) => {},
				}
			}
		}

		if c_source_found {
			let lib_name = self.lib_name.ok_or_else(
				|| MldError::from(
					"library name MUST be configured when at least one C source is configured"))?;
			build.try_compile(lib_name)?;
			built_something = true;
			results.lib_name = Some(String::from(lib_name));
		}

		if !built_something {
			return Err(MldError::from("no source files configured"));
		}

		Ok(results)
	}
}

///
/// The pair of a C header file and the generated
/// [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding file,
/// held in [`BuildResults`].
///
#[derive(Debug)]
pub struct HeaderBinding
{
	/// The C header file.
	pub c_header_file: StdPathBuf,
	/// The [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding
	/// file.
	pub rust_binding_file: StdPathBuf,
}

impl HeaderBinding
{
	/// Create the new [`HeaderBinding`] data for the given C header and
	/// [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding file
	/// pair.
	fn new(c_header_file: StdPathBuf, rust_binding_file: StdPathBuf) -> HeaderBinding
	{
		HeaderBinding {
			c_header_file: c_header_file,
			rust_binding_file: rust_binding_file,
		}
	}
}

impl Display for HeaderBinding
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError>
	{
		write!(f,
			"({} -> {})",
			self.c_header_file.display(),
			self.rust_binding_file.display())
	}
}

///
/// The build results returned by [`Config::build`].
///
/// This structure supports [`std::process::Termination`] so that the main
/// function of the build script can be implemented as a one-liner that calls
/// [`Config::build`].
///
#[derive(Debug)]
pub struct BuildResults
{
	/// The output directory.
	pub out_dir: StdPathBuf,
	/// The library name, if generated.
	pub lib_name: Option<String>,
	/// The C source files.
	pub c_source_files: Vec<StdPathBuf>,
	/// The C header and generated
	/// [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding file pair.
	pub c_header_rust_binding_files: Vec<HeaderBinding>,
}

impl BuildResults
{
	/// Create the new [`HeaderBinding`] data.
	fn new() -> BuildResults
	{
		BuildResults {
			out_dir: StdPathBuf::from("."),
			lib_name: None,
			c_source_files: Vec::new(),
			c_header_rust_binding_files: Vec::new(),
		}
	}
}

impl Display for BuildResults
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError>
	{
		write!(f,
			"(out_dir: {}, lib_name: {}, c_source_files: {}, c_header_rust_binding_files: {})",
			self.out_dir.display(),
			(self.lib_name.as_ref()).unwrap_or(&("None".to_string())),
			str_iter_to_string(self.c_source_files.iter().map(|path_buf| {path_buf.display()})),
			str_iter_to_string(self.c_header_rust_binding_files.iter()))
	}
}

impl Termination for BuildResults
{
	/// Fixed to [`ExitCode::SUCCESS`]; the return of [`BuildResults`] means a success.
	fn report(self) -> ExitCode
	{
		ExitCode::SUCCESS
	}
}

/// Display each string in the iterator as the comma-separated elements wrapped
/// by the square brackets.
fn str_iter_to_string<T, IT>(iter: IT) -> String
	where T: ToString, IT: Iterator<Item = T>
{
	let mut string = String::new();

	string += "[";
	let mut first = true;
	for i in iter {
		string += &format!("{}", i.to_string());
		if first {
			string += ", ";
			first = false;
		}
	}
	string += "]";

	string
}

///
/// The C source file extensions.
///
#[derive(Debug, PartialEq)]
enum FileExtension
{
	/// A C source file. (`*.c`)
	C,
	/// A C header file. (`*.h`)
	H,
	/// An unsupported extension.
	Unsupported(String),
}

impl From<Option<&OsStr>> for FileExtension
{
	fn from(ext: Option<&OsStr>) -> Self
	{
		match ext {
			Some(ext_os) => {
				match ext_os.to_str() {
					Some(ext_str) => {
						if "c" == ext_str {
							FileExtension::C
						}
						else if "h" == ext_str {
							FileExtension::H
						}
						else {
							FileExtension::Unsupported(String::from(ext_str))
						}
					},
					None => FileExtension::Unsupported("".to_string())
				}
			},
			None => {
				FileExtension::Unsupported("".to_string())
			}
		}
	}
}

impl From<&Path> for FileExtension
{
	fn from(path: &Path) -> Self
	{
		FileExtension::from(path.extension())
	}
}

impl Display for FileExtension
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError>
	{
		match self {
			FileExtension::C => write!(f, "FileExtension: C"),
			FileExtension::H => write!(f, "FileExtension: H"),
			FileExtension::Unsupported(ext) => write!(f, "FileExtension: Unsupported({})", ext),
		}
		
	}
}
