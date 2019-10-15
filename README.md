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
