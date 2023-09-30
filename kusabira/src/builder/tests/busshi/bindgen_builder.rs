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

use bindgen::{Builder as BindgenBuilder, BindgenError, CargoCallbacks};
use bindgen::callbacks::ParseCallbacks;
use std::boxed::Box;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::convert::{AsRef, Into};
use std::default::Default;
use std::io::{Error as StdIoError, ErrorKind as StdIoErrorKind, Result as StdIoResult};
use std::path::{Path, PathBuf};

pub trait BindgenBuilderContext
where Self: Default
{
	fn emulate_generate_error_set(&mut self, emulate_generate_error: bool) -> &mut Self;
	fn emulate_generate_error_get(&self) -> bool;
	fn emulate_write_error_set(&mut self, emulate_generate_error: bool) -> &mut Self;
	fn emulate_write_error_get(&self) -> bool;
}

#[derive(Debug)]
struct BindgenBuilderContextTLS
{
	emulate_generate_error: bool,
	emulate_write_error: bool,
}

impl Default for BindgenBuilderContextTLS
{
	fn default() -> Self
	{
		BindgenBuilderContextTLS {
			emulate_generate_error: false,
			emulate_write_error: false,
		}
	}
}

impl BindgenBuilderContext for BindgenBuilderContextTLS
{
	fn emulate_generate_error_set(&mut self, emulate_generate_error: bool) -> &mut Self
	{
		self.emulate_generate_error = emulate_generate_error;
		self
	}

	fn emulate_generate_error_get(&self) -> bool
	{
		self.emulate_generate_error
	}

	fn emulate_write_error_set(&mut self, emulate_write_error: bool) -> &mut Self
	{
		self.emulate_write_error = emulate_write_error;
		self
	}

	fn emulate_write_error_get(&self) -> bool
	{
		self.emulate_write_error
	}
}

thread_local!
{
	static BINDGEN_BUILDER_CONTEXT_TLS: RefCell<BindgenBuilderContextTLS> =
		RefCell::new(BindgenBuilderContextTLS::default());
}

#[derive(Debug)]
pub struct BindgenBuilderContextAccess
{
}

impl Default for BindgenBuilderContextAccess
{
	fn default() -> Self
	{
		BindgenBuilderContextAccess {}
	}
}

impl BindgenBuilderContext for BindgenBuilderContextAccess
{
	fn emulate_generate_error_set(&mut self, emulate_generate_error: bool) -> &mut Self
	{
		BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
		{
			ctx.borrow_mut().emulate_generate_error_set(emulate_generate_error);
		});
		self
	}

	fn emulate_generate_error_get(&self) -> bool
	{
		BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
		{
			ctx.borrow().emulate_generate_error_get()
		})
	}

	fn emulate_write_error_set(&mut self, emulate_write_error: bool) -> &mut Self
	{
		BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
		{
			ctx.borrow_mut().emulate_write_error_set(emulate_write_error);
		});
		self
	}

	fn emulate_write_error_get(&self) -> bool
	{
		BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
		{
			ctx.borrow().emulate_write_error_get()
		})
	}
}

#[derive(Debug)]
pub struct Builder
{
	builder: BindgenBuilder,
	header: Option<String>,
	parse_callbacks: VecDeque<Box<dyn ParseCallbacks>>,
	generate_block: bool,
	generate_comments: bool,
}

#[derive(Debug)]
pub struct Bindings
{
	pub builder: Builder,
	write_to_file: RefCell<PathBuf>,
}

impl Builder
{
	pub fn header<T: Into<String>>(mut self, header: T) -> Builder
	{
		let header_str = header.into();

		self.builder = self.builder.header(header_str.clone());
		self.header = Some(header_str);
		self
	}

	pub fn parse_callbacks(mut self, cb: Box<dyn ParseCallbacks>) -> Self
	{
		self.parse_callbacks.push_back(cb);
		// cb is not passed to builder now because it does not support the
		// clone trait.
		self
	}

	pub fn generate_block(mut self, doit: bool) -> Self
	{
		self.builder = self.builder.generate_block(doit);
		self.generate_block = doit;
		self
	}

	pub fn generate_comments(mut self, doit: bool) -> Self
	{
		self.builder = self.builder.generate_comments(doit);
		self.generate_comments = doit;
		self
	}

	pub fn generate(mut self) -> Result<Bindings, BindgenError>
	{
		while let Some(cb) = self.parse_callbacks.pop_front() {
			self.builder = self.builder.parse_callbacks(cb);
		}

		let bindgen_builder_ctx = BindgenBuilderContextAccess::default();
		let emulate_error = bindgen_builder_ctx.emulate_generate_error_get();

		if emulate_error {
			Err(BindgenError::ClangDiagnostic("emulated by mock".to_string()))
		} else {
			Ok(Bindings::new(self))
		}
	}
}

impl Default for Builder
{
	fn default() -> Self
	{
		Builder {
			builder: BindgenBuilder::default(),
			header: None,
			parse_callbacks: VecDeque::new(),
			generate_block: false,
			generate_comments: false,
		}
	}
}

impl Bindings
{
	fn new(builder: Builder) -> Self
	{
		Bindings {
			builder: builder,
			write_to_file: RefCell::new(PathBuf::new()),
		}
	}

	pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> StdIoResult<()>
	{
		self.write_to_file.borrow_mut().push(path);

		let bindgen_builder_ctx = BindgenBuilderContextAccess::default();
		let emulate_error = bindgen_builder_ctx.emulate_write_error_get();

		if emulate_error {
			Err(StdIoError::new(StdIoErrorKind::Other, "emulated by mock"))
		} else {
			Ok(())
		}
	}
}

mod tests {

use super::*;

#[test]
fn test_struct_bindgen_builder_context()
{
	let mut bindgen_builder_ctx = BindgenBuilderContextAccess::default();
	BINDGEN_BUILDER_CONTEXT_TLS.with( |ctx|
	{
		assert_eq!(ctx.borrow().emulate_generate_error, false);
		assert_eq!(ctx.borrow().emulate_write_error, false);
	});
	assert_eq!(bindgen_builder_ctx.emulate_generate_error_get(), false);
	assert_eq!(bindgen_builder_ctx.emulate_write_error_get(), false);

	bindgen_builder_ctx.emulate_generate_error_set(true);
	BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().emulate_generate_error, true);
		assert_eq!(ctx.borrow().emulate_write_error, false);
	});
	assert_eq!(bindgen_builder_ctx.emulate_generate_error_get(), true);
	assert_eq!(bindgen_builder_ctx.emulate_write_error_get(), false);

	bindgen_builder_ctx.emulate_write_error_set(true);
	BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().emulate_generate_error, true);
		assert_eq!(ctx.borrow().emulate_write_error, true);
	});
	assert_eq!(bindgen_builder_ctx.emulate_generate_error_get(), true);
	assert_eq!(bindgen_builder_ctx.emulate_write_error_get(), true);

	bindgen_builder_ctx.emulate_generate_error_set(false);
	BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().emulate_generate_error, false);
		assert_eq!(ctx.borrow().emulate_write_error, true);
	});
	assert_eq!(bindgen_builder_ctx.emulate_generate_error_get(), false);
	assert_eq!(bindgen_builder_ctx.emulate_write_error_get(), true);

	bindgen_builder_ctx.emulate_write_error_set(false);
	BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
	{
		assert_eq!(ctx.borrow().emulate_generate_error, false);
		assert_eq!(ctx.borrow().emulate_write_error, false);
	});
	assert_eq!(bindgen_builder_ctx.emulate_generate_error_get(), false);
	assert_eq!(bindgen_builder_ctx.emulate_write_error_get(), false);

	BINDGEN_BUILDER_CONTEXT_TLS.with(|ctx|
	{
		println!("bindgen_builder_ctx (TLS) = {:?}.", ctx.borrow());
	});
	println!("bindgen_builder_ctx (Access) = {:?}.", bindgen_builder_ctx);
}

#[test]
fn test_struct_builder()
{
	let mut builder = Builder::default();
	assert!(builder.header.is_none());

	let header_filename = "hello_world_c_exported_to_rush.h";
	builder = builder.header(String::from(header_filename));
	assert_eq!(builder.header, Some(String::from(header_filename)));

	assert_eq!(builder.parse_callbacks.len(), 0);
	builder = builder.parse_callbacks(Box::new(CargoCallbacks));
	assert_eq!(builder.parse_callbacks.len(), 1);

	println!("builder = {:?}.", builder);

	let bindings = builder.generate().unwrap();
	let mut binding_pathbuf = PathBuf::new();
	binding_pathbuf.push(header_filename);
	binding_pathbuf.set_extension("in");
	bindings.write_to_file(&binding_pathbuf)
		.expect("bindings.write_to_file MUST succeed");
	assert_eq!(*(bindings.write_to_file.borrow()), binding_pathbuf);

	println!("bindings = {:?}.", bindings);
}
}
