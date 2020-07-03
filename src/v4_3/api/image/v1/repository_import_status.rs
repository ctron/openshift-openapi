// Generated from definition com.github.openshift.api.image.v1.RepositoryImportStatus

/// RepositoryImportStatus describes the result of an image repository import
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepositoryImportStatus {
    /// AdditionalTags are tags that exist in the repository but were not imported because a maximum limit of automatic imports was applied.
    pub additional_tags: Option<Vec<String>>,

    /// Images is a list of images successfully retrieved by the import of the repository.
    pub images: Option<Vec<crate::api::image::v1::ImageImportStatus>>,

    /// Status reflects whether any failure occurred during import
    pub status: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Status>,
}

impl<'de> serde::Deserialize<'de> for RepositoryImportStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_additional_tags,
            Key_images,
            Key_status,
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
                            "additionalTags" => Field::Key_additional_tags,
                            "images" => Field::Key_images,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RepositoryImportStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RepositoryImportStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_additional_tags: Option<Vec<String>> = None;
                let mut value_images: Option<Vec<crate::api::image::v1::ImageImportStatus>> = None;
                let mut value_status: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Status> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_additional_tags => value_additional_tags = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_images => value_images = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RepositoryImportStatus {
                    additional_tags: value_additional_tags,
                    images: value_images,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "RepositoryImportStatus",
            &[
                "additionalTags",
                "images",
                "status",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RepositoryImportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RepositoryImportStatus",
            self.additional_tags.as_ref().map_or(0, |_| 1) +
            self.images.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.additional_tags {
            serde::ser::SerializeStruct::serialize_field(&mut state, "additionalTags", value)?;
        }
        if let Some(value) = &self.images {
            serde::ser::SerializeStruct::serialize_field(&mut state, "images", value)?;
        }
        if let Some(value) = &self.status {
            serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
