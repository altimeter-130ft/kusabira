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
//! # Introduction
//! [`himetake`](crate) is the demo crate of [`kusabira`].
//!
//! This crate is separate from [`kusabira`] because it is meant to be
//! used by the build script of [`Cargo`](https://doc.rust-lang.org/cargo/),
//! which cannot depend on its library crate by design.
//!
//! # Terminology
//! The key words "IETF _XXX_" in this document set are to be interpreted as
//! the documents published by [IETF](https://www.ietf.org) when, and only
//! when, they appear in all capitals except for the _-s_ suffix denoting
//! plural, as shown here.  This key word usage distinguishes
//! [the IETF RFCs](https://www.ietf.org/standards/rfcs/) from
//! [the Rust RFCs](https://github.com/rust-lang/rfcs).
//!
//! The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
//! NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED",
//! "MAY", and "OPTIONAL" in this document set are to be interpreted as
//! described in `IETF BCP 14`
//! ([`IETF RFC2119`](https://datatracker.ietf.org/doc/html/rfc2119),
//! [`IETF RFC8174`](https://datatracker.ietf.org/doc/html/rfc8174))
//! when, and only when, they appear in all capitals, as shown here.
//!
//! # License
//! The terms "Licensor", "You", "Your", "Work", "Contribution" and
//! "Contributor" in the sections of this document set regarding licenseng are
//! to be interpreted as defined in _1. Definitions_ of
//! [Apache License Version 2.0][Apache-2.0] when, and only when, they appear
//! [capitalized](https://en.wikipedia.org/wiki/Capitalization), as shown here,
//! as well as with any inflection.
//!
//! This software is licensed under either or both of:
//!
//! * [Apache License Version 2.0][Apache-2.0]
//! * [MIT License][MIT]
//!
//! at Your option.
//!
//! [Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0 "Apache License Version 2.0"
//! [MIT]: https://choosealicense.com/licenses/mit/ "MIT License"
//!
//! ## Practice of _5. Submission of Contributions, Apache License Version 2.0_
//! Unless You explicitly state otherwise, any Contribution intentionally
//! submitted for inclusion in the Work by You to the Licensor shall be under
//! the terms and conditions of [Apache License Version 2.0][Apache-2.0] and
//! [MIT License][MIT], without any additional terms or conditions.
//!
//! # Informational Definitions
//! ## Himetake (姫菌), _noun_, _Japanese_
//!    Refer to _[Kusabira](kusabira#kusabira-菌-noun-japanese)_.
//!

#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]

/// The items imported via [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html).
pub mod imported;
/// The demo module 1.
pub mod hello_world_rust_1;
/// The demo module 2.
pub mod hello_world_rust_2;
/// The demo module for [`unixcw`](https://sourceforge.net/projects/unixcw/).
pub mod unixcw_libcw_demo;

use std::ffi::CString;

use imported::hello_world_c_1_fn;
use imported::hello_world_c_2_fn;
use unixcw_libcw_demo::unixcw_libcw_demo_1_rust_fn;

// This `use` is required for the document to link to `kusabira`.
#[cfg(doc)]
use kusabira::KusabiraError;

///
/// The entry point of the demo.
///
/// First, call [`imported::hello_world_c_1_fn`] with a string argument, which
/// is added to the printed message.
///
/// Then, call [`imported::hello_world_c_2_fn`] with a callback function
/// ([`hello_world_rust_2::hello_world_rust_2_callback`]), which is then called
/// in [`imported::hello_world_c_2_fn`].
///
/// Finally, call [`unixcw_libcw_demo::unixcw_libcw_demo_1_rust_fn`] to
/// demonstrate [`unixcw`](https://sourceforge.net/projects/unixcw/).
///
pub fn do_demo()
{
	println!("\n==> Demo 1.");

	let msg = String::from("from Rust");
	let msg_cstr = CString::new(msg).unwrap();

	let ret = unsafe {
		hello_world_c_1_fn(msg_cstr.as_ptr())
	};

	println!("hello_world_c_1_fn: ret = {ret}.");

	println!("\n==> Demo 2.");

	let callback = hello_world_rust_2::hello_world_rust_2_callback;
	let ret2 = unsafe {
		hello_world_c_2_fn(Some(callback))
	};

	println!("hello_world_c_2_fn: ret2 = {ret2}.");

	println!("\n==> Demo 3.");

	unixcw_libcw_demo_1_rust_fn();
}

#[cfg(test)]
mod tests
{

use super::*;

#[test]
fn test_do_demo()
{
	do_demo();
}

}
