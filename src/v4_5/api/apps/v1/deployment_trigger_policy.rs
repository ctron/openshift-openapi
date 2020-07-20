// Generated from definition com.github.openshift.api.apps.v1.DeploymentTriggerPolicy

/// DeploymentTriggerPolicy describes a policy for a single trigger that results in a new deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentTriggerPolicy {
    /// ImageChangeParams represents the parameters for the ImageChange trigger.
    pub image_change_params: Option<crate::api::apps::v1::DeploymentTriggerImageChangeParams>,

    /// Type of the trigger
    pub type_: Option<String>,
}

impl<'de> serde::Deserialize<'de> for DeploymentTriggerPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_image_change_params,
            Key_type_,
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
                            "imageChangeParams" => Field::Key_image_change_params,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentTriggerPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentTriggerPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_image_change_params: Option<crate::api::apps::v1::DeploymentTriggerImageChangeParams> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_image_change_params => value_image_change_params = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentTriggerPolicy {
                    image_change_params: value_image_change_params,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentTriggerPolicy",
            &[
                "imageChangeParams",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentTriggerPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentTriggerPolicy",
            self.image_change_params.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.image_change_params {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageChangeParams", value)?;
        }
        if let Some(value) = &self.type_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
