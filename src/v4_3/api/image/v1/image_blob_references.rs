// Generated from definition com.github.openshift.api.image.v1.ImageBlobReferences

/// ImageBlobReferences describes the blob references within an image.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageBlobReferences {
    /// config, if set, is the blob that contains the image config. Some images do not have separate config blobs and this field will be set to nil if so.
    pub config: Option<String>,

    /// imageMissing is true if the image is referenced by the image stream but the image object has been deleted from the API by an administrator. When this field is set, layers and config fields may be empty and callers that depend on the image metadata should consider the image to be unavailable for download or viewing.
    pub image_missing: Option<bool>,

    /// layers is the list of blobs that compose this image, from base layer to top layer. All layers referenced by this array will be defined in the blobs map. Some images may have zero layers.
    pub layers: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for ImageBlobReferences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config,
            Key_image_missing,
            Key_layers,
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
                            "config" => Field::Key_config,
                            "imageMissing" => Field::Key_image_missing,
                            "layers" => Field::Key_layers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageBlobReferences;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageBlobReferences")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_config: Option<String> = None;
                let mut value_image_missing: Option<bool> = None;
                let mut value_layers: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config => value_config = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_missing => value_image_missing = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_layers => value_layers = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageBlobReferences {
                    config: value_config,
                    image_missing: value_image_missing,
                    layers: value_layers,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageBlobReferences",
            &[
                "config",
                "imageMissing",
                "layers",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageBlobReferences {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageBlobReferences",
            self.config.as_ref().map_or(0, |_| 1) +
            self.image_missing.as_ref().map_or(0, |_| 1) +
            self.layers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config {
            serde::ser::SerializeStruct::serialize_field(&mut state, "config", value)?;
        }
        if let Some(value) = &self.image_missing {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageMissing", value)?;
        }
        if let Some(value) = &self.layers {
            serde::ser::SerializeStruct::serialize_field(&mut state, "layers", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
