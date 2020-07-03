// Generated from definition com.github.openshift.api.apps.v1.DeploymentCause

/// DeploymentCause captures information about a particular cause of a deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentCause {
    /// ImageTrigger contains the image trigger details, if this trigger was fired based on an image change
    pub image_trigger: Option<crate::api::apps::v1::DeploymentCauseImageTrigger>,

    /// Type of the trigger that resulted in the creation of a new deployment
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for DeploymentCause {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_image_trigger,
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
                            "imageTrigger" => Field::Key_image_trigger,
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
            type Value = DeploymentCause;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentCause")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_image_trigger: Option<crate::api::apps::v1::DeploymentCauseImageTrigger> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_image_trigger => value_image_trigger = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentCause {
                    image_trigger: value_image_trigger,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentCause",
            &[
                "imageTrigger",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentCause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentCause",
            1 +
            self.image_trigger.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.image_trigger {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageTrigger", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
