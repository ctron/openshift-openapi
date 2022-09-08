#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cognitive_complexity,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::large_enum_variant,
    clippy::match_single_binding,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::use_self
)]

//! Bindings for the OpenShift client API, generated from the OpenAPI spec.
//!
//! Each supported version of OpenShift is represented by a feature name (like `v4_4`). Only one such feature can be enabled at a time.
//! As this crate depends on the other Kubernetes APIs, it references the `k8s_openapi` crate for
//! these types. Selecting a supported version for OpenShift (like `v4_4`) will automatically
//! select the corresponding Kubernetes feature on the `k8s_openapi` crate. So for OpenShift
//! `v4_4` it will automatically enable the feature `k8s_openapi/v1_17`.
//!
//! These docs have been generated with the `

#![cfg_attr(feature = "v4_4", doc = "v4_4")]

//! ` feature enabled. To see docs for one of the other supported versions, please generate the docs locally with `cargo doc --features 'v4_<>'`
//!
//!
//! # Examples
//!
//! ## Resources
//!
//! This example creates an instance of [`api::core::v1::PodSpec`] with no other properties set, and pretty-prints it.
//!
//! ```rust
//! use k8s_openapi::api::core::v1 as api;
//!
//! fn main() {
//!     let pod_spec: api::PodSpec = Default::default();
//!     println!("{:#?}", pod_spec);
//! }
//! ```
//!
//! ## Client API
//!
//! (This requires the `api` feature to be enabled. The feature is enabled by default. See ["Crate features"](#crate-features) below for more details.)
//!
//! This example executes the [`api::core::v1::Pod::list_namespaced_pod`] API operation to list all pods inside a namespace.
//! It demonstrates the common patterns implemented by all API operation functions in this crate:
//!
//! 1. The API function has required parameters and optional parameters. All optional parameters are taken as a single struct with optional fields.
//!
//!    Specifically for the [`api::core::v1::Pod::list_namespaced_pod`] operation, the `namespace` parameter is required and taken by the function itself,
//!    while other optional parameters like `field_selector` are fields of the [`ListOptional`] struct. An instance of
//!    this struct is taken as the last parameter of `Pod::list_namespaced_pod`. This struct impls [`Default`] so that you can just pass in `Default::default()`
//!    if you don't want to specify values for any of the optional parameters.
//!
//!    Some API operations have a single common type for optional parameters:
//!
//!    - All create API take optional parameters using the [`CreateOptional`] struct.
//!    - All delete API take optional parameters using the [`DeleteOptional`] struct.
//!    - All list API take optional parameters using the [`ListOptional`] struct.
//!    - All patch API take optional parameters using the [`PatchOptional`] struct.
//!    - All replace API take optional parameters using the [`ReplaceOptional`] struct.
//!    - All watch API take optional parameters using the [`WatchOptional`] struct.
//!    - All delete-collection API take optional parameters using the [`DeleteOptional`] struct for delete options and the [`ListOptional`] struct for list options.
//!
//!    Other API functions have their own `Optional` structs with fields corresponding to the specific parameters for those functions,
//!    such as [`api::core::v1::ReadNamespacedPodOptional`] for [`api::core::v1::Pod::read_namespaced_pod`]
//!
//! 1. The function returns an [`http::Request`] value with the URL path, query string, and request body filled out according to the parameters
//!    given to the function. The function does *not* execute this request. You can execute this `http::Request` using any HTTP client library you want to use.
//!    It does not matter whether you use a synchronous client like `reqwest`, or an asynchronous client like `hyper`, or a mock client that returns bytes
//!    read from a test file.
//!
//! 1. For each API operation function, there is a corresponding response type. For `Pod::list_namespaced_pod` this is [`ListResponse`]`<`[`api::core::v1::Pod`]`>`.
//!    This is an enum with variants for each of the possible HTTP status codes that the operation can return, and contains the data that the API server would
//!    return corresponding to that status code. For example, the list-namespaced-pod operation returns a pod list with HTTP 200 OK, so one of the variants of
//!    that type is `Ok(`[`List`]`<`[`api::core::v1::Pod`]`>)`
//!
//! 1. The response types impl the [`Response`] trait, which contains a single [`Response::try_from_parts`] function. This function takes an [`http::StatusCode`]
//!    and a `&u8` byte buffer, and tries to parse the byte buffer as the response type. For example, if you executed the request and received an HTTP 200 OK response
//!    with some bytes, you could call `<ListResponse<Pod> as Response>::try_from_parts(status_code, buf)` and expect to get
//!    `Ok(ListResponse::<Pod>::Ok(pod_list))` from it.
//!
//!    Once again, this design ensures that the crate is not tied to a specific HTTP client library or interface. It does not matter how you execute the HTTP request,
//!    nor whether your library is synchronous or asynchronous, since every HTTP client library gives you a way to get the HTTP response status code and the bytes
//!    of the response body.
//!
//! 1. The API operation function also returns another value next to the `http::Request`. This value is a function that takes an [`http::StatusCode`] and returns
//!    a [`ResponseBody`]`<ListResponse<Pod>>`. As mentioned above, `Response::try_from_parts` requires you to maintain a byte buffer for the response body.
//!    `ResponseBody` is a helper that maintains such a buffer internally. It provides an `append_slice()` function to append slices to this internal buffer,
//!    and a `parse()` function to parse the buffer as the expected type (`ListResponse<Pod>` in this case).
//!
//!    It is not *necessary* to use the `ResponseBody` returned by the API operation function to parse the response. The `ResponseBody::parse` function is
//!    only a wrapper around the underlying `Response::try_from_parts` function, and handles growing and shrinking its inner buffer as necessary. It also
//!    helps ensure that the response body is parsed as the *correct* type for the operation, `ListResponse<Pod>` in this case, and not some other type.
//!    However, you can instead use your own byte buffer instead of the `ResponseBody` value and call `ListResponse<Pod>::try_from_parts` yourself.
//!
//! 1. The response types are enums with variants corresponding to HTTP status codes. For example, the `ListResponse<Pod>::Ok` variant corresponds to the
//!    HTTP 200 response of the list-namespaced-pod API.
//!
//!    Each response enum also has an `Other` variant, that is yielded when the response status code does not match any of the other variants.
//!    This variant has a `Result<Option<`[`serde_json::Value`]`>, `[`serde_json::Error`]`>` value.
//!
//!    If the response body is empty, this value will be `Ok(None)`.
//!
//!    If the response body is not empty, this value will be an `Ok(Some(value))` or `Err(err)` from attempting to parse that body as a `serde_json::Value`.
//!    If you expect the response body to be a specific JSON type such as [`apimachinery::pkg::apis::meta::v1::Status`], you can use the `serde_json::Value`
//!    as a [`serde::Deserializer`] like `let status = <Status as Deserialize>::deserialize(value)?;`. On the other hand, if you expect the response body to not be
//!    a JSON value, then ignore the `Err(err)` and parse the raw bytes of the response into the appropriate type.
//!
//! Also see the `get_single_value` and `get_multiple_values` functions in
//! [the `k8s-openapi-tests` directory in the repository](https://github.com/Arnavion/k8s-openapi/tree/master/k8s-openapi-tests/src)
//! for examples of how to use a synchronous client with this style of API.
//!
#![cfg_attr(feature = "api", doc = "```rust,no_run")]
#![cfg_attr(not(feature = "api"), doc = "```rust,ignore")]
//! // Re-export of the http crate since it's used in the public API
//! use k8s_openapi::http;
//!
//! use k8s_openapi::api::core::v1 as api;
//!
//! # struct Response;
//! # impl Response {
//! #     fn status_code(&self) -> http::StatusCode {
//! #         unimplemented!()
//! #     }
//! #     fn read_into(&self, _buf: &mut [u8]) -> std::io::Result<usize> {
//! #         unimplemented!()
//! #     }
//! # }
//! #
//! // Assume `execute` is some function that takes an `http::Request` and
//! // executes it synchronously or asynchronously to get a response. This is
//! // provided by your HTTP client library.
//! //
//! // Note that the `http::Request` values returned by API operation functions
//! // only have a URL path, query string and request body filled out. That is,
//! // they do *not* have a URL host. So the real `execute` implementation
//! // would first mutate the URL of the request to an absolute URL with
//! // the API server's authority, add authorization headers, etc before
//! // actually executing it.
//! fn execute(req: http::Request<Vec<u8>>) -> Response { unimplemented!(); }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a `http::Request` to list all the pods in the
//!     // "kube-system" namespace.
//!     let (request, response_body) =
//!         api::Pod::list_namespaced_pod("kube-system", Default::default())?;
//!
//!     // Execute the request and get a response.
//!     // If this is an asynchronous operation, you would await
//!     // or otherwise yield to the event loop here.
//!     let response = execute(request);
//!
//!     // Got a status code from executing the request.
//!     let status_code: http::StatusCode = response.status_code();
//!
//!     // Construct the `ResponseBody<ListResponse<Pod>>` using the
//!     // constructor returned by the API function.
//!     let mut response_body = response_body(status_code);
//!
//!     // Buffer used for each read from the HTTP response.
//!     let mut buf = Box::new([0u8; 4096]);
//!
//!     let pod_list = loop {
//!         // Read some bytes from the HTTP response into the buffer.
//!         // If this is an asynchronous operation, you would await or
//!         // yield to the event loop here.
//!         let read = response.read_into(&mut *buf)?;
//!
//!         // `buf` now contains some data read from the response. Append it
//!         // to the `ResponseBody` and try to parse it into
//!         // the response type.
//!         response_body.append_slice(&buf[..read]);
//!         let response = response_body.parse();
//!         match response {
//!             // Successful response (HTTP 200 and parsed successfully)
//!             Ok(k8s_openapi::ListResponse::Ok(pod_list)) =>
//!                 break pod_list,
//!
//!             // Some unexpected response
//!             // (not HTTP 200, but still parsed successfully)
//!             Ok(other) => return Err(format!(
//!                 "expected Ok but got {} {:?}",
//!                 status_code, other).into()),
//!
//!             // Need more response data.
//!             // Read more bytes from the response into the `ResponseBody`
//!             Err(k8s_openapi::ResponseError::NeedMoreData) => continue,
//!
//!             // Some other error, like the response body being
//!             // malformed JSON or invalid UTF-8.
//!             Err(err) => return Err(format!(
//!                 "error: {} {:?}",
//!                 status_code, err).into()),
//!         }
//!     };
//!
//!     for pod in pod_list.items {
//!         println!("{:#?}", pod);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! # Crate features
//!
//! - This crate contains several `v4_*` features. Enabling one of the `v4_*` features selects which version of the Kubernetes API server this crate should target.
//!   For example, enabling the `v4_3` feature means the crate will only contain the API exposed by OpenShift 4.3. It will not expose API
//!   that were removed in 4.3 or earlier, nor any API added in 4.4 or later.
//!
//! - The crate also contains a feature named `api`. If this feature is disabled, the library will only contain the resource types like [`api::core::v1::Pod`],
//!   and not the associated operation functions like [`api::core::v1::Pod::read_namespaced_pod`]. The `Response` and `Optional` types for the operation functions
//!   will also not be accessible.
//!
//!   This feature is enabled by default, but can be disabled if your crate does not need the operation functions to save on compile time and resources.
//!
//! One and only one of the `v1_*` features must be enabled at the same time, otherwise the crate will not compile. This ensures that all crates in the crate graph
//! use the same types. If it was possible for one library crate to use `api::core::v1::Pod` corresponding to v1.15 and another to use the type
//! corresponding to v1.16, an application would not be able to use the same `Pod` value with both.
//!
//! Thus, it is recommended that only application crates must enable one of the `v1_*` features, corresponding to the version of Kubernetes
//! that the application wants to support.
//!
//! ```toml
//! # For application crates
//!
//! [dependencies]
//! k8s-openapi = { version = "...", features = ["v1_14"] }
//! ```
//!
//! If you're writing a library crate, your crate *must not* enable any features of `k8s-openapi` directly. The choice of which feature to enable
//! must be left to any application crates that use your library. This ensures that all `k8s-openapi`-using dependencies in that application crate's dependency graph
//! use the same set of `k8s-openapi` types and are interoperable.
//!
//! If your library crate has tests or examples, you should also add a dev-dependency on `k8s-openapi` in addition to the direct dependency,
//! and enable a version feature only for that dev-dependency.
//!
//! ```toml
//! # For library crates
//!
//! [dependencies]
//! k8s-openapi = "..."
//!
//! [dev-dependencies]
//! k8s-openapi = { version = "...", features = ["v1_14"] }
//! ```
//!
//!
//! # Conditional compilation
//!
//! As the previous section explained, library crates must not enable any version features in their `k8s-openapi` dependency. However, your library crate may
//! need to know about which version gets selected eventually.
//!
//! For example:
//!
//! 1. Your crate creates a custom resource definition using the apiextensions v1 API. This API is only available in Kubernetes 1.16+,
//!    so your crate would fail to compile if a lower feature was enabled.
//!
//! 1. Your crate creates a custom resource definition. If the `v1_16` or later feature is enabled, your crate wants to use the apiextensions v1 API,
//!    otherwise it falls back to the v1beta1 API.
//!
//! There are two ways for your crate to determine which feature of `k8s-openapi` is enabled:
//!
//! 1. The `k8s-openapi` crate exports [`k8s_if_*` macros,](#macros) which either expand to their contents or don't. See the docs of the macros for more details.
//!
//!    With these macros, the two cases above would be solved like this:
//!
//!    - ```rust,ignore
//!      #[macro_use] extern crate k8s_openapi;
//!
//!      // The compile_error!() is only emitted if 1.15 or lower is selected.
//!      k8s_if_le_1_15! {
//!          compile_error!("This crate requires the v1_16 (or higher) feature to be enabled on the k8s-openapi crate.");
//!      }
//!      ```
//!
//!    - ```rust,ignore
//!      #[macro_use] extern crate k8s_openapi;
//!
//!      k8s_if_le_1_15! {
//!          use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;
//!      }
//!      k8s_if_ge_1_16! {
//!          use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensions;
//!      }
//!
//!      // Common fields regardless of the apiextensions version
//!      let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
//!          group: ...,
//!          names: ...,
//!          scope: ...,
//!          ..Default::default()
//!      };
//!
//!      // Set v1beta1 `version` and `validation` fields on v1.15 and earlier.
//!      k8s_if_le_1_15! {
//!          let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
//!              version: <FooBar as k8s_openapi::Resource>::VERSION.to_owned().into(),
//!              validation: Some(custom_resource_validation),
//!              ..custom_resource_definition_spec
//!          };
//!      }
//!      // Set v1 `versions` field on v1.16 and later.
//!      k8s_if_ge_1_16! {
//!          let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
//!              versions: vec![
//!                  apiextensions::CustomResourceDefinitionVersion {
//!                      name: <FooBar as k8s_openapi::Resource>::VERSION.to_owned(),
//!                      schema: Some(custom_resource_validation),
//!                      served: true,
//!                      storage: true,
//!                      ..Default::default()
//!                  },
//!              ].into(),
//!              ..custom_resource_definition_spec
//!          };
//!      }
//!      ```
//!
//! 1. The `k8s-openapi` crate emits the selected version number as metadata that your crate can read in a build script
//!    from the `DEP_K8S_OPENAPI_*_VERSION` env var.
//!
//!    ```rust,no_run
//!    // Your crate's build.rs
//!
//!    fn main() {
//!        let k8s_openapi_version: u32 =
//!            std::env::vars_os()
//!            .find_map(|(key, value)| {
//!                let key = key.into_string().ok()?;
//!                if key.starts_with("DEP_K8S_OPENAPI_") && key.ends_with("_VERSION") {
//!                    let value = value.into_string().ok()?;
//!                    Some(value)
//!                }
//!                else {
//!                    None
//!                }
//!            }).expect("DEP_K8S_OPENAPI_*_VERSION must have been set by k8s-openapi")
//!            .parse().expect("DEP_K8S_OPENAPI_*_VERSION is malformed");
//!
//!        // k8s_openapi_version has the format 0x00_MM_NN_00.
//!        //
//!        // - MM is the major version.
//!        // - NN is the minor version.
//!        //
//!        // Thus, if the v1_16 feature was enabled, k8s_openapi_version would be 0x00_01_10_00
//!
//!        // The build script can now do arbitrary things with the information.
//!        // For example, it could define custom cfgs:
//!        if k8s_openapi_version >= 0x00_01_10_00 {
//!            println!(r#"cargo:rustc-cfg=k8s_apiextensions="v1""#);
//!        }
//!        else {
//!            println!(r#"cargo:rustc-cfg=k8s_apiextensions="v1beta1""#);
//!        }
//!
//!        // or emit new source code files under OUT_DIR, or anything else a build script can do.
//!    }
//!    ```
//!
//!    With these cfgs, the two cases above would be solved like this:
//!
//!    - ```rust,ignore
//!      // Your crate's src/lib.rs
//!
//!      #[cfg(k8s_apiextensions = "v1")]
//!      compile_error!("This crate requires the v1_16 (or higher) feature to be enabled on the k8s-openapi crate.");
//!      ```
//!
//!    - ```rust,ignore
//!      #[cfg(k8s_apiextensions = "v1beta1")]
//!      use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;
//!      #[cfg(k8s_apiextensions = "v1")]
//!      use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensions;
//!
//!      // Common fields regardless of the apiextensions version
//!      let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
//!          group: ...,
//!          names: ...,
//!          scope: ...,
//!          ..Default::default()
//!      };
//!
//!      // Set v1beta1 `version` and `validation` fields on v1.15 and earlier.
//!      #[cfg(k8s_apiextensions = "v1beta1")]
//!      let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
//!          version: <FooBar as k8s_openapi::Resource>::VERSION.to_owned().into(),
//!          validation: Some(custom_resource_validation),
//!          ..custom_resource_definition_spec
//!      };
//!      // Set v1 `versions` field on v1.16 and later.
//!      #[cfg(k8s_apiextensions = "v1")]
//!      let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
//!          versions: vec![
//!              apiextensions::CustomResourceDefinitionVersion {
//!                  name: <FooBar as k8s_openapi::Resource>::VERSION.to_owned(),
//!                  schema: Some(custom_resource_validation),
//!                  served: true,
//!                  storage: true,
//!                  ..Default::default()
//!              },
//!          ].into(),
//!          ..custom_resource_definition_spec
//!      };
//!      ```
//!
//! Note that both approaches require your crate to have a direct dependency on the `k8s-openapi` crate. Neither approach is available if your crate
//! only has a transitive dependency on the `k8s-openapi` crate.
//!
//! The macros approach is easier to use since it doesn't require a build script.
//!
//! The build script method lets you emit arbitrary cfgs, emit arbitrary source code, and generally gives you more options, at the cost of needing a build script.
//! For example, `cfg()`s can be used in places where macros cannot, such as this example for conditionally setting attrs using `cfg_attr`:
//!
//! ```rust,ignore
//! #[derive(
//!     Clone, Debug, PartialEq,
//!     k8s_openapi_derive::CustomResourceDefinition,
//!     serde_derive::Deserialize, serde_derive::Serialize,
//! )]
//! #[custom_resource_definition(
//!     group = "k8s-openapi-tests-custom-resource-definition.com",
//!     version = "v1",
//!     plural = "foobars",
//!     namespaced,
//! )]
//! #[cfg_attr(k8s_apiextensions = "v1beta1", custom_resource_definition(has_subresources = "v1beta1"))]
//! #[cfg_attr(k8s_apiextensions = "v1", custom_resource_definition(has_subresources = "v1"))]
//! struct FooBarSpec {
//!     prop1: String,
//!     prop2: Vec<bool>,
//!     #[serde(skip_serializing_if = "Option::is_none")]
//!     prop3: Option<i32>,
//! }
//! ```
//!
//! It isn't possible to conditionally set attributes using macros, so the entire `struct FooBarSpec` declaration would have to be duplicated and wrapped inside
//! `k8s_if_le_1_15! { }` and `k8s_if_ge_1_16! { }` respectively.
//!
//!
//! # Custom resource definitions
//!
//! The [`k8s-openapi-derive` crate](https://crates.io/crates/k8s-openapi-derive) provides a custom derive for generating clientsets
//! for custom resources. See that crate's docs for more information.

pub use chrono;
pub use serde_json;
pub use serde_value;

#[cfg(feature = "v4_3")]
mod v4_3;
#[cfg(feature = "v4_3")]
pub use self::v4_3::*;

#[cfg(feature = "v4_4")]
mod v4_4;
#[cfg(feature = "v4_4")]
pub use self::v4_4::*;

#[cfg(feature = "v4_5")]
mod v4_5;
#[cfg(feature = "v4_5")]
pub use self::v4_5::*;

#[cfg(feature = "v4_6")]
mod v4_6;
#[cfg(feature = "v4_6")]
pub use self::v4_6::*;


include!(concat!(
    env!("OUT_DIR"),
    "/conditional_compilation_macros.rs"
));
