// Generated from definition com.github.openshift.api.apps.v1.DeploymentTriggerImageChangeParams

/// DeploymentTriggerImageChangeParams represents the parameters to the ImageChange trigger.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentTriggerImageChangeParams {
    /// Automatic means that the detection of a new tag value should result in an image update inside the pod template.
    pub automatic: Option<bool>,

    /// ContainerNames is used to restrict tag updates to the specified set of container names in a pod. If multiple triggers point to the same containers, the resulting behavior is undefined. Future API versions will make this a validation error. If ContainerNames does not point to a valid container, the trigger will be ignored. Future API versions will make this a validation error.
    pub container_names: Option<Vec<String>>,

    /// From is a reference to an image stream tag to watch for changes. From.Name is the only required subfield - if From.Namespace is blank, the namespace of the current deployment trigger will be used.
    pub from: k8s_openapi::api::core::v1::ObjectReference,

    /// LastTriggeredImage is the last image to be triggered.
    pub last_triggered_image: Option<String>,
}

impl<'de> serde::Deserialize<'de> for DeploymentTriggerImageChangeParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_automatic,
            Key_container_names,
            Key_from,
            Key_last_triggered_image,
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
                            "automatic" => Field::Key_automatic,
                            "containerNames" => Field::Key_container_names,
                            "from" => Field::Key_from,
                            "lastTriggeredImage" => Field::Key_last_triggered_image,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentTriggerImageChangeParams;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentTriggerImageChangeParams")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_automatic: Option<bool> = None;
                let mut value_container_names: Option<Vec<String>> = None;
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_last_triggered_image: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_automatic => value_automatic = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_container_names => value_container_names = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_last_triggered_image => value_last_triggered_image = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentTriggerImageChangeParams {
                    automatic: value_automatic,
                    container_names: value_container_names,
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    last_triggered_image: value_last_triggered_image,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentTriggerImageChangeParams",
            &[
                "automatic",
                "containerNames",
                "from",
                "lastTriggeredImage",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentTriggerImageChangeParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentTriggerImageChangeParams",
            1 +
            self.automatic.as_ref().map_or(0, |_| 1) +
            self.container_names.as_ref().map_or(0, |_| 1) +
            self.last_triggered_image.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.automatic {
            serde::ser::SerializeStruct::serialize_field(&mut state, "automatic", value)?;
        }
        if let Some(value) = &self.container_names {
            serde::ser::SerializeStruct::serialize_field(&mut state, "containerNames", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        if let Some(value) = &self.last_triggered_image {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lastTriggeredImage", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
