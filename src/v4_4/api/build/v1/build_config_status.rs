// Generated from definition com.github.openshift.api.build.v1.BuildConfigStatus

/// BuildConfigStatus contains current state of the build config object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildConfigStatus {
    /// lastVersion is used to inform about number of last triggered build.
    pub last_version: i64,
}

impl<'de> serde::Deserialize<'de> for BuildConfigStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_version,
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
                            "lastVersion" => Field::Key_last_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildConfigStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildConfigStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_last_version: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_version => value_last_version = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildConfigStatus {
                    last_version: value_last_version.ok_or_else(|| serde::de::Error::missing_field("lastVersion"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildConfigStatus",
            &[
                "lastVersion",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildConfigStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildConfigStatus",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "lastVersion", &self.last_version)?;
        serde::ser::SerializeStruct::end(state)
    }
}
