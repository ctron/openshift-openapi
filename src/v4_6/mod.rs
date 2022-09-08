pub mod api;

// Generated from operation getAppsOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetAppsOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetAppsOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apps_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetAppsOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/apps.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetAppsOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apps_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAppsOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetAppsOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetAppsOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetAppsOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAppsOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetAppsOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetAppsOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_apps_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetAppsOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/apps.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetAppsOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_apps_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAppsOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetAppsOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetAppsOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAppsOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthorizationOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetAuthorizationOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetAuthorizationOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authorization_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetAuthorizationOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/authorization.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetAuthorizationOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authorization_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthorizationOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetAuthorizationOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetAuthorizationOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetAuthorizationOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getAuthorizationOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetAuthorizationOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetAuthorizationOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_authorization_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetAuthorizationOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/authorization.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetAuthorizationOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_authorization_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetAuthorizationOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetAuthorizationOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetAuthorizationOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetAuthorizationOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getBuildOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetBuildOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetBuildOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_build_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetBuildOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/build.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetBuildOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_build_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetBuildOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetBuildOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetBuildOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetBuildOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getBuildOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetBuildOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetBuildOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_build_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetBuildOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/build.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetBuildOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_build_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetBuildOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetBuildOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetBuildOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetBuildOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getImageOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetImageOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetImageOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_image_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetImageOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/image.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetImageOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_image_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetImageOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetImageOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetImageOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetImageOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getImageOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetImageOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetImageOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_image_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetImageOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/image.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetImageOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_image_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetImageOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetImageOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetImageOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetImageOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getNetworkOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetNetworkOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetNetworkOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_network_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetNetworkOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/network.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetNetworkOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_network_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetNetworkOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetNetworkOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetNetworkOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetNetworkOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getNetworkOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetNetworkOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetNetworkOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_network_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetNetworkOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/network.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetNetworkOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_network_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetNetworkOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetNetworkOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetNetworkOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetNetworkOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getOauthOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetOauthOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetOauthOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_oauth_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetOauthOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/oauth.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetOauthOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_oauth_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetOauthOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetOauthOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetOauthOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetOauthOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getOauthOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetOauthOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetOauthOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_oauth_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetOauthOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/oauth.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetOauthOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_oauth_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetOauthOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetOauthOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetOauthOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetOauthOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getProjectOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetProjectOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetProjectOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_project_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetProjectOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/project.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetProjectOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_project_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetProjectOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetProjectOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetProjectOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetProjectOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getProjectOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetProjectOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetProjectOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_project_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetProjectOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/project.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetProjectOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_project_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetProjectOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetProjectOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetProjectOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetProjectOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getQuotaOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetQuotaOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetQuotaOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_quota_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetQuotaOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/quota.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetQuotaOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_quota_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetQuotaOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetQuotaOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetQuotaOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetQuotaOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getQuotaOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetQuotaOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetQuotaOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_quota_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetQuotaOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/quota.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetQuotaOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_quota_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetQuotaOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetQuotaOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetQuotaOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetQuotaOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getRouteOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetRouteOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetRouteOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_route_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetRouteOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/route.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetRouteOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_route_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetRouteOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetRouteOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetRouteOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetRouteOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getRouteOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetRouteOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetRouteOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_route_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetRouteOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/route.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetRouteOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_route_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetRouteOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetRouteOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetRouteOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetRouteOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getSecurityOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetSecurityOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetSecurityOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_security_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetSecurityOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/security.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetSecurityOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_security_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetSecurityOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetSecurityOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetSecurityOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetSecurityOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getSecurityOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetSecurityOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetSecurityOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_security_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetSecurityOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/security.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetSecurityOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_security_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetSecurityOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetSecurityOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetSecurityOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetSecurityOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getTemplateOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetTemplateOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetTemplateOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_template_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetTemplateOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/template.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetTemplateOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_template_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetTemplateOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetTemplateOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetTemplateOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetTemplateOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getTemplateOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetTemplateOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetTemplateOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_template_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetTemplateOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/template.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetTemplateOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_template_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetTemplateOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetTemplateOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetTemplateOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetTemplateOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getUserOpenshiftIoAPIGroup

/// get information of a group
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetUserOpenshiftIoAPIGroupResponse`]`>` constructor, or [`GetUserOpenshiftIoAPIGroupResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_user_openshift_io_api_group(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetUserOpenshiftIoAPIGroupResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/user.openshift.io/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetUserOpenshiftIoAPIGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_user_openshift_io_api_group`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetUserOpenshiftIoAPIGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetUserOpenshiftIoAPIGroupResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetUserOpenshiftIoAPIGroupResponse::Ok(result), buf.len()))
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
                Ok((GetUserOpenshiftIoAPIGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation getUserOpenshiftIoV1APIResources

/// get available resources
///
/// Use the returned [`k8s_openapi::ResponseBody`]`<`[`GetUserOpenshiftIoV1APIResourcesResponse`]`>` constructor, or [`GetUserOpenshiftIoV1APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_user_openshift_io_v1_api_resources(
) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<GetUserOpenshiftIoV1APIResourcesResponse>), k8s_openapi::RequestError> {
    let __url = "/apis/user.openshift.io/v1/".to_owned();

    let __request = http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
        Err(err) => Err(k8s_openapi::RequestError::Http(err)),
    }
}

/// Use `<GetUserOpenshiftIoV1APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_user_openshift_io_v1_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetUserOpenshiftIoV1APIResourcesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for GetUserOpenshiftIoV1APIResourcesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((GetUserOpenshiftIoV1APIResourcesResponse::Ok(result), buf.len()))
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
                Ok((GetUserOpenshiftIoV1APIResourcesResponse::Other(result), read))
            },
        }
    }
}
