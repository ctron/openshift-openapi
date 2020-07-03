# OpenShift API for Rust

![CI](https://github.com/ctron/openshift-openapi/workflows/CI/badge.svg)
[![Crates.io version shield](https://img.shields.io/crates/v/openshift-openapi.svg)](https://crates.io/crates/openshift-openapi)
[![Docs](https://docs.rs/openshift-openapi/badge.svg)](https://docs.rs/openshift-openapi)
[![Crates.io license shield](https://img.shields.io/crates/l/openshift-openapi.svg)](https://crates.io/crates/openshift-openapi)

This crate is a Rust OpenShift API client. It contains bindings for the
resources and operations in the OpenShift client API,
auto-generated from the OpenAPI spec.

This work is based on the wonderful work from https://github.com/Arnavion/k8s-openapi. It does not contain
the Kubernetes APIs, that is handled by the `k8s_openapi` crate. It only contains the types added by OpenShift
(like `Route` and `ImageStream`) and references the existing Kubernetes (like `Pod` and `Deployment`) from the
`k8s_openapi` crate.

**Note**: This currently uses a patched version `k8s-openapi`. A few changes to the
code generator had been necessary. This is currently being tracked in PR https://github.com/Arnavion/k8s-openapi/pull/68.
Use at your own risk!
