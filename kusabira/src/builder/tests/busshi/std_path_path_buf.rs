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

use std::cmp::PartialEq;
use std::convert::{AsRef, From};
use std::iter::{FromIterator, IntoIterator};
use std::path::{Path as StdPath, PathBuf as StdPathBuf};

#[derive(Clone, Debug)]
pub struct PathBuf
{
	path_buf: StdPathBuf,
	pub is_dir: bool,
}

impl PathBuf 
{
	pub fn to_str<'a>(&'a self) -> Option<&'a str>
	{
		self.path_buf.to_str()
	}

	pub fn clear(&mut self)
	{
		self.path_buf.clear()
	}

	pub fn is_dir(&self) -> bool
	{
		self.is_dir
	}

	pub fn join<P: AsRef<StdPath>>(&self, path: P) -> StdPathBuf
	{
		self.path_buf.join(path)
	}

	pub fn push<P>(&mut self, path: P)
	where P: AsRef<StdPath>
	{
		self.path_buf.push(path)
	}
}

impl From<String> for PathBuf
{
	fn from(path: String) -> Self
	{
		PathBuf {
			path_buf: StdPathBuf::from(path),
			is_dir: true,
		}
	}
}

impl AsRef<StdPath> for PathBuf
{
	fn as_ref(&self) -> &StdPath
	{
		self.path_buf.as_ref()
	}
}

impl<P> FromIterator<P> for PathBuf
where P: AsRef<StdPath>
{
	fn from_iter<T>(iter: T) -> Self
	where T: IntoIterator<Item = P>
	{
		PathBuf {
			path_buf: StdPathBuf::from_iter(iter),
			is_dir: true,
		}
	}
}

impl PartialEq for PathBuf
{
	fn eq(&self, other: &Self) -> bool
	{
		self.path_buf == other.path_buf
	}
}

mod tests {

use super::*;

#[test]
fn test_struct_path_buf()
{
	let path_str = "test_dir";
	let path_str_2 = "test_dir";

	let mut path_buf = PathBuf::from(String::from(path_str));
	assert_eq!(path_buf.to_str(), Some(path_str));

	path_buf.is_dir = false;
	assert_eq!(path_buf.is_dir(), false);
	path_buf.is_dir = true;
	assert_eq!(path_buf.is_dir(), true);

	let _ref_std_path: &StdPath = path_buf.as_ref();

	let _joined_std_path = path_buf.join(path_str_2);

	println!("path_buf = {:?}.", path_buf);

	path_buf.clear();
}
}
