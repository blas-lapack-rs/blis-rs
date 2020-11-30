# BLIS Rust bindings

This repository holds crates to call BLIS, the BLAS-like Library Instantiation Framework, from Rust.

* `blis-src` builds and links BLIS with (Fortran) BLAS and/or CBLAS interfaces.
  It can be used via [blas-sys](https://lib.rs/crates/blas-sys) or
  [cblas-sys](https://lib.rs/crates/cblas-sys), or one can use `extern "C"` to
  call the full BLIS API.

# Roadmap

* Add `blis-sys` bindings to use the more flexible API provided on top of which
  BLAS is implemented in BLIS.
* Add `blis` for a safe, Rust-y wrapping on top of BLIS regular and/or
  object-oriented API.

# License

## Apache 2.0/MIT

All original work licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
     at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
