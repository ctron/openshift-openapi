// Generated from definition com.github.openshift.api.build.v1.GitLabWebHookCause

/// GitLabWebHookCause has information about a GitLab webhook that triggered a build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GitLabWebHookCause {
    /// Revision is the git source revision information of the trigger.
    pub revision: Option<crate::api::build::v1::SourceRevision>,

    /// Secret is the obfuscated webhook secret that triggered a build.
    pub secret: Option<String>,
}

impl<'de> serde::Deserialize<'de> for GitLabWebHookCause {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_revision,
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
                            "revision" => Field::Key_revision,
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
            type Value = GitLabWebHookCause;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GitLabWebHookCause")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_revision: Option<crate::api::build::v1::SourceRevision> = None;
                let mut value_secret: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_revision => value_revision = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GitLabWebHookCause {
                    revision: value_revision,
                    secret: value_secret,
                })
            }
        }

        deserializer.deserialize_struct(
            "GitLabWebHookCause",
            &[
                "revision",
                "secret",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for GitLabWebHookCause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GitLabWebHookCause",
            self.revision.as_ref().map_or(0, |_| 1) +
            self.secret.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.revision {
            serde::ser::SerializeStruct::serialize_field(&mut state, "revision", value)?;
        }
        if let Some(value) = &self.secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
