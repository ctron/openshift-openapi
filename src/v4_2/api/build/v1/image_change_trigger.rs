// Generated from definition com.github.openshift.api.build.v1.ImageChangeTrigger

/// ImageChangeTrigger allows builds to be triggered when an ImageStream changes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageChangeTrigger {
    /// from is a reference to an ImageStreamTag that will trigger a build when updated It is optional. If no From is specified, the From image from the build strategy will be used. Only one ImageChangeTrigger with an empty From reference is allowed in a build configuration.
    pub from: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// lastTriggeredImageID is used internally by the ImageChangeController to save last used image ID for build
    pub last_triggered_image_id: Option<String>,

    /// paused is true if this trigger is temporarily disabled. Optional.
    pub paused: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for ImageChangeTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from,
            Key_last_triggered_image_id,
            Key_paused,
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
                            "lastTriggeredImageID" => Field::Key_last_triggered_image_id,
                            "paused" => Field::Key_paused,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageChangeTrigger;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageChangeTrigger")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_last_triggered_image_id: Option<String> = None;
                let mut value_paused: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from => value_from = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_triggered_image_id => value_last_triggered_image_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_paused => value_paused = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageChangeTrigger {
                    from: value_from,
                    last_triggered_image_id: value_last_triggered_image_id,
                    paused: value_paused,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageChangeTrigger",
            &[
                "from",
                "lastTriggeredImageID",
                "paused",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageChangeTrigger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageChangeTrigger",
            self.from.as_ref().map_or(0, |_| 1) +
            self.last_triggered_image_id.as_ref().map_or(0, |_| 1) +
            self.paused.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.from {
            serde::ser::SerializeStruct::serialize_field(&mut state, "from", value)?;
        }
        if let Some(value) = &self.last_triggered_image_id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lastTriggeredImageID", value)?;
        }
        if let Some(value) = &self.paused {
            serde::ser::SerializeStruct::serialize_field(&mut state, "paused", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
