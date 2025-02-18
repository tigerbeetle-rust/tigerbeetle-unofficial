`tigerbeetle-unofficial`
========================

[![crates.io](https://img.shields.io/crates/v/tigerbeetle-unofficial.svg "crates.io")](https://crates.io/crates/tigerbeetle-unofficial)
[![Rust 1.78+](https://img.shields.io/badge/rustc-1.78+-lightgray.svg "Rust 1.78+")](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html)  
[![CI](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/actions/workflows/ci.yml/badge.svg?branch=main "CI")](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/actions?query=workflow%3ACI+branch%3Amain)
[![Rust docs](https://docs.rs/tigerbeetle-unofficial/badge.svg "Rust docs")](https://docs.rs/tigerbeetle-unofficial)

[Changelog](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/blob/v0.8.0%2B0.16.28/CHANGELOG.md)

Unofficial [TigerBeetle] bindings for [Rust].




## Status

Because this [TigerBeetle] client library implementation is not a part of the [official `tigerbeetle` repos][1], it is hard to ensure and keep some of [Rust] safety guarantees from the outside. For that reason I invite people to contribute to this repo or finally develop the official [Rust] client library.




## Repo Overview

The repository hosts the following libraries:

 * [![Crates.io](https://img.shields.io/crates/v/tigerbeetle-unofficial.svg?label=tigerbeetle-unofficial)](https://crates.io/crates/tigerbeetle-unofficial)
   [![docs.rs](https://docs.rs/tigerbeetle-unofficial/badge.svg)](https://docs.rs/tigerbeetle-unofficial) - Safe high-level async bindings. Implemented with `#![forbid(unsafe_code)]` upon `tigerbeetle-unofficial-core`.
 * [![Crates.io](https://img.shields.io/crates/v/tigerbeetle-unofficial-core.svg?label=tigerbeetle-unofficial-core)](https://crates.io/crates/tigerbeetle-unofficial-core)
   [![docs.rs](https://docs.rs/tigerbeetle-unofficial-core/badge.svg)](https://docs.rs/tigerbeetle-unofficial-core) - Safe low-level callback-based async bindings.
 * [![Crates.io](https://img.shields.io/crates/v/tigerbeetle-unofficial-sys.svg?label=tigerbeetle-unofficial-sys)](https://crates.io/crates/tigerbeetle-unofficial-sys)
   [![docs.rs](https://docs.rs/tigerbeetle-unofficial-sys/badge.svg)](https://docs.rs/tigerbeetle-unofficial-sys) - Unsafe native bindings.





## License

[TigerBeetle] is licensed under [Apache License, Version 2.0](https://github.com/tigerbeetle/tigerbeetle/blob/0.16.28/LICENSE).

`tigerbeetle-unofficial` crates are licensed under the [Apache License, Version 2.0](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/blob/v0.8.0%2B0.16.28/LICENSE) (the "License"); you may not use these files except in compliance with the License. You may obtain a copy of the License at

https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.




[Rust]: https://www.rust-lang.org
[TigerBeetle]: https://tigerbeetle.com
[1]: https://github.com/tigerbeetle
