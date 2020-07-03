// Generated from definition com.github.openshift.api.template.v1.TemplateInstanceStatus

/// TemplateInstanceStatus describes the current state of a TemplateInstance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TemplateInstanceStatus {
    /// conditions represent the latest available observations of a TemplateInstance's current state.
    pub conditions: Option<Vec<crate::api::template::v1::TemplateInstanceCondition>>,

    /// Objects references the objects created by the TemplateInstance.
    pub objects: Option<Vec<crate::api::template::v1::TemplateInstanceObject>>,
}

impl<'de> serde::Deserialize<'de> for TemplateInstanceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_objects,
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
                            "conditions" => Field::Key_conditions,
                            "objects" => Field::Key_objects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TemplateInstanceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TemplateInstanceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::template::v1::TemplateInstanceCondition>> = None;
                let mut value_objects: Option<Vec<crate::api::template::v1::TemplateInstanceObject>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_objects => value_objects = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TemplateInstanceStatus {
                    conditions: value_conditions,
                    objects: value_objects,
                })
            }
        }

        deserializer.deserialize_struct(
            "TemplateInstanceStatus",
            &[
                "conditions",
                "objects",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TemplateInstanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TemplateInstanceStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.objects.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.objects {
            serde::ser::SerializeStruct::serialize_field(&mut state, "objects", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
