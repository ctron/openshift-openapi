// Generated from definition com.github.openshift.api.template.v1.TemplateInstanceObject

/// TemplateInstanceObject references an object created by a TemplateInstance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TemplateInstanceObject {
    /// ref is a reference to the created object.  When used under .spec, only name and namespace are used; these can contain references to parameters which will be substituted following the usual rules.
    pub r#ref: Option<k8s_openapi::api::core::v1::ObjectReference>,
}

impl<'de> serde::Deserialize<'de> for TemplateInstanceObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ref,
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
                            "ref" => Field::Key_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TemplateInstanceObject;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TemplateInstanceObject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ref: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ref => value_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TemplateInstanceObject {
                    r#ref: value_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "TemplateInstanceObject",
            &[
                "ref",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TemplateInstanceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TemplateInstanceObject",
            self.r#ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.r#ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ref", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
