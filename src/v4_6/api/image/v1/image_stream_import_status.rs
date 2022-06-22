// Generated from definition com.github.openshift.api.image.v1.ImageStreamImportStatus

/// ImageStreamImportStatus contains information about the status of an image stream import.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageStreamImportStatus {
    /// Images is set with the result of importing spec.images
    pub images: Option<Vec<crate::api::image::v1::ImageImportStatus>>,

    /// Import is the image stream that was successfully updated or created when 'to' was set.
    pub import: Option<crate::api::image::v1::ImageStream>,

    /// Repository is set if spec.repository was set to the outcome of the import
    pub repository: Option<crate::api::image::v1::RepositoryImportStatus>,
}

impl<'de> serde::Deserialize<'de> for ImageStreamImportStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_images,
            Key_import,
            Key_repository,
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
                            "images" => Field::Key_images,
                            "import" => Field::Key_import,
                            "repository" => Field::Key_repository,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageStreamImportStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageStreamImportStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_images: Option<Vec<crate::api::image::v1::ImageImportStatus>> = None;
                let mut value_import: Option<crate::api::image::v1::ImageStream> = None;
                let mut value_repository: Option<crate::api::image::v1::RepositoryImportStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_images => value_images = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_import => value_import = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_repository => value_repository = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageStreamImportStatus {
                    images: value_images,
                    import: value_import,
                    repository: value_repository,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageStreamImportStatus",
            &[
                "images",
                "import",
                "repository",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageStreamImportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageStreamImportStatus",
            self.images.as_ref().map_or(0, |_| 1) +
            self.import.as_ref().map_or(0, |_| 1) +
            self.repository.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.images {
            serde::ser::SerializeStruct::serialize_field(&mut state, "images", value)?;
        }
        if let Some(value) = &self.import {
            serde::ser::SerializeStruct::serialize_field(&mut state, "import", value)?;
        }
        if let Some(value) = &self.repository {
            serde::ser::SerializeStruct::serialize_field(&mut state, "repository", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
