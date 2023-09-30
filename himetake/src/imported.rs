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

//!
//! # Licensing of Included Items
//! The Rust items generated out of any third-party software by [`bindgen`]
//! and included into this module are under the license of each software from
//! which the included Rust items originate.
//!
//! Such the Rust items SHALL NOT be regarded as the Derivative Works of the
//! originating third-party software, unless expressed otherwise.  This
//! treatment aligns with the _Derivative Works_ definition in _1. Definitions_
//! of [Apache License Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).
//!
//! # Practice Guide
//! Contrary to the very limited number of the imported items from the C
//! sources truely in use, the nominal number of the items in module
//! [`crate::imported`] is very large.  A dedicated module is a good way to
//! control many unused items and avoid the namespace pollution.
//!

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(missing_docs)]

/// The wrappers to use the imported items safely in Rust.
pub mod wrapper;

// This `use` is required for the document to link to `bindgen`.
#[cfg(doc)]
use bindgen::builder;

include!(concat!(env!("OUT_DIR"), "/hello_world_export_to_rust.in"));
