// Generated from definition com.github.openshift.api.project.v1.ProjectSpec

/// ProjectSpec describes the attributes on a Project
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage
    pub finalizers: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for ProjectSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_finalizers,
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
                            "finalizers" => Field::Key_finalizers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProjectSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ProjectSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_finalizers: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_finalizers => value_finalizers = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ProjectSpec {
                    finalizers: value_finalizers,
                })
            }
        }

        deserializer.deserialize_struct(
            "ProjectSpec",
            &[
                "finalizers",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ProjectSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ProjectSpec",
            self.finalizers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.finalizers {
            serde::ser::SerializeStruct::serialize_field(&mut state, "finalizers", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
