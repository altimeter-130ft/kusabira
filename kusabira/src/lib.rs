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
//! # Introduction
//! [`kusabira`](crate) is the integrated C/C++/assembly source build frontend
//! for the [`Cargo`](https://doc.rust-lang.org/cargo/)
//! [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
//!
//! # Features
//! * Integrated C/C++/assembly building by [`cc`] and
//!   [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding generation
//!   by [`bindgen`] as the backends.
//! * Multiple source and header files with the glob support by [`glob`].
//! * Single-line configuration and build execution.
//! * Cooperation with [`system_deps`] to integrate the system libraries.
//! * [`Cargo`](https://doc.rust-lang.org/cargo/) metadata output to
//!   [`std::io::Stdout`].
//! * Highly flexible backend configuration via the hooks.
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
//! This terminology also applies to the error messages reported by
//! [`kusabira`](crate).
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
//! ## Kusabira (菌), _noun_, _Japanese_
//! 1. An anchient word for fungus or its mushroom.
//! 2. A play of [Kyogen(狂言)](https://en.wikipedia.org/wiki/Ky%C5%8Dgen) with
//!    the following plot:
//!
//!    > A garden owner, tired of the fungi growing up after and after in the
//!    > garden and messing up the scenary, asks a priest for the prayer to
//!    > drive them away.  The priest accepts the request and sets on the work.
//!    > After a seeming success, the fungi grow up even worse and get the
//!    > priest into a trouble.  Finally, _himetake(姫菌)_, the queen fungi,
//!    > appears and blows up her big mushroom, driving away the garden owner
//!    > and priest.
//!
//!    [The performance by the Sengoroh Shigeyama family(茂山千五郎家) on Youtube](https://www.youtube.com/watch?v=o1T-r_agzow&t=5081s).
//!

#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]

/// The build frontend.
pub mod builder;
/// The error data.
pub mod error;
/// The ready-to-go hooks.
pub mod hooks;

pub use error::Error as KusabiraError;

// This `use` is required for the document to link to
// `system_deps`.
#[cfg(doc)]
use system_deps::Config as SystemDepsConfig;
