// Generated from definition com.github.openshift.api.network.v1.EgressNetworkPolicy

/// EgressNetworkPolicy describes the current egress network policy for a Namespace. When using the 'redhat/openshift-ovs-multitenant' network plugin, traffic from a pod to an IP address outside the cluster will be checked against each EgressNetworkPolicyRule in the pod's namespace's EgressNetworkPolicy, in order. If no rule matches (or no EgressNetworkPolicy is present) then the traffic will be allowed by default.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EgressNetworkPolicy {
    /// metadata for EgressNetworkPolicy
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// spec is the specification of the current egress network policy
    pub spec: crate::api::network::v1::EgressNetworkPolicySpec,
}

// Begin network.openshift.io/v1/EgressNetworkPolicy

// Generated from operation createNetworkOpenshiftIoV1NamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// create an EgressNetworkPolicy
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::CreateResponse`]`<Self>>` constructor, or [`k8s_openapi::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_namespaced_egress_network_policy(
        namespace: &str,
        body: &crate::api::network::v1::EgressNetworkPolicy,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies?",
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteNetworkOpenshiftIoV1CollectionNamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// delete collection of EgressNetworkPolicy
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>>` constructor, or [`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_namespaced_egress_network_policy(
        namespace: &str,
        delete_optional: k8s_openapi::DeleteOptional<'_>,
        list_optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies?",
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&delete_optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteNetworkOpenshiftIoV1NamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// delete an EgressNetworkPolicy
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the EgressNetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_namespaced_egress_network_policy(
        name: &str,
        namespace: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies/{name}",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation listNetworkOpenshiftIoV1EgressNetworkPolicyForAllNamespaces

impl EgressNetworkPolicy {
    /// list or watch objects of kind EgressNetworkPolicy
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ListResponse`]`<Self>>` constructor, or [`k8s_openapi::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_egress_network_policy_for_all_namespaces(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/network.openshift.io/v1/egressnetworkpolicies?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation listNetworkOpenshiftIoV1NamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// list or watch objects of kind EgressNetworkPolicy
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ListResponse`]`<Self>>` constructor, or [`k8s_openapi::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_namespaced_egress_network_policy(
        namespace: &str,
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies?",
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchNetworkOpenshiftIoV1NamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// partially update the specified EgressNetworkPolicy
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the EgressNetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_namespaced_egress_network_policy(
        name: &str,
        namespace: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation readNetworkOpenshiftIoV1NamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// read the specified EgressNetworkPolicy
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadNamespacedEgressNetworkPolicyResponse`]`>` constructor, or [`ReadNamespacedEgressNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the EgressNetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_egress_network_policy(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedEgressNetworkPolicyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadNamespacedEgressNetworkPolicyResponse>), k8s_openapi::RequestError> {
        let ReadNamespacedEgressNetworkPolicyOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`EgressNetworkPolicy::read_namespaced_egress_network_policy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedEgressNetworkPolicyOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedEgressNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`EgressNetworkPolicy::read_namespaced_egress_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedEgressNetworkPolicyResponse {
    Ok(crate::api::network::v1::EgressNetworkPolicy),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadNamespacedEgressNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedEgressNetworkPolicyResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadNamespacedEgressNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceNetworkOpenshiftIoV1NamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// replace the specified EgressNetworkPolicy
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the EgressNetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_namespaced_egress_network_policy(
        name: &str,
        namespace: &str,
        body: &crate::api::network::v1::EgressNetworkPolicy,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchNetworkOpenshiftIoV1EgressNetworkPolicyForAllNamespaces

impl EgressNetworkPolicy {
    /// list or watch objects of kind EgressNetworkPolicy
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::WatchResponse`]`<Self>>` constructor, or [`k8s_openapi::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_egress_network_policy_for_all_namespaces(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/network.openshift.io/v1/egressnetworkpolicies?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchNetworkOpenshiftIoV1NamespacedEgressNetworkPolicy

impl EgressNetworkPolicy {
    /// list or watch objects of kind EgressNetworkPolicy
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::WatchResponse`]`<Self>>` constructor, or [`k8s_openapi::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_namespaced_egress_network_policy(
        namespace: &str,
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/network.openshift.io/v1/namespaces/{namespace}/egressnetworkpolicies?",
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// End network.openshift.io/v1/EgressNetworkPolicy

impl k8s_openapi::Resource for EgressNetworkPolicy {
    const API_VERSION: &'static str = "network.openshift.io/v1";
    const GROUP: &'static str = "network.openshift.io";
    const KIND: &'static str = "EgressNetworkPolicy";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::ListableResource for EgressNetworkPolicy {
    const LIST_KIND: &'static str = concat!("EgressNetworkPolicy", "List");
}

impl k8s_openapi::Metadata for EgressNetworkPolicy {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for EgressNetworkPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EgressNetworkPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::network::v1::EgressNetworkPolicySpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as k8s_openapi::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as k8s_openapi::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as k8s_openapi::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as k8s_openapi::Resource>::KIND));
                            }
                        },
                        Field::Key_metadata => value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_spec => value_spec = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EgressNetworkPolicy {
                    metadata: value_metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    spec: value_spec.ok_or_else(|| serde::de::Error::missing_field("spec"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EgressNetworkPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            4,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        serde::ser::SerializeStruct::end(state)
    }
}
