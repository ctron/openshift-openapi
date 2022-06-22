// Generated from definition com.github.openshift.api.apps.v1.DeploymentRequest

/// DeploymentRequest is a request to a deployment config for a new deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentRequest {
    /// ExcludeTriggers instructs the instantiator to avoid processing the specified triggers. This field overrides the triggers from latest and allows clients to control specific logic. This field is ignored if not specified.
    pub exclude_triggers: Option<Vec<String>>,

    /// Force will try to force a new deployment to run. If the deployment config is paused, then setting this to true will return an Invalid error.
    pub force: bool,

    /// Latest will update the deployment config with the latest state from all triggers.
    pub latest: bool,

    /// Name of the deployment config for requesting a new deployment.
    pub name: String,
}

// Begin apps.openshift.io/v1/DeploymentRequest

// Generated from operation createAppsOpenshiftIoV1NamespacedDeploymentConfigInstantiate

impl DeploymentRequest {
    /// create instantiate of a DeploymentConfig
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::CreateResponse`]`<Self>>` constructor, or [`k8s_openapi::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the DeploymentRequest
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
    pub fn create_namespaced_deployment_config_instantiate(
        name: &str,
        namespace: &str,
        body: &crate::api::apps::v1::DeploymentRequest,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/apps.openshift.io/v1/namespaces/{namespace}/deploymentconfigs/{name}/instantiate?",
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

// End apps.openshift.io/v1/DeploymentRequest

impl k8s_openapi::Resource for DeploymentRequest {
    const API_VERSION: &'static str = "apps.openshift.io/v1";
    const GROUP: &'static str = "apps.openshift.io";
    const KIND: &'static str = "DeploymentRequest";
    const VERSION: &'static str = "v1";
}

impl<'de> serde::Deserialize<'de> for DeploymentRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_exclude_triggers,
            Key_force,
            Key_latest,
            Key_name,
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
                            "excludeTriggers" => Field::Key_exclude_triggers,
                            "force" => Field::Key_force,
                            "latest" => Field::Key_latest,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentRequest;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_exclude_triggers: Option<Vec<String>> = None;
                let mut value_force: Option<bool> = None;
                let mut value_latest: Option<bool> = None;
                let mut value_name: Option<String> = None;

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
                        Field::Key_exclude_triggers => value_exclude_triggers = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_force => value_force = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_latest => value_latest = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentRequest {
                    exclude_triggers: value_exclude_triggers,
                    force: value_force.ok_or_else(|| serde::de::Error::missing_field("force"))?,
                    latest: value_latest.ok_or_else(|| serde::de::Error::missing_field("latest"))?,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "excludeTriggers",
                "force",
                "latest",
                "name",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            5 +
            self.exclude_triggers.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.exclude_triggers {
            serde::ser::SerializeStruct::serialize_field(&mut state, "excludeTriggers", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "force", &self.force)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "latest", &self.latest)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::end(state)
    }
}
