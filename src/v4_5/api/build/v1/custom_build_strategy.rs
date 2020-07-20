// Generated from definition com.github.openshift.api.build.v1.CustomBuildStrategy

/// CustomBuildStrategy defines input parameters specific to Custom build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomBuildStrategy {
    /// buildAPIVersion is the requested API version for the Build object serialized and passed to the custom builder
    pub build_api_version: Option<String>,

    /// env contains additional environment variables you want to pass into a builder container.
    pub env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// exposeDockerSocket will allow running Docker commands (and build container images) from inside the container.
    pub expose_docker_socket: Option<bool>,

    /// forcePull describes if the controller should configure the build pod to always pull the images for the builder or only pull if it is not present locally
    pub force_pull: Option<bool>,

    /// from is reference to an DockerImage, ImageStreamTag, or ImageStreamImage from which the container image should be pulled
    pub from: k8s_openapi::api::core::v1::ObjectReference,

    /// pullSecret is the name of a Secret that would be used for setting up the authentication for pulling the container images from the private Docker registries
    pub pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference>,

    /// secrets is a list of additional secrets that will be included in the build pod
    pub secrets: Option<Vec<crate::api::build::v1::SecretSpec>>,
}

impl<'de> serde::Deserialize<'de> for CustomBuildStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_build_api_version,
            Key_env,
            Key_expose_docker_socket,
            Key_force_pull,
            Key_from,
            Key_pull_secret,
            Key_secrets,
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
                            "buildAPIVersion" => Field::Key_build_api_version,
                            "env" => Field::Key_env,
                            "exposeDockerSocket" => Field::Key_expose_docker_socket,
                            "forcePull" => Field::Key_force_pull,
                            "from" => Field::Key_from,
                            "pullSecret" => Field::Key_pull_secret,
                            "secrets" => Field::Key_secrets,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomBuildStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomBuildStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_build_api_version: Option<String> = None;
                let mut value_env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_expose_docker_socket: Option<bool> = None;
                let mut value_force_pull: Option<bool> = None;
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;
                let mut value_secrets: Option<Vec<crate::api::build::v1::SecretSpec>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_build_api_version => value_build_api_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env => value_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_expose_docker_socket => value_expose_docker_socket = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_force_pull => value_force_pull = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_pull_secret => value_pull_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secrets => value_secrets = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomBuildStrategy {
                    build_api_version: value_build_api_version,
                    env: value_env,
                    expose_docker_socket: value_expose_docker_socket,
                    force_pull: value_force_pull,
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    pull_secret: value_pull_secret,
                    secrets: value_secrets,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomBuildStrategy",
            &[
                "buildAPIVersion",
                "env",
                "exposeDockerSocket",
                "forcePull",
                "from",
                "pullSecret",
                "secrets",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CustomBuildStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomBuildStrategy",
            1 +
            self.build_api_version.as_ref().map_or(0, |_| 1) +
            self.env.as_ref().map_or(0, |_| 1) +
            self.expose_docker_socket.as_ref().map_or(0, |_| 1) +
            self.force_pull.as_ref().map_or(0, |_| 1) +
            self.pull_secret.as_ref().map_or(0, |_| 1) +
            self.secrets.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.build_api_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "buildAPIVersion", value)?;
        }
        if let Some(value) = &self.env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.expose_docker_socket {
            serde::ser::SerializeStruct::serialize_field(&mut state, "exposeDockerSocket", value)?;
        }
        if let Some(value) = &self.force_pull {
            serde::ser::SerializeStruct::serialize_field(&mut state, "forcePull", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        if let Some(value) = &self.pull_secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pullSecret", value)?;
        }
        if let Some(value) = &self.secrets {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secrets", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
