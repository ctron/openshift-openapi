// Generated from definition com.github.openshift.api.apps.v1.DeploymentCauseImageTrigger

/// DeploymentCauseImageTrigger represents details about the cause of a deployment originating from an image change trigger
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentCauseImageTrigger {
    /// From is a reference to the changed object which triggered a deployment. The field may have the kinds DockerImage, ImageStreamTag, or ImageStreamImage.
    pub from: k8s_openapi::api::core::v1::ObjectReference,
}

impl<'de> serde::Deserialize<'de> for DeploymentCauseImageTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from,
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
                            "from" => Field::Key_from,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentCauseImageTrigger;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentCauseImageTrigger")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentCauseImageTrigger {
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentCauseImageTrigger",
            &[
                "from",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentCauseImageTrigger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentCauseImageTrigger",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        serde::ser::SerializeStruct::end(state)
    }
}
