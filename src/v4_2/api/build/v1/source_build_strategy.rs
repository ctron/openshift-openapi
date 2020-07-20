// Generated from definition com.github.openshift.api.build.v1.SourceBuildStrategy

/// SourceBuildStrategy defines input parameters specific to an Source build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceBuildStrategy {
    /// env contains additional environment variables you want to pass into a builder container.
    pub env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// forcePull describes if the builder should pull the images from registry prior to building.
    pub force_pull: Option<bool>,

    /// from is reference to an DockerImage, ImageStreamTag, or ImageStreamImage from which the container image should be pulled
    pub from: k8s_openapi::api::core::v1::ObjectReference,

    /// incremental flag forces the Source build to do incremental builds if true.
    pub incremental: Option<bool>,

    /// pullSecret is the name of a Secret that would be used for setting up the authentication for pulling the container images from the private Docker registries
    pub pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference>,

    /// scripts is the location of Source scripts
    pub scripts: Option<String>,
}

impl<'de> serde::Deserialize<'de> for SourceBuildStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_env,
            Key_force_pull,
            Key_from,
            Key_incremental,
            Key_pull_secret,
            Key_scripts,
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
                            "env" => Field::Key_env,
                            "forcePull" => Field::Key_force_pull,
                            "from" => Field::Key_from,
                            "incremental" => Field::Key_incremental,
                            "pullSecret" => Field::Key_pull_secret,
                            "scripts" => Field::Key_scripts,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SourceBuildStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SourceBuildStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_force_pull: Option<bool> = None;
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_incremental: Option<bool> = None;
                let mut value_pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;
                let mut value_scripts: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_env => value_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_force_pull => value_force_pull = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_incremental => value_incremental = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pull_secret => value_pull_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scripts => value_scripts = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SourceBuildStrategy {
                    env: value_env,
                    force_pull: value_force_pull,
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    incremental: value_incremental,
                    pull_secret: value_pull_secret,
                    scripts: value_scripts,
                })
            }
        }

        deserializer.deserialize_struct(
            "SourceBuildStrategy",
            &[
                "env",
                "forcePull",
                "from",
                "incremental",
                "pullSecret",
                "scripts",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SourceBuildStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SourceBuildStrategy",
            1 +
            self.env.as_ref().map_or(0, |_| 1) +
            self.force_pull.as_ref().map_or(0, |_| 1) +
            self.incremental.as_ref().map_or(0, |_| 1) +
            self.pull_secret.as_ref().map_or(0, |_| 1) +
            self.scripts.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.force_pull {
            serde::ser::SerializeStruct::serialize_field(&mut state, "forcePull", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        if let Some(value) = &self.incremental {
            serde::ser::SerializeStruct::serialize_field(&mut state, "incremental", value)?;
        }
        if let Some(value) = &self.pull_secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pullSecret", value)?;
        }
        if let Some(value) = &self.scripts {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scripts", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
