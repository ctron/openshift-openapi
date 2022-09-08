// Generated from definition com.github.openshift.api.image.v1.ImageImportStatus

/// ImageImportStatus describes the result of an image import.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageImportStatus {
    /// Image is the metadata of that image, if the image was located
    pub image: Option<crate::api::image::v1::Image>,

    /// Status is the status of the image import, including errors encountered while retrieving the image
    pub status: k8s_openapi::apimachinery::pkg::apis::meta::v1::Status,

    /// Tag is the tag this image was located under, if any
    pub tag: Option<String>,
}

impl<'de> serde::Deserialize<'de> for ImageImportStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_image,
            Key_status,
            Key_tag,
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
                            "image" => Field::Key_image,
                            "status" => Field::Key_status,
                            "tag" => Field::Key_tag,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageImportStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageImportStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_image: Option<crate::api::image::v1::Image> = None;
                let mut value_status: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Status> = None;
                let mut value_tag: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_image => value_image = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_tag => value_tag = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageImportStatus {
                    image: value_image,
                    status: value_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    tag: value_tag,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageImportStatus",
            &[
                "image",
                "status",
                "tag",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageImportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageImportStatus",
            1 +
            self.image.as_ref().map_or(0, |_| 1) +
            self.tag.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.image {
            serde::ser::SerializeStruct::serialize_field(&mut state, "image", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        if let Some(value) = &self.tag {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tag", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
