// Generated from definition com.github.openshift.api.build.v1.ImageChangeCause

/// ImageChangeCause contains information about the image that triggered a build
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageChangeCause {
    /// fromRef contains detailed information about an image that triggered a build.
    pub from_ref: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// imageID is the ID of the image that triggered a a new build.
    pub image_id: Option<String>,
}

impl<'de> serde::Deserialize<'de> for ImageChangeCause {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from_ref,
            Key_image_id,
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
                            "fromRef" => Field::Key_from_ref,
                            "imageID" => Field::Key_image_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageChangeCause;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageChangeCause")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_from_ref: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_image_id: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from_ref => value_from_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_id => value_image_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageChangeCause {
                    from_ref: value_from_ref,
                    image_id: value_image_id,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageChangeCause",
            &[
                "fromRef",
                "imageID",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageChangeCause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageChangeCause",
            self.from_ref.as_ref().map_or(0, |_| 1) +
            self.image_id.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.from_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fromRef", value)?;
        }
        if let Some(value) = &self.image_id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageID", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
