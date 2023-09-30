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
//! The module for the mocks used in the unit tests.
//!
//! # Informational Definitions
//! ## Busshi (仏師), _noun_, _Japanese_
//! 1. An artist of the Budda statue.
//! 2. A play of [Kyogen(狂言)](https://en.wikipedia.org/wiki/Ky%C5%8Dgen) with
//!    the following plot:
//!
//!    > A mischievous man, disguising as a _busshi_, receives the order of a
//!    > brand-new Budda statue from an innocent customer.  Having no
//!    > skills to carve at all, the _busshi_ attempts to disguise himself
//!    > as the statue.  The customer then comes to the _busshi_ to pick up the
//!    > ordered statue.  The _busshi_ receives the customer and presents the
//!    > disguised statue in turn, until the _busshi_ misses the disguise in
//!    > haste and the customer drives him away.
//!

pub mod bindgen_builder;
pub mod cc_build;
pub mod glob;
pub mod std_path_path_buf;
