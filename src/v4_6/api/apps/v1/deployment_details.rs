// Generated from definition com.github.openshift.api.apps.v1.DeploymentDetails

/// DeploymentDetails captures information about the causes of a deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentDetails {
    /// Causes are extended data associated with all the causes for creating a new deployment
    pub causes: Vec<crate::api::apps::v1::DeploymentCause>,

    /// Message is the user specified change message, if this deployment was triggered manually by the user
    pub message: Option<String>,
}

impl<'de> serde::Deserialize<'de> for DeploymentDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_causes,
            Key_message,
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
                            "causes" => Field::Key_causes,
                            "message" => Field::Key_message,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentDetails;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentDetails")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_causes: Option<Vec<crate::api::apps::v1::DeploymentCause>> = None;
                let mut value_message: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_causes => value_causes = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentDetails {
                    causes: value_causes.ok_or_else(|| serde::de::Error::missing_field("causes"))?,
                    message: value_message,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentDetails",
            &[
                "causes",
                "message",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentDetails",
            1 +
            self.message.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "causes", &self.causes)?;
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
