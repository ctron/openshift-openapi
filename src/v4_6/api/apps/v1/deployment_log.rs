// Generated from definition com.github.openshift.api.apps.v1.DeploymentLog

/// DeploymentLog represents the logs for a deployment
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentLog {
}

// Begin apps.openshift.io/v1/DeploymentLog

// Generated from operation readAppsOpenshiftIoV1NamespacedDeploymentConfigLog

impl DeploymentLog {
    /// read log of the specified DeploymentConfig
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadNamespacedDeploymentConfigLogResponse`]`>` constructor, or [`ReadNamespacedDeploymentConfigLogResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the DeploymentLog
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_deployment_config_log(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedDeploymentConfigLogOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadNamespacedDeploymentConfigLogResponse>), k8s_openapi::RequestError> {
        let ReadNamespacedDeploymentConfigLogOptional {
            container,
            follow,
            limit_bytes,
            nowait,
            pretty,
            previous,
            since_seconds,
            tail_lines,
            timestamps,
            version,
        } = optional;
        let __url = format!("/apis/apps.openshift.io/v1/namespaces/{namespace}/deploymentconfigs/{name}/log?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(follow) = follow {
            __query_pairs.append_pair("follow", &follow.to_string());
        }
        if let Some(limit_bytes) = limit_bytes {
            __query_pairs.append_pair("limitBytes", &limit_bytes.to_string());
        }
        if let Some(nowait) = nowait {
            __query_pairs.append_pair("nowait", &nowait.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(previous) = previous {
            __query_pairs.append_pair("previous", &previous.to_string());
        }
        if let Some(since_seconds) = since_seconds {
            __query_pairs.append_pair("sinceSeconds", &since_seconds.to_string());
        }
        if let Some(tail_lines) = tail_lines {
            __query_pairs.append_pair("tailLines", &tail_lines.to_string());
        }
        if let Some(timestamps) = timestamps {
            __query_pairs.append_pair("timestamps", &timestamps.to_string());
        }
        if let Some(version) = version {
            __query_pairs.append_pair("version", &version.to_string());
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

/// Optional parameters of [`DeploymentLog::read_namespaced_deployment_config_log`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedDeploymentConfigLogOptional<'a> {
    /// The container for which to stream logs. Defaults to only container if there is one container in the pod.
    pub container: Option<&'a str>,
    /// Follow if true indicates that the build log should be streamed until the build terminates.
    pub follow: Option<bool>,
    /// If set, the number of bytes to read from the server before terminating the log output. This may not display a complete final line of logging, and may return slightly more or slightly less than the specified limit.
    pub limit_bytes: Option<i64>,
    /// NoWait if true causes the call to return immediately even if the deployment is not available yet. Otherwise the server will wait until the deployment has started.
    pub nowait: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Return previous deployment logs. Defaults to false.
    pub previous: Option<bool>,
    /// A relative time in seconds before the current time from which to show logs. If this value precedes the time a pod was started, only logs since the pod start will be returned. If this value is in the future, no logs will be returned. Only one of sinceSeconds or sinceTime may be specified.
    pub since_seconds: Option<i64>,
    /// If set, the number of lines from the end of the logs to show. If not specified, logs are shown from the creation of the container or sinceSeconds or sinceTime
    pub tail_lines: Option<i64>,
    /// If true, add an RFC3339 or RFC3339Nano timestamp at the beginning of every line of log output. Defaults to false.
    pub timestamps: Option<bool>,
    /// Version of the deployment for which to view logs.
    pub version: Option<i64>,
}

/// Use `<ReadNamespacedDeploymentConfigLogResponse as Response>::try_from_parts` to parse the HTTP response body of [`DeploymentLog::read_namespaced_deployment_config_log`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedDeploymentConfigLogResponse {
    Ok(crate::api::apps::v1::DeploymentLog),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadNamespacedDeploymentConfigLogResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedDeploymentConfigLogResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedDeploymentConfigLogResponse::Other(result), read))
            },
        }
    }
}

// End apps.openshift.io/v1/DeploymentLog

impl k8s_openapi::Resource for DeploymentLog {
    const API_VERSION: &'static str = "apps.openshift.io/v1";
    const GROUP: &'static str = "apps.openshift.io";
    const KIND: &'static str = "DeploymentLog";
    const VERSION: &'static str = "v1";
}

impl<'de> serde::Deserialize<'de> for DeploymentLog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentLog;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {

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
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentLog {
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentLog {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        serde::ser::SerializeStruct::end(state)
    }
}
