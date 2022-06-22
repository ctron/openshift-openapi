// Generated from definition com.github.openshift.api.build.v1.DockerStrategyOptions

/// DockerStrategyOptions contains extra strategy options for container image builds
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DockerStrategyOptions {
    /// Args contains any build arguments that are to be passed to Docker.  See https://docs.docker.com/engine/reference/builder/#/arg for more details
    pub build_args: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// noCache overrides the docker-strategy noCache option in the build config
    pub no_cache: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for DockerStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_build_args,
            Key_no_cache,
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
                            "buildArgs" => Field::Key_build_args,
                            "noCache" => Field::Key_no_cache,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DockerStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DockerStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_build_args: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_no_cache: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_build_args => value_build_args = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_no_cache => value_no_cache = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DockerStrategyOptions {
                    build_args: value_build_args,
                    no_cache: value_no_cache,
                })
            }
        }

        deserializer.deserialize_struct(
            "DockerStrategyOptions",
            &[
                "buildArgs",
                "noCache",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DockerStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DockerStrategyOptions",
            self.build_args.as_ref().map_or(0, |_| 1) +
            self.no_cache.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.build_args {
            serde::ser::SerializeStruct::serialize_field(&mut state, "buildArgs", value)?;
        }
        if let Some(value) = &self.no_cache {
            serde::ser::SerializeStruct::serialize_field(&mut state, "noCache", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
