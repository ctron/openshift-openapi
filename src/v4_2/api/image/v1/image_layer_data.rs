// Generated from definition com.github.openshift.api.image.v1.ImageLayerData

/// ImageLayerData contains metadata about an image layer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageLayerData {
    /// MediaType of the referenced object.
    pub media_type: String,

    /// Size of the layer in bytes as defined by the underlying store. This field is optional if the necessary information about size is not available.
    pub size: i64,
}

impl<'de> serde::Deserialize<'de> for ImageLayerData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_media_type,
            Key_size,
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
                            "mediaType" => Field::Key_media_type,
                            "size" => Field::Key_size,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageLayerData;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageLayerData")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_media_type: Option<String> = None;
                let mut value_size: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_media_type => value_media_type = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_size => value_size = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageLayerData {
                    media_type: value_media_type.ok_or_else(|| serde::de::Error::missing_field("mediaType"))?,
                    size: value_size.ok_or_else(|| serde::de::Error::missing_field("size"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageLayerData",
            &[
                "mediaType",
                "size",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageLayerData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageLayerData",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "mediaType", &self.media_type)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "size", &self.size)?;
        serde::ser::SerializeStruct::end(state)
    }
}
