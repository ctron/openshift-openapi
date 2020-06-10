// Generated from definition com.github.openshift.api.build.v1.BuildRequest

/// BuildRequest is the resource used to pass parameters to build generator
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildRequest {
    /// binary indicates a request to build from a binary provided to the builder
    pub binary: Option<crate::api::build::v1::BinaryBuildSource>,

    /// DockerStrategyOptions contains additional docker-strategy specific options for the build
    pub docker_strategy_options: Option<crate::api::build::v1::DockerStrategyOptions>,

    /// env contains additional environment variables you want to pass into a builder container.
    pub env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// from is the reference to the ImageStreamTag that triggered the build.
    pub from: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// lastVersion (optional) is the LastVersion of the BuildConfig that was used to generate the build. If the BuildConfig in the generator doesn't match, a build will not be generated.
    pub last_version: Option<i64>,

    /// metadata for BuildRequest.
    pub metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// revision is the information from the source for a specific repo snapshot.
    pub revision: Option<crate::api::build::v1::SourceRevision>,

    /// SourceStrategyOptions contains additional source-strategy specific options for the build
    pub source_strategy_options: Option<crate::api::build::v1::SourceStrategyOptions>,

    /// triggeredBy describes which triggers started the most recent update to the build configuration and contains information about those triggers.
    pub triggered_by: Vec<crate::api::build::v1::BuildTriggerCause>,

    /// triggeredByImage is the Image that triggered this build.
    pub triggered_by_image: Option<k8s_openapi::api::core::v1::ObjectReference>,
}

// Begin build.openshift.io/v1/BuildRequest

// Generated from operation createBuildOpenshiftIoV1NamespacedBuildClone

impl BuildRequest {
    /// create clone of a Build
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::CreateResponse`]`<Self>>` constructor, or [`k8s_openapi::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the BuildRequest
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
    pub fn create_namespaced_build_clone(
        name: &str,
        namespace: &str,
        body: &crate::api::build::v1::BuildRequest,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/build.openshift.io/v1/namespaces/{namespace}/builds/{name}/clone?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation createBuildOpenshiftIoV1NamespacedBuildConfigInstantiate

impl BuildRequest {
    /// create instantiate of a BuildConfig
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`CreateNamespacedBuildConfigInstantiateResponse`]`>` constructor, or [`CreateNamespacedBuildConfigInstantiateResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the BuildRequest
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
    pub fn create_namespaced_build_config_instantiate(
        name: &str,
        namespace: &str,
        body: &crate::api::build::v1::BuildRequest,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<CreateNamespacedBuildConfigInstantiateResponse>), k8s_openapi::RequestError> {
        let __url = format!("/apis/build.openshift.io/v1/namespaces/{namespace}/buildconfigs/{name}/instantiate?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Use `<CreateNamespacedBuildConfigInstantiateResponse as Response>::try_from_parts` to parse the HTTP response body of [`BuildRequest::create_namespaced_build_config_instantiate`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNamespacedBuildConfigInstantiateResponse {
    Ok(crate::api::build::v1::Build),
    Created(crate::api::build::v1::Build),
    Accepted(crate::api::build::v1::Build),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for CreateNamespacedBuildConfigInstantiateResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedBuildConfigInstantiateResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedBuildConfigInstantiateResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedBuildConfigInstantiateResponse::Accepted(result), buf.len()))
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
                Ok((CreateNamespacedBuildConfigInstantiateResponse::Other(result), read))
            },
        }
    }
}

// End build.openshift.io/v1/BuildRequest

impl k8s_openapi::Resource for BuildRequest {
    const API_VERSION: &'static str = "build.openshift.io/v1";
    const GROUP: &'static str = "build.openshift.io";
    const KIND: &'static str = "BuildRequest";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::Metadata for BuildRequest {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as k8s_openapi::Metadata>::Ty> {
        self.metadata.as_ref()
    }

    fn metadata_mut(&mut self) -> Option<&mut<Self as k8s_openapi::Metadata>::Ty> {
        self.metadata.as_mut()
    }

    fn set_metadata(&mut self, metadata: <Self as k8s_openapi::Metadata>::Ty) {
        self.metadata = Some(metadata);
    }
}

impl<'de> serde::Deserialize<'de> for BuildRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_binary,
            Key_docker_strategy_options,
            Key_env,
            Key_from,
            Key_last_version,
            Key_metadata,
            Key_revision,
            Key_source_strategy_options,
            Key_triggered_by,
            Key_triggered_by_image,
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
                            "binary" => Field::Key_binary,
                            "dockerStrategyOptions" => Field::Key_docker_strategy_options,
                            "env" => Field::Key_env,
                            "from" => Field::Key_from,
                            "lastVersion" => Field::Key_last_version,
                            "metadata" => Field::Key_metadata,
                            "revision" => Field::Key_revision,
                            "sourceStrategyOptions" => Field::Key_source_strategy_options,
                            "triggeredBy" => Field::Key_triggered_by,
                            "triggeredByImage" => Field::Key_triggered_by_image,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildRequest;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_binary: Option<crate::api::build::v1::BinaryBuildSource> = None;
                let mut value_docker_strategy_options: Option<crate::api::build::v1::DockerStrategyOptions> = None;
                let mut value_env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_last_version: Option<i64> = None;
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_revision: Option<crate::api::build::v1::SourceRevision> = None;
                let mut value_source_strategy_options: Option<crate::api::build::v1::SourceStrategyOptions> = None;
                let mut value_triggered_by: Option<Vec<crate::api::build::v1::BuildTriggerCause>> = None;
                let mut value_triggered_by_image: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

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
                        Field::Key_binary => value_binary = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_strategy_options => value_docker_strategy_options = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env => value_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_version => value_last_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision => value_revision = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source_strategy_options => value_source_strategy_options = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_triggered_by => value_triggered_by = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_triggered_by_image => value_triggered_by_image = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildRequest {
                    binary: value_binary,
                    docker_strategy_options: value_docker_strategy_options,
                    env: value_env,
                    from: value_from,
                    last_version: value_last_version,
                    metadata: value_metadata,
                    revision: value_revision,
                    source_strategy_options: value_source_strategy_options,
                    triggered_by: value_triggered_by.ok_or_else(|| serde::de::Error::missing_field("triggeredBy"))?,
                    triggered_by_image: value_triggered_by_image,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "binary",
                "dockerStrategyOptions",
                "env",
                "from",
                "lastVersion",
                "metadata",
                "revision",
                "sourceStrategyOptions",
                "triggeredBy",
                "triggeredByImage",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            3 +
            self.binary.as_ref().map_or(0, |_| 1) +
            self.docker_strategy_options.as_ref().map_or(0, |_| 1) +
            self.env.as_ref().map_or(0, |_| 1) +
            self.from.as_ref().map_or(0, |_| 1) +
            self.last_version.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.revision.as_ref().map_or(0, |_| 1) +
            self.source_strategy_options.as_ref().map_or(0, |_| 1) +
            self.triggered_by_image.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.binary {
            serde::ser::SerializeStruct::serialize_field(&mut state, "binary", value)?;
        }
        if let Some(value) = &self.docker_strategy_options {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerStrategyOptions", value)?;
        }
        if let Some(value) = &self.env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.from {
            serde::ser::SerializeStruct::serialize_field(&mut state, "from", value)?;
        }
        if let Some(value) = &self.last_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lastVersion", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.revision {
            serde::ser::SerializeStruct::serialize_field(&mut state, "revision", value)?;
        }
        if let Some(value) = &self.source_strategy_options {
            serde::ser::SerializeStruct::serialize_field(&mut state, "sourceStrategyOptions", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "triggeredBy", &self.triggered_by)?;
        if let Some(value) = &self.triggered_by_image {
            serde::ser::SerializeStruct::serialize_field(&mut state, "triggeredByImage", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
