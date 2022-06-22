// Generated from definition com.github.openshift.api.build.v1.SecretBuildSource

/// SecretBuildSource describes a secret and its destination directory that will be used only at the build time. The content of the secret referenced here will be copied into the destination directory instead of mounting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecretBuildSource {
    /// destinationDir is the directory where the files from the secret should be available for the build time. For the Source build strategy, these will be injected into a container where the assemble script runs. Later, when the script finishes, all files injected will be truncated to zero length. For the container image build strategy, these will be copied into the build directory, where the Dockerfile is located, so users can ADD or COPY them during container image build.
    pub destination_dir: Option<String>,

    /// secret is a reference to an existing secret that you want to use in your build.
    pub secret: k8s_openapi::api::core::v1::LocalObjectReference,
}

impl<'de> serde::Deserialize<'de> for SecretBuildSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_destination_dir,
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
                            "destinationDir" => Field::Key_destination_dir,
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
            type Value = SecretBuildSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SecretBuildSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_destination_dir: Option<String> = None;
                let mut value_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_destination_dir => value_destination_dir = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecretBuildSource {
                    destination_dir: value_destination_dir,
                    secret: value_secret.ok_or_else(|| serde::de::Error::missing_field("secret"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecretBuildSource",
            &[
                "destinationDir",
                "secret",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SecretBuildSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecretBuildSource",
            1 +
            self.destination_dir.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.destination_dir {
            serde::ser::SerializeStruct::serialize_field(&mut state, "destinationDir", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "secret", &self.secret)?;
        serde::ser::SerializeStruct::end(state)
    }
}
