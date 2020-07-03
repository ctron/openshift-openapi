// Generated from definition com.github.openshift.api.build.v1.SourceStrategyOptions

/// SourceStrategyOptions contains extra strategy options for Source builds
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceStrategyOptions {
    /// incremental overrides the source-strategy incremental option in the build config
    pub incremental: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for SourceStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_incremental,
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
                            "incremental" => Field::Key_incremental,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SourceStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SourceStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_incremental: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_incremental => value_incremental = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SourceStrategyOptions {
                    incremental: value_incremental,
                })
            }
        }

        deserializer.deserialize_struct(
            "SourceStrategyOptions",
            &[
                "incremental",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SourceStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SourceStrategyOptions",
            self.incremental.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.incremental {
            serde::ser::SerializeStruct::serialize_field(&mut state, "incremental", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
