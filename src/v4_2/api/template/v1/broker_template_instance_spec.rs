// Generated from definition com.github.openshift.api.template.v1.BrokerTemplateInstanceSpec

/// BrokerTemplateInstanceSpec describes the state of a BrokerTemplateInstance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BrokerTemplateInstanceSpec {
    /// bindingids is a list of 'binding_id's provided during successive bind calls to the template service broker.
    pub binding_i_ds: Option<Vec<String>>,

    /// secret is a reference to a Secret object residing in a namespace, containing the necessary template parameters.
    pub secret: k8s_openapi::api::core::v1::ObjectReference,

    /// templateinstance is a reference to a TemplateInstance object residing in a namespace.
    pub template_instance: k8s_openapi::api::core::v1::ObjectReference,
}

impl<'de> serde::Deserialize<'de> for BrokerTemplateInstanceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_binding_i_ds,
            Key_secret,
            Key_template_instance,
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
                            "bindingIDs" => Field::Key_binding_i_ds,
                            "secret" => Field::Key_secret,
                            "templateInstance" => Field::Key_template_instance,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BrokerTemplateInstanceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BrokerTemplateInstanceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_binding_i_ds: Option<Vec<String>> = None;
                let mut value_secret: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_template_instance: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_binding_i_ds => value_binding_i_ds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_template_instance => value_template_instance = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BrokerTemplateInstanceSpec {
                    binding_i_ds: value_binding_i_ds,
                    secret: value_secret.ok_or_else(|| serde::de::Error::missing_field("secret"))?,
                    template_instance: value_template_instance.ok_or_else(|| serde::de::Error::missing_field("templateInstance"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BrokerTemplateInstanceSpec",
            &[
                "bindingIDs",
                "secret",
                "templateInstance",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BrokerTemplateInstanceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BrokerTemplateInstanceSpec",
            2 +
            self.binding_i_ds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.binding_i_ds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "bindingIDs", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "secret", &self.secret)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "templateInstance", &self.template_instance)?;
        serde::ser::SerializeStruct::end(state)
    }
}
