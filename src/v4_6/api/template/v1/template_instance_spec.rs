// Generated from definition com.github.openshift.api.template.v1.TemplateInstanceSpec

/// TemplateInstanceSpec describes the desired state of a TemplateInstance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TemplateInstanceSpec {
    /// requester holds the identity of the agent requesting the template instantiation.
    pub requester: crate::api::template::v1::TemplateInstanceRequester,

    /// secret is a reference to a Secret object containing the necessary template parameters.
    pub secret: Option<k8s_openapi::api::core::v1::LocalObjectReference>,

    /// template is a full copy of the template for instantiation.
    pub template: crate::api::template::v1::Template,
}

impl<'de> serde::Deserialize<'de> for TemplateInstanceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_requester,
            Key_secret,
            Key_template,
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
                            "requester" => Field::Key_requester,
                            "secret" => Field::Key_secret,
                            "template" => Field::Key_template,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TemplateInstanceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TemplateInstanceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_requester: Option<crate::api::template::v1::TemplateInstanceRequester> = None;
                let mut value_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;
                let mut value_template: Option<crate::api::template::v1::Template> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_requester => value_requester = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_secret => value_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TemplateInstanceSpec {
                    requester: value_requester.ok_or_else(|| serde::de::Error::missing_field("requester"))?,
                    secret: value_secret,
                    template: value_template.ok_or_else(|| serde::de::Error::missing_field("template"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TemplateInstanceSpec",
            &[
                "requester",
                "secret",
                "template",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TemplateInstanceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TemplateInstanceSpec",
            2 +
            self.secret.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "requester", &self.requester)?;
        if let Some(value) = &self.secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        serde::ser::SerializeStruct::end(state)
    }
}
