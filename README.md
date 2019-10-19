[![pipeline](https://gitlab.com/d-e-s-o/waitfor/badges/master/pipeline.svg)](https://gitlab.com/d-e-s-o/waitfor/commits/master)
[![coverage](https://gitlab.com/d-e-s-o/waitfor/badges/master/coverage.svg)](https://gitlab.com/d-e-s-o/waitfor/commits/master)
[![crates.io](https://img.shields.io/crates/v/waitfor.svg)](https://crates.io/crates/waitfor)
[![docs](https://docs.rs/waitfor/badge.svg)](https://docs.rs/waitfor)
[![rustc](https://img.shields.io/badge/rustc-1.31+-blue.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

waitfor
=======

- [Changelog](CHANGELOG.md)

**waitfor** is a crate allowing for retry of operations until a timeout
or deadline is reached. If operation failed this failure is bubbled up
directly.
