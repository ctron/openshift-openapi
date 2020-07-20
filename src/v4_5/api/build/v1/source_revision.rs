// Generated from definition com.github.openshift.api.build.v1.SourceRevision

/// SourceRevision is the revision or commit information from the source for the build
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceRevision {
    /// Git contains information about git-based build source
    pub git: Option<crate::api::build::v1::GitSourceRevision>,

    /// type of the build source, may be one of 'Source', 'Dockerfile', 'Binary', or 'Images'
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for SourceRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_git,
            Key_type_,
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
                            "git" => Field::Key_git,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SourceRevision;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SourceRevision")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_git: Option<crate::api::build::v1::GitSourceRevision> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_git => value_git = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SourceRevision {
                    git: value_git,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SourceRevision",
            &[
                "git",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SourceRevision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SourceRevision",
            1 +
            self.git.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.git {
            serde::ser::SerializeStruct::serialize_field(&mut state, "git", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
