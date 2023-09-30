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
//! This module publishes some common and intrinsic hooks for
//! [`cc::Build`].
//!

#![deny(missing_docs)]

#[cfg(not(test))]
use cc::Build;
#[cfg(test)]
use super::super::builder::tests::busshi::cc_build::Build;

///
/// Reflect the input, ie return the configuration as is.
///
/// This is the default [`cc::Build`] configuration hook.
///
/// # Example
/// ```
/// use cc::Build;
/// use kusabira::hooks::cc::reflect;
///
/// let mut before = Build::default();
/// let after = reflect(&mut before);
/// ```
///
/// ```
/// use kusabira::builder::Config;
/// use kusabira::hooks::cc::reflect;
///
/// let config = Config::default()
/// 	.add_cc_build_hook(reflect);
/// ```
///
pub fn reflect(build: &mut Build) -> &mut Build
{
	build
}

///
/// Enable all and extra warnings, and treat any warnings as errors.
///
/// # Example
/// ```
/// use cc::Build;
/// use kusabira::hooks::cc::warnings_into_errors;
///
/// let mut before = Build::default();
/// let after = warnings_into_errors(&mut before);
/// ```
///
/// ```
/// use kusabira::builder::Config;
/// use kusabira::hooks::cc::warnings_into_errors;
///
/// let config = Config::default()
/// 	.add_cc_build_hook(warnings_into_errors);
/// ```
///
pub fn warnings_into_errors(build: &mut Build) -> &mut Build
{
	build.warnings(true)
		.extra_warnings(true)
		.warnings_into_errors(true)
}
