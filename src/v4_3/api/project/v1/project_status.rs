// Generated from definition com.github.openshift.api.project.v1.ProjectStatus

/// ProjectStatus is information about the current status of a Project
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectStatus {
    /// Phase is the current lifecycle phase of the project
    pub phase: Option<String>,
}

impl<'de> serde::Deserialize<'de> for ProjectStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_phase,
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
                            "phase" => Field::Key_phase,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProjectStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ProjectStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_phase: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_phase => value_phase = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ProjectStatus {
                    phase: value_phase,
                })
            }
        }

        deserializer.deserialize_struct(
            "ProjectStatus",
            &[
                "phase",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ProjectStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ProjectStatus",
            self.phase.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.phase {
            serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
