# Tonic-Deploy 

Perf Testing Tonic.

[Tonic](https://github.com/hyperium/tonic) 

## Overview

A normal workflow for grpc/tonic deployments: 
    1) compile the protobuf files,  
    2) build the source code,
    3) deploy the application components to 1 or more hosts 

The sub-directories here contain samples supporting that process. 

## Rustwide-Build 

[Rustwide](https://github.com/rust-lang/rustwide) is a baseline environment for building Rust crates/projects. It is used by the 
rust-doc project as well as Crater, a testing framework for the Rust crate eco-system.  

Using rustwide for building let's us:

1) Maintain a consistent build environment (a docker image),
2) Easily switch out the Rust toolchain for testing,
3) Easily configure different sources for building (local/github repository)
4) Document a build process

There's some overlap with what `cargo build` and a typical CI/CD process do, so, if used, it needs to be crafted to fit your specific workflow.  One useful scenario: spin up the sandboxed-tonic environment with constraints (e.g. memory) on the container to faciliate performance testing.

## Docker-Compose 

WIP

## Project layout

The `tokio` crate, found at the root, is primarily intended for use by
application developers.  Library authors should depend on the sub crates, which
have greater guarantees of stability.

The crates included as part of Tokio are:

* [`tokio-executor`]: Task executors and related utilities. Includes a
  single-threaded executor and a multi-threaded, work-stealing, executor.

* [`tokio-fs`]: Filesystem (and standard in / out) APIs.

* [`tokio-codec`]: Utilities for encoding and decoding protocol frames.

* [`tokio-io`]: Asynchronous I/O related traits and utilities.

* [`tokio-macros`]: Macros for usage with Tokio.

* [`tokio-net`]: Event loop that drives I/O resources as well as TCP, UDP, and
  unix domain socket apis.

* [ `tokio-timer`]: Time related APIs.

[`tokio-codec`]: tokio-codec
[`tokio-current-thread`]: tokio-current-thread
[`tokio-executor`]: tokio-executor
[`tokio-fs`]: tokio-fs
[`tokio-io`]: tokio-io
[`tokio-macros`]: tokio-macros
[`tokio-net`]: tokio-net
[`tokio-timer`]: tokio-timer

## Related Projects

In addition to the crates in this repository, the Tokio project also maintains
several other libraries, including:

* [`tracing`] (formerly `tokio-trace`): A framework for application-level
  tracing and async-aware diagnostics.

* [`mio`]: A low-level, cross-platform abstraction over OS I/O APIs that powers
  `tokio`.

* [`bytes`]: Utilities for working with bytes, including efficient byte buffers.

[`tracing`]: https://github.com/tokio-rs/tracing
[`mio`]: https://github.com/tokio-rs/mio
[`bytes`]: https://github.com/tokio-rs/bytes

## Supported Rust Versions

Tokio is built against the latest stable, nightly, and beta Rust releases. The
minimum version supported is the stable release from three months before the
current stable release version. For example, if the latest stable Rust is 1.29,
the minimum version supported is 1.26. The current Tokio version is not
guaranteed to build on Rust versions earlier than the minimum supported version.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you, shall be licensed as MIT, without any additional
terms or conditions.

