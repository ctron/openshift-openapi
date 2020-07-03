// Generated from definition com.github.openshift.api.build.v1.GitSourceRevision

/// GitSourceRevision is the commit information from a git source for a build
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GitSourceRevision {
    /// author is the author of a specific commit
    pub author: Option<crate::api::build::v1::SourceControlUser>,

    /// commit is the commit hash identifying a specific commit
    pub commit: Option<String>,

    /// committer is the committer of a specific commit
    pub committer: Option<crate::api::build::v1::SourceControlUser>,

    /// message is the description of a specific commit
    pub message: Option<String>,
}

impl<'de> serde::Deserialize<'de> for GitSourceRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_author,
            Key_commit,
            Key_committer,
            Key_message,
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
                            "author" => Field::Key_author,
                            "commit" => Field::Key_commit,
                            "committer" => Field::Key_committer,
                            "message" => Field::Key_message,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GitSourceRevision;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GitSourceRevision")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_author: Option<crate::api::build::v1::SourceControlUser> = None;
                let mut value_commit: Option<String> = None;
                let mut value_committer: Option<crate::api::build::v1::SourceControlUser> = None;
                let mut value_message: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_author => value_author = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_commit => value_commit = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_committer => value_committer = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GitSourceRevision {
                    author: value_author,
                    commit: value_commit,
                    committer: value_committer,
                    message: value_message,
                })
            }
        }

        deserializer.deserialize_struct(
            "GitSourceRevision",
            &[
                "author",
                "commit",
                "committer",
                "message",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for GitSourceRevision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GitSourceRevision",
            self.author.as_ref().map_or(0, |_| 1) +
            self.commit.as_ref().map_or(0, |_| 1) +
            self.committer.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.author {
            serde::ser::SerializeStruct::serialize_field(&mut state, "author", value)?;
        }
        if let Some(value) = &self.commit {
            serde::ser::SerializeStruct::serialize_field(&mut state, "commit", value)?;
        }
        if let Some(value) = &self.committer {
            serde::ser::SerializeStruct::serialize_field(&mut state, "committer", value)?;
        }
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
