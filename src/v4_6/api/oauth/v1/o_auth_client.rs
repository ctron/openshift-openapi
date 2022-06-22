// Generated from definition com.github.openshift.api.oauth.v1.OAuthClient

/// OAuthClient describes an OAuth client
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OAuthClient {
    /// AccessTokenInactivityTimeoutSeconds overrides the default token inactivity timeout for tokens granted to this client. The value represents the maximum amount of time that can occur between consecutive uses of the token. Tokens become invalid if they are not used within this temporal window. The user will need to acquire a new token to regain access once a token times out. This value needs to be set only if the default set in configuration is not appropriate for this client. Valid values are: - 0: Tokens for this client never time out - X: Tokens time out if there is no activity for X seconds The current minimum allowed value for X is 300 (5 minutes)
    pub access_token_inactivity_timeout_seconds: Option<i32>,

    /// AccessTokenMaxAgeSeconds overrides the default access token max age for tokens granted to this client. 0 means no expiration.
    pub access_token_max_age_seconds: Option<i32>,

    /// AdditionalSecrets holds other secrets that may be used to identify the client.  This is useful for rotation and for service account token validation
    pub additional_secrets: Option<Vec<String>>,

    /// GrantMethod determines how to handle grants for this client. If no method is provided, the cluster default grant handling method will be used. Valid grant handling methods are:
    ///  - auto:   always approves grant requests, useful for trusted clients
    ///  - prompt: prompts the end user for approval of grant requests, useful for third-party clients
    ///  - deny:   always denies grant requests, useful for black-listed clients
    pub grant_method: Option<String>,

    /// Standard object's metadata.
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// RedirectURIs is the valid redirection URIs associated with a client
    pub redirect_ur_is: Option<Vec<String>>,

    /// RespondWithChallenges indicates whether the client wants authentication needed responses made in the form of challenges instead of redirects
    pub respond_with_challenges: Option<bool>,

    /// ScopeRestrictions describes which scopes this client can request.  Each requested scope is checked against each restriction.  If any restriction matches, then the scope is allowed. If no restriction matches, then the scope is denied.
    pub scope_restrictions: Option<Vec<crate::api::oauth::v1::ScopeRestriction>>,

    /// Secret is the unique secret associated with a client
    pub secret: Option<String>,
}

// Begin oauth.openshift.io/v1/OAuthClient

// Generated from operation createOauthOpenshiftIoV1OAuthClient

impl OAuthClient {
    /// create an OAuthClient
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
    pub fn create_o_auth_client(
        body: &crate::api::oauth::v1::OAuthClient,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthclients?".to_owned();
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

// Generated from operation deleteOauthOpenshiftIoV1CollectionOAuthClient

impl OAuthClient {
    /// delete collection of OAuthClient
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
    pub fn delete_collection_o_auth_client(
        delete_optional: k8s_openapi::DeleteOptional<'_>,
        list_optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthclients?".to_owned();
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

// Generated from operation deleteOauthOpenshiftIoV1OAuthClient

impl OAuthClient {
    /// delete an OAuthClient
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthClient
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_o_auth_client(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/oauth.openshift.io/v1/oauthclients/{name}",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation listOauthOpenshiftIoV1OAuthClient

impl OAuthClient {
    /// list or watch objects of kind OAuthClient
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
    pub fn list_o_auth_client(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthclients?".to_owned();
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

// Generated from operation patchOauthOpenshiftIoV1OAuthClient

impl OAuthClient {
    /// partially update the specified OAuthClient
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthClient
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_o_auth_client(
        name: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/oauth.openshift.io/v1/oauthclients/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation readOauthOpenshiftIoV1OAuthClient

impl OAuthClient {
    /// read the specified OAuthClient
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadOAuthClientResponse`]`>` constructor, or [`ReadOAuthClientResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthClient
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_o_auth_client(
        name: &str,
        optional: ReadOAuthClientOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadOAuthClientResponse>), k8s_openapi::RequestError> {
        let ReadOAuthClientOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/oauth.openshift.io/v1/oauthclients/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

/// Optional parameters of [`OAuthClient::read_o_auth_client`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadOAuthClientOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadOAuthClientResponse as Response>::try_from_parts` to parse the HTTP response body of [`OAuthClient::read_o_auth_client`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadOAuthClientResponse {
    Ok(crate::api::oauth::v1::OAuthClient),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadOAuthClientResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadOAuthClientResponse::Ok(result), buf.len()))
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
                Ok((ReadOAuthClientResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceOauthOpenshiftIoV1OAuthClient

impl OAuthClient {
    /// replace the specified OAuthClient
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthClient
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_o_auth_client(
        name: &str,
        body: &crate::api::oauth::v1::OAuthClient,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/oauth.openshift.io/v1/oauthclients/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation watchOauthOpenshiftIoV1OAuthClient

impl OAuthClient {
    /// list or watch objects of kind OAuthClient
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
    pub fn watch_o_auth_client(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthclients?".to_owned();
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

// End oauth.openshift.io/v1/OAuthClient

impl k8s_openapi::Resource for OAuthClient {
    const API_VERSION: &'static str = "oauth.openshift.io/v1";
    const GROUP: &'static str = "oauth.openshift.io";
    const KIND: &'static str = "OAuthClient";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::ListableResource for OAuthClient {
    const LIST_KIND: &'static str = concat!("OAuthClient", "List");
}

impl k8s_openapi::Metadata for OAuthClient {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for OAuthClient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_access_token_inactivity_timeout_seconds,
            Key_access_token_max_age_seconds,
            Key_additional_secrets,
            Key_grant_method,
            Key_metadata,
            Key_redirect_ur_is,
            Key_respond_with_challenges,
            Key_scope_restrictions,
            Key_secret,
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
                            "accessTokenInactivityTimeoutSeconds" => Field::Key_access_token_inactivity_timeout_seconds,
                            "accessTokenMaxAgeSeconds" => Field::Key_access_token_max_age_seconds,
                            "additionalSecrets" => Field::Key_additional_secrets,
                            "grantMethod" => Field::Key_grant_method,
                            "metadata" => Field::Key_metadata,
                            "redirectURIs" => Field::Key_redirect_ur_is,
                            "respondWithChallenges" => Field::Key_respond_with_challenges,
                            "scopeRestrictions" => Field::Key_scope_restrictions,
                            "secret" => Field::Key_secret,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OAuthClient;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_access_token_inactivity_timeout_seconds: Option<i32> = None;
                let mut value_access_token_max_age_seconds: Option<i32> = None;
                let mut value_additional_secrets: Option<Vec<String>> = None;
                let mut value_grant_method: Option<String> = None;
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_redirect_ur_is: Option<Vec<String>> = None;
                let mut value_respond_with_challenges: Option<bool> = None;
                let mut value_scope_restrictions: Option<Vec<crate::api::oauth::v1::ScopeRestriction>> = None;
                let mut value_secret: Option<String> = None;

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
                        Field::Key_access_token_inactivity_timeout_seconds => value_access_token_inactivity_timeout_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_access_token_max_age_seconds => value_access_token_max_age_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_additional_secrets => value_additional_secrets = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_grant_method => value_grant_method = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_redirect_ur_is => value_redirect_ur_is = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_respond_with_challenges => value_respond_with_challenges = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope_restrictions => value_scope_restrictions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(OAuthClient {
                    access_token_inactivity_timeout_seconds: value_access_token_inactivity_timeout_seconds,
                    access_token_max_age_seconds: value_access_token_max_age_seconds,
                    additional_secrets: value_additional_secrets,
                    grant_method: value_grant_method,
                    metadata: value_metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    redirect_ur_is: value_redirect_ur_is,
                    respond_with_challenges: value_respond_with_challenges,
                    scope_restrictions: value_scope_restrictions,
                    secret: value_secret,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "accessTokenInactivityTimeoutSeconds",
                "accessTokenMaxAgeSeconds",
                "additionalSecrets",
                "grantMethod",
                "metadata",
                "redirectURIs",
                "respondWithChallenges",
                "scopeRestrictions",
                "secret",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for OAuthClient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            3 +
            self.access_token_inactivity_timeout_seconds.as_ref().map_or(0, |_| 1) +
            self.access_token_max_age_seconds.as_ref().map_or(0, |_| 1) +
            self.additional_secrets.as_ref().map_or(0, |_| 1) +
            self.grant_method.as_ref().map_or(0, |_| 1) +
            self.redirect_ur_is.as_ref().map_or(0, |_| 1) +
            self.respond_with_challenges.as_ref().map_or(0, |_| 1) +
            self.scope_restrictions.as_ref().map_or(0, |_| 1) +
            self.secret.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.access_token_inactivity_timeout_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "accessTokenInactivityTimeoutSeconds", value)?;
        }
        if let Some(value) = &self.access_token_max_age_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "accessTokenMaxAgeSeconds", value)?;
        }
        if let Some(value) = &self.additional_secrets {
            serde::ser::SerializeStruct::serialize_field(&mut state, "additionalSecrets", value)?;
        }
        if let Some(value) = &self.grant_method {
            serde::ser::SerializeStruct::serialize_field(&mut state, "grantMethod", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.redirect_ur_is {
            serde::ser::SerializeStruct::serialize_field(&mut state, "redirectURIs", value)?;
        }
        if let Some(value) = &self.respond_with_challenges {
            serde::ser::SerializeStruct::serialize_field(&mut state, "respondWithChallenges", value)?;
        }
        if let Some(value) = &self.scope_restrictions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scopeRestrictions", value)?;
        }
        if let Some(value) = &self.secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
