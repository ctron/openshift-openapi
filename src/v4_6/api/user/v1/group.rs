// Generated from definition com.github.openshift.api.user.v1.Group

/// Group represents a referenceable set of Users
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Group {
    /// Standard object's metadata.
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Users is the list of users in this group.
    pub users: Vec<String>,
}

// Begin user.openshift.io/v1/Group

// Generated from operation createUserOpenshiftIoV1Group

impl Group {
    /// create a Group
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
    pub fn create_group(
        body: &crate::api::user::v1::Group,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/user.openshift.io/v1/groups?".to_owned();
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

// Generated from operation deleteUserOpenshiftIoV1CollectionGroup

impl Group {
    /// delete collection of Group
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>>` constructor, or [`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_group(
        delete_optional: k8s_openapi::DeleteOptional<'_>,
        list_optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(
                http::StatusCode,
            )
                -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/user.openshift.io/v1/groups?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::delete(__url);
        let __body =
            serde_json::to_vec(&delete_optional).map_err(k8s_openapi::RequestError::Json)?;
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

// Generated from operation deleteUserOpenshiftIoV1Group

impl Group {
    /// delete a Group
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Group
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_group(
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
            "/apis/user.openshift.io/v1/groups/{name}",
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

// Generated from operation listUserOpenshiftIoV1Group

impl Group {
    /// list or watch objects of kind Group
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
    pub fn list_group(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/user.openshift.io/v1/groups?".to_owned();
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

// Generated from operation patchUserOpenshiftIoV1Group

impl Group {
    /// partially update the specified Group
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Group
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_group(
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
            "/apis/user.openshift.io/v1/groups/{name}?",
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

// Generated from operation readUserOpenshiftIoV1Group

impl Group {
    /// read the specified Group
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadGroupResponse`]`>` constructor, or [`ReadGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Group
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_group(
        name: &str,
        optional: ReadGroupOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadGroupResponse>,
        ),
        k8s_openapi::RequestError,
    > {
        let ReadGroupOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!(
            "/apis/user.openshift.io/v1/groups/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
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

/// Optional parameters of [`Group::read_group`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadGroupOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`Group::read_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadGroupResponse {
    Ok(crate::api::user::v1::Group),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadGroupResponse {
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
                Ok((ReadGroupResponse::Ok(result), buf.len()))
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
                Ok((ReadGroupResponse::Other(result), read))
            }
        }
    }
}

// Generated from operation replaceUserOpenshiftIoV1Group

impl Group {
    /// replace the specified Group
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Group
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_group(
        name: &str,
        body: &crate::api::user::v1::Group,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = format!(
            "/apis/user.openshift.io/v1/groups/{name}?",
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

// Generated from operation watchUserOpenshiftIoV1Group

impl Group {
    /// list or watch objects of kind Group
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
    pub fn watch_group(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/user.openshift.io/v1/groups?".to_owned();
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

// End user.openshift.io/v1/Group

impl k8s_openapi::Resource for Group {
    const API_VERSION: &'static str = "user.openshift.io/v1";
    const GROUP: &'static str = "user.openshift.io";
    const KIND: &'static str = "Group";
    const VERSION: &'static str = "v1";
    // fixed `Resource` impl
    const URL_PATH_SEGMENT: &'static str = "groups";
    type Scope = k8s_openapi::ClusterResourceScope;
}

impl k8s_openapi::ListableResource for Group {
    const LIST_KIND: &'static str = concat!("Group", "List");
}

impl k8s_openapi::Metadata for Group {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for Group {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_users,
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
                            "metadata" => Field::Key_metadata,
                            "users" => Field::Key_users,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Group;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value_metadata: Option<
                    k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                > = None;
                let mut value_users: Option<Vec<String>> = None;

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
                        Field::Key_metadata => {
                            value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_users => {
                            value_users = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Other => {
                            let _: serde::de::IgnoredAny =
                                serde::de::MapAccess::next_value(&mut map)?;
                        }
                    }
                }

                Ok(Group {
                    metadata: value_metadata
                        .ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    users: value_users.ok_or_else(|| serde::de::Error::missing_field("users"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &["apiVersion", "kind", "metadata", "users"],
            Visitor,
        )
    }
}

impl serde::Serialize for Group {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct(<Self as k8s_openapi::Resource>::KIND, 4)?;
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
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "users", &self.users)?;
        serde::ser::SerializeStruct::end(state)
    }
}
