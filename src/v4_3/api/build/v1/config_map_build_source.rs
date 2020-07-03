// Generated from definition com.github.openshift.api.build.v1.ConfigMapBuildSource

/// ConfigMapBuildSource describes a configmap and its destination directory that will be used only at the build time. The content of the configmap referenced here will be copied into the destination directory instead of mounting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigMapBuildSource {
    /// configMap is a reference to an existing configmap that you want to use in your build.
    pub config_map: k8s_openapi::api::core::v1::LocalObjectReference,

    /// destinationDir is the directory where the files from the configmap should be available for the build time. For the Source build strategy, these will be injected into a container where the assemble script runs. For the container image build strategy, these will be copied into the build directory, where the Dockerfile is located, so users can ADD or COPY them during container image build.
    pub destination_dir: Option<String>,
}

impl<'de> serde::Deserialize<'de> for ConfigMapBuildSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_map,
            Key_destination_dir,
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
                            "configMap" => Field::Key_config_map,
                            "destinationDir" => Field::Key_destination_dir,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConfigMapBuildSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ConfigMapBuildSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_config_map: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;
                let mut value_destination_dir: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_map => value_config_map = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_destination_dir => value_destination_dir = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ConfigMapBuildSource {
                    config_map: value_config_map.ok_or_else(|| serde::de::Error::missing_field("configMap"))?,
                    destination_dir: value_destination_dir,
                })
            }
        }

        deserializer.deserialize_struct(
            "ConfigMapBuildSource",
            &[
                "configMap",
                "destinationDir",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ConfigMapBuildSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ConfigMapBuildSource",
            1 +
            self.destination_dir.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "configMap", &self.config_map)?;
        if let Some(value) = &self.destination_dir {
            serde::ser::SerializeStruct::serialize_field(&mut state, "destinationDir", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
