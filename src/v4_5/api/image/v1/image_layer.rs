// Generated from definition com.github.openshift.api.image.v1.ImageLayer

/// ImageLayer represents a single layer of the image. Some images may have multiple layers. Some may have none.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageLayer {
    /// MediaType of the referenced object.
    pub media_type: String,

    /// Name of the layer as defined by the underlying store.
    pub name: String,

    /// Size of the layer in bytes as defined by the underlying store.
    pub size: i64,
}

impl<'de> serde::Deserialize<'de> for ImageLayer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_media_type,
            Key_name,
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
                            "name" => Field::Key_name,
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
            type Value = ImageLayer;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageLayer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_media_type: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_size: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_media_type => value_media_type = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_size => value_size = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageLayer {
                    media_type: value_media_type.ok_or_else(|| serde::de::Error::missing_field("mediaType"))?,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    size: value_size.ok_or_else(|| serde::de::Error::missing_field("size"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageLayer",
            &[
                "mediaType",
                "name",
                "size",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageLayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageLayer",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "mediaType", &self.media_type)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "size", &self.size)?;
        serde::ser::SerializeStruct::end(state)
    }
}
