// Generated from definition com.github.openshift.api.oauth.v1.OAuthAuthorizeToken

/// OAuthAuthorizeToken describes an OAuth authorization token
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OAuthAuthorizeToken {
    /// ClientName references the client that created this token.
    pub client_name: Option<String>,

    /// CodeChallenge is the optional code_challenge associated with this authorization code, as described in rfc7636
    pub code_challenge: Option<String>,

    /// CodeChallengeMethod is the optional code_challenge_method associated with this authorization code, as described in rfc7636
    pub code_challenge_method: Option<String>,

    /// ExpiresIn is the seconds from CreationTime before this token expires.
    pub expires_in: Option<i64>,

    /// Standard object's metadata.
    pub metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// RedirectURI is the redirection associated with the token.
    pub redirect_uri: Option<String>,

    /// Scopes is an array of the requested scopes.
    pub scopes: Option<Vec<String>>,

    /// State data from request
    pub state: Option<String>,

    /// UserName is the user name associated with this token
    pub user_name: Option<String>,

    /// UserUID is the unique UID associated with this token. UserUID and UserName must both match for this token to be valid.
    pub user_uid: Option<String>,
}

// Begin oauth.openshift.io/v1/OAuthAuthorizeToken

// Generated from operation createOauthOpenshiftIoV1OAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// create an OAuthAuthorizeToken
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
    pub fn create_o_auth_authorize_token(
        body: &crate::api::oauth::v1::OAuthAuthorizeToken,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthauthorizetokens?".to_owned();
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

// Generated from operation deleteOauthOpenshiftIoV1CollectionOAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// delete collection of OAuthAuthorizeToken
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
    pub fn delete_collection_o_auth_authorize_token(
        delete_optional: k8s_openapi::DeleteOptional<'_>,
        list_optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthauthorizetokens?".to_owned();
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

// Generated from operation deleteOauthOpenshiftIoV1OAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// delete an OAuthAuthorizeToken
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthAuthorizeToken
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_o_auth_authorize_token(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/oauth.openshift.io/v1/oauthauthorizetokens/{name}",
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

// Generated from operation listOauthOpenshiftIoV1OAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// list or watch objects of kind OAuthAuthorizeToken
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
    pub fn list_o_auth_authorize_token(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthauthorizetokens?".to_owned();
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

// Generated from operation patchOauthOpenshiftIoV1OAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// partially update the specified OAuthAuthorizeToken
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthAuthorizeToken
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_o_auth_authorize_token(
        name: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/oauth.openshift.io/v1/oauthauthorizetokens/{name}?",
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

// Generated from operation readOauthOpenshiftIoV1OAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// read the specified OAuthAuthorizeToken
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadOAuthAuthorizeTokenResponse`]`>` constructor, or [`ReadOAuthAuthorizeTokenResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthAuthorizeToken
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_o_auth_authorize_token(
        name: &str,
        optional: ReadOAuthAuthorizeTokenOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadOAuthAuthorizeTokenResponse>), k8s_openapi::RequestError> {
        let ReadOAuthAuthorizeTokenOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/oauth.openshift.io/v1/oauthauthorizetokens/{name}?",
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

/// Optional parameters of [`OAuthAuthorizeToken::read_o_auth_authorize_token`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadOAuthAuthorizeTokenOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadOAuthAuthorizeTokenResponse as Response>::try_from_parts` to parse the HTTP response body of [`OAuthAuthorizeToken::read_o_auth_authorize_token`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadOAuthAuthorizeTokenResponse {
    Ok(crate::api::oauth::v1::OAuthAuthorizeToken),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadOAuthAuthorizeTokenResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadOAuthAuthorizeTokenResponse::Ok(result), buf.len()))
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
                Ok((ReadOAuthAuthorizeTokenResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceOauthOpenshiftIoV1OAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// replace the specified OAuthAuthorizeToken
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the OAuthAuthorizeToken
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_o_auth_authorize_token(
        name: &str,
        body: &crate::api::oauth::v1::OAuthAuthorizeToken,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/oauth.openshift.io/v1/oauthauthorizetokens/{name}?",
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

// Generated from operation watchOauthOpenshiftIoV1OAuthAuthorizeToken

impl OAuthAuthorizeToken {
    /// list or watch objects of kind OAuthAuthorizeToken
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
    pub fn watch_o_auth_authorize_token(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/oauth.openshift.io/v1/oauthauthorizetokens?".to_owned();
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

// End oauth.openshift.io/v1/OAuthAuthorizeToken

impl k8s_openapi::Resource for OAuthAuthorizeToken {
    const API_VERSION: &'static str = "oauth.openshift.io/v1";
    const GROUP: &'static str = "oauth.openshift.io";
    const KIND: &'static str = "OAuthAuthorizeToken";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::ListableResource for OAuthAuthorizeToken {
    const LIST_KIND: &'static str = concat!("OAuthAuthorizeToken", "List");
}

impl k8s_openapi::Metadata for OAuthAuthorizeToken {
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

impl<'de> serde::Deserialize<'de> for OAuthAuthorizeToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_client_name,
            Key_code_challenge,
            Key_code_challenge_method,
            Key_expires_in,
            Key_metadata,
            Key_redirect_uri,
            Key_scopes,
            Key_state,
            Key_user_name,
            Key_user_uid,
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
                            "clientName" => Field::Key_client_name,
                            "codeChallenge" => Field::Key_code_challenge,
                            "codeChallengeMethod" => Field::Key_code_challenge_method,
                            "expiresIn" => Field::Key_expires_in,
                            "metadata" => Field::Key_metadata,
                            "redirectURI" => Field::Key_redirect_uri,
                            "scopes" => Field::Key_scopes,
                            "state" => Field::Key_state,
                            "userName" => Field::Key_user_name,
                            "userUID" => Field::Key_user_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OAuthAuthorizeToken;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_client_name: Option<String> = None;
                let mut value_code_challenge: Option<String> = None;
                let mut value_code_challenge_method: Option<String> = None;
                let mut value_expires_in: Option<i64> = None;
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_redirect_uri: Option<String> = None;
                let mut value_scopes: Option<Vec<String>> = None;
                let mut value_state: Option<String> = None;
                let mut value_user_name: Option<String> = None;
                let mut value_user_uid: Option<String> = None;

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
                        Field::Key_client_name => value_client_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_code_challenge => value_code_challenge = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_code_challenge_method => value_code_challenge_method = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_expires_in => value_expires_in = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_redirect_uri => value_redirect_uri = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scopes => value_scopes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user_name => value_user_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user_uid => value_user_uid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(OAuthAuthorizeToken {
                    client_name: value_client_name,
                    code_challenge: value_code_challenge,
                    code_challenge_method: value_code_challenge_method,
                    expires_in: value_expires_in,
                    metadata: value_metadata,
                    redirect_uri: value_redirect_uri,
                    scopes: value_scopes,
                    state: value_state,
                    user_name: value_user_name,
                    user_uid: value_user_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "clientName",
                "codeChallenge",
                "codeChallengeMethod",
                "expiresIn",
                "metadata",
                "redirectURI",
                "scopes",
                "state",
                "userName",
                "userUID",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for OAuthAuthorizeToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            2 +
            self.client_name.as_ref().map_or(0, |_| 1) +
            self.code_challenge.as_ref().map_or(0, |_| 1) +
            self.code_challenge_method.as_ref().map_or(0, |_| 1) +
            self.expires_in.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.redirect_uri.as_ref().map_or(0, |_| 1) +
            self.scopes.as_ref().map_or(0, |_| 1) +
            self.state.as_ref().map_or(0, |_| 1) +
            self.user_name.as_ref().map_or(0, |_| 1) +
            self.user_uid.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.client_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "clientName", value)?;
        }
        if let Some(value) = &self.code_challenge {
            serde::ser::SerializeStruct::serialize_field(&mut state, "codeChallenge", value)?;
        }
        if let Some(value) = &self.code_challenge_method {
            serde::ser::SerializeStruct::serialize_field(&mut state, "codeChallengeMethod", value)?;
        }
        if let Some(value) = &self.expires_in {
            serde::ser::SerializeStruct::serialize_field(&mut state, "expiresIn", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.redirect_uri {
            serde::ser::SerializeStruct::serialize_field(&mut state, "redirectURI", value)?;
        }
        if let Some(value) = &self.scopes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scopes", value)?;
        }
        if let Some(value) = &self.state {
            serde::ser::SerializeStruct::serialize_field(&mut state, "state", value)?;
        }
        if let Some(value) = &self.user_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "userName", value)?;
        }
        if let Some(value) = &self.user_uid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "userUID", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
