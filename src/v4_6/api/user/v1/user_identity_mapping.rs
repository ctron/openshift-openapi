// Generated from definition com.github.openshift.api.user.v1.UserIdentityMapping

/// UserIdentityMapping maps a user to an identity
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserIdentityMapping {
    /// Identity is a reference to an identity
    pub identity: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// Standard object's metadata.
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// User is a reference to a user
    pub user: Option<k8s_openapi::api::core::v1::ObjectReference>,
}

// Begin user.openshift.io/v1/UserIdentityMapping

// Generated from operation createUserOpenshiftIoV1UserIdentityMapping

impl UserIdentityMapping {
    /// create an UserIdentityMapping
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::CreateResponse`]`<Self>>` constructor, or [`k8s_openapi::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_user_identity_mapping(
        body: &crate::api::user::v1::UserIdentityMapping,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/user.openshift.io/v1/useridentitymappings?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteUserOpenshiftIoV1UserIdentityMapping

impl UserIdentityMapping {
    /// delete an UserIdentityMapping
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the UserIdentityMapping
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_user_identity_mapping(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = format!(
            "/apis/user.openshift.io/v1/useridentitymappings/{name}",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchUserOpenshiftIoV1UserIdentityMapping

impl UserIdentityMapping {
    /// partially update the specified UserIdentityMapping
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the UserIdentityMapping
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_user_identity_mapping(
        name: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = format!(
            "/apis/user.openshift.io/v1/useridentitymappings/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static(match body {
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => {
                    "application/json-patch+json"
                }
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => {
                    "application/merge-patch+json"
                }
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => {
                    "application/strategic-merge-patch+json"
                }
            }),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation readUserOpenshiftIoV1UserIdentityMapping

impl UserIdentityMapping {
    /// read the specified UserIdentityMapping
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadUserIdentityMappingResponse`]`>` constructor, or [`ReadUserIdentityMappingResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the UserIdentityMapping
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_user_identity_mapping(
        name: &str,
        optional: ReadUserIdentityMappingOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadUserIdentityMappingResponse>,
        ),
        k8s_openapi::RequestError,
    > {
        let ReadUserIdentityMappingOptional { pretty } = optional;
        let __url = format!(
            "/apis/user.openshift.io/v1/useridentitymappings/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`UserIdentityMapping::read_user_identity_mapping`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadUserIdentityMappingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadUserIdentityMappingResponse as Response>::try_from_parts` to parse the HTTP response body of [`UserIdentityMapping::read_user_identity_mapping`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadUserIdentityMappingResponse {
    Ok(crate::api::user::v1::UserIdentityMapping),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadUserIdentityMappingResponse {
    fn try_from_parts(
        status_code: http::StatusCode,
        buf: &[u8],
    ) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => {
                        return Err(k8s_openapi::ResponseError::NeedMoreData)
                    }
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadUserIdentityMappingResponse::Ok(result), buf.len()))
            }
            _ => {
                let (result, read) = if buf.is_empty() {
                    (Ok(None), 0)
                } else {
                    match serde_json::from_slice(buf) {
                        Ok(value) => (Ok(Some(value)), buf.len()),
                        Err(ref err) if err.is_eof() => {
                            return Err(k8s_openapi::ResponseError::NeedMoreData)
                        }
                        Err(err) => (Err(err), 0),
                    }
                };
                Ok((ReadUserIdentityMappingResponse::Other(result), read))
            }
        }
    }
}

// Generated from operation replaceUserOpenshiftIoV1UserIdentityMapping

impl UserIdentityMapping {
    /// replace the specified UserIdentityMapping
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the UserIdentityMapping
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_user_identity_mapping(
        name: &str,
        body: &crate::api::user::v1::UserIdentityMapping,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = format!(
            "/apis/user.openshift.io/v1/useridentitymappings/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// End user.openshift.io/v1/UserIdentityMapping

impl k8s_openapi::Resource for UserIdentityMapping {
    const API_VERSION: &'static str = "user.openshift.io/v1";
    const GROUP: &'static str = "user.openshift.io";
    const KIND: &'static str = "UserIdentityMapping";
    const VERSION: &'static str = "v1";
    // fixed `Resource` impl
    const URL_PATH_SEGMENT: &'static str = "useridentitymappings";
    type Scope = k8s_openapi::ClusterResourceScope;
}

impl k8s_openapi::Metadata for UserIdentityMapping {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for UserIdentityMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_identity,
            Key_metadata,
            Key_user,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "identity" => Field::Key_identity,
                            "metadata" => Field::Key_metadata,
                            "user" => Field::Key_user,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UserIdentityMapping;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value_identity: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_metadata: Option<
                    k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                > = None;
                let mut value_user: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String =
                                serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version
                                != <Self::Value as k8s_openapi::Resource>::API_VERSION
                            {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value_api_version),
                                    &<Self::Value as k8s_openapi::Resource>::API_VERSION,
                                ));
                            }
                        }
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as k8s_openapi::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value_kind),
                                    &<Self::Value as k8s_openapi::Resource>::KIND,
                                ));
                            }
                        }
                        Field::Key_identity => {
                            value_identity = serde::de::MapAccess::next_value(&mut map)?
                        }
                        Field::Key_metadata => {
                            value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_user => value_user = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => {
                            let _: serde::de::IgnoredAny =
                                serde::de::MapAccess::next_value(&mut map)?;
                        }
                    }
                }

                Ok(UserIdentityMapping {
                    identity: value_identity,
                    metadata: value_metadata
                        .ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &["apiVersion", "kind", "identity", "metadata", "user"],
            Visitor,
        )
    }
}

impl serde::Serialize for UserIdentityMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            3 + self.identity.as_ref().map_or(0, |_| 1) + self.user.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "apiVersion",
            <Self as k8s_openapi::Resource>::API_VERSION,
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "kind",
            <Self as k8s_openapi::Resource>::KIND,
        )?;
        if let Some(value) = &self.identity {
            serde::ser::SerializeStruct::serialize_field(&mut state, "identity", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.user {
            serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
