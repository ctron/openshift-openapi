// Generated from definition com.github.openshift.api.image.v1.ImageStreamImportSpec

/// ImageStreamImportSpec defines what images should be imported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageStreamImportSpec {
    /// Images are a list of individual images to import.
    pub images: Option<Vec<crate::api::image::v1::ImageImportSpec>>,

    /// Import indicates whether to perform an import - if so, the specified tags are set on the spec and status of the image stream defined by the type meta.
    pub import: bool,

    /// Repository is an optional import of an entire container image repository. A maximum limit on the number of tags imported this way is imposed by the server.
    pub repository: Option<crate::api::image::v1::RepositoryImportSpec>,
}

impl<'de> serde::Deserialize<'de> for ImageStreamImportSpec {
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
            type Value = ImageStreamImportSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageStreamImportSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_images: Option<Vec<crate::api::image::v1::ImageImportSpec>> = None;
                let mut value_import: Option<bool> = None;
                let mut value_repository: Option<crate::api::image::v1::RepositoryImportSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_images => value_images = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_import => value_import = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_repository => value_repository = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageStreamImportSpec {
                    images: value_images,
                    import: value_import.ok_or_else(|| serde::de::Error::missing_field("import"))?,
                    repository: value_repository,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageStreamImportSpec",
            &[
                "images",
                "import",
                "repository",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageStreamImportSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageStreamImportSpec",
            1 +
            self.images.as_ref().map_or(0, |_| 1) +
            self.repository.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.images {
            serde::ser::SerializeStruct::serialize_field(&mut state, "images", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "import", &self.import)?;
        if let Some(value) = &self.repository {
            serde::ser::SerializeStruct::serialize_field(&mut state, "repository", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
