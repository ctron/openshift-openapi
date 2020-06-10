// Generated from definition com.github.openshift.api.build.v1.SourceControlUser

/// SourceControlUser defines the identity of a user of source control
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceControlUser {
    /// email of the source control user
    pub email: Option<String>,

    /// name of the source control user
    pub name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for SourceControlUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_email,
            Key_name,
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
                            "email" => Field::Key_email,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SourceControlUser;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SourceControlUser")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_email: Option<String> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_email => value_email = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SourceControlUser {
                    email: value_email,
                    name: value_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "SourceControlUser",
            &[
                "email",
                "name",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SourceControlUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SourceControlUser",
            self.email.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.email {
            serde::ser::SerializeStruct::serialize_field(&mut state, "email", value)?;
        }
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
