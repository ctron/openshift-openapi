// Generated from definition com.github.openshift.api.build.v1.BuildStatusOutput

/// BuildStatusOutput contains the status of the built image.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildStatusOutput {
    /// to describes the status of the built image being pushed to a registry.
    pub to: Option<crate::api::build::v1::BuildStatusOutputTo>,
}

impl<'de> serde::Deserialize<'de> for BuildStatusOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_to,
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
                            "to" => Field::Key_to,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildStatusOutput;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildStatusOutput")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_to: Option<crate::api::build::v1::BuildStatusOutputTo> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_to => value_to = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildStatusOutput {
                    to: value_to,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildStatusOutput",
            &[
                "to",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildStatusOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildStatusOutput",
            self.to.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.to {
            serde::ser::SerializeStruct::serialize_field(&mut state, "to", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
