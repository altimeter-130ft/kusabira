# Introduction
This is the [`Cargo`](https://doc.rust-lang.org/cargo/) workspace of the
following [Rust](https://www.rust-lang.org) crates:

* [`kusabira`](kusabira/)
  The integrated C/C++/assembly source build frontend for the
  [`Cargo`](https://doc.rust-lang.org/cargo/)
  [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
* [`himetake`](himetake/)
  The demo crate of [`kusabira`](kusabira/).

# Features of `kusabira`
* Integrated C/C++/assembly building by
  [`cc`](https://github.com/rust-lang/cc-rs) and
  [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) binding generation
  by [`bindgen`](https://rust-lang.github.io/rust-bindgen/) as the backends.
* Multiple source and header files with the glob support by
  [`glob`](https://github.com/rust-lang/glob).
* Single-line configuration and build execution.
* Cooperation with [`system_deps`](https://github.com/gdesmott/system-deps)
  to integrate the system libraries.
* [`Cargo`](https://doc.rust-lang.org/cargo/) metadata output to
  [`std::io::Stdout`](https://doc.rust-lang.org/std/io/struct.Stdout.html).
* Highly flexible backend configuration via the hooks.

# License
This software is licensed under either or both of:

* [Apache License Version 2.0][Apache-2.0]
* [MIT License][MIT]

at Your option.

[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0 "Apache License Version 2.0"
[MIT]: https://choosealicense.com/licenses/mit/ "MIT License"

## Practice of _5. Submission of Contributions, Apache License Version 2.0_
Unless You explicitly state otherwise, any Contribution intentionally
submitted for inclusion in the Work by You to the Licensor shall be under
the terms and conditions of [Apache License Version 2.0][Apache-2.0] and
[MIT License][MIT], without any additional terms or conditions.
