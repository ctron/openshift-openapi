// Generated from definition com.github.openshift.api.image.v1.TagImportPolicy

/// TagImportPolicy controls how images related to this tag will be imported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagImportPolicy {
    /// Insecure is true if the server may bypass certificate verification or connect directly over HTTP during image import.
    pub insecure: Option<bool>,

    /// Scheduled indicates to the server that this tag should be periodically checked to ensure it is up to date, and imported
    pub scheduled: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for TagImportPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_insecure,
            Key_scheduled,
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
                            "insecure" => Field::Key_insecure,
                            "scheduled" => Field::Key_scheduled,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TagImportPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TagImportPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_insecure: Option<bool> = None;
                let mut value_scheduled: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_insecure => value_insecure = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduled => value_scheduled = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TagImportPolicy {
                    insecure: value_insecure,
                    scheduled: value_scheduled,
                })
            }
        }

        deserializer.deserialize_struct(
            "TagImportPolicy",
            &[
                "insecure",
                "scheduled",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TagImportPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TagImportPolicy",
            self.insecure.as_ref().map_or(0, |_| 1) +
            self.scheduled.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.insecure {
            serde::ser::SerializeStruct::serialize_field(&mut state, "insecure", value)?;
        }
        if let Some(value) = &self.scheduled {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scheduled", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
