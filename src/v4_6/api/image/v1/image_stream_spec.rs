// Generated from definition com.github.openshift.api.image.v1.ImageStreamSpec

/// ImageStreamSpec represents options for ImageStreams.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageStreamSpec {
    /// dockerImageRepository is optional, if specified this stream is backed by a container repository on this server Deprecated: This field is deprecated as of v3.7 and will be removed in a future release. Specify the source for the tags to be imported in each tag via the spec.tags.from reference instead.
    pub docker_image_repository: Option<String>,

    /// lookupPolicy controls how other resources reference images within this namespace.
    pub lookup_policy: Option<crate::api::image::v1::ImageLookupPolicy>,

    /// tags map arbitrary string values to specific image locators
    pub tags: Option<Vec<crate::api::image::v1::TagReference>>,
}

impl<'de> serde::Deserialize<'de> for ImageStreamSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_docker_image_repository,
            Key_lookup_policy,
            Key_tags,
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
                            "dockerImageRepository" => Field::Key_docker_image_repository,
                            "lookupPolicy" => Field::Key_lookup_policy,
                            "tags" => Field::Key_tags,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageStreamSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageStreamSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_docker_image_repository: Option<String> = None;
                let mut value_lookup_policy: Option<crate::api::image::v1::ImageLookupPolicy> = None;
                let mut value_tags: Option<Vec<crate::api::image::v1::TagReference>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_docker_image_repository => value_docker_image_repository = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lookup_policy => value_lookup_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tags => value_tags = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageStreamSpec {
                    docker_image_repository: value_docker_image_repository,
                    lookup_policy: value_lookup_policy,
                    tags: value_tags,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageStreamSpec",
            &[
                "dockerImageRepository",
                "lookupPolicy",
                "tags",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageStreamSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageStreamSpec",
            self.docker_image_repository.as_ref().map_or(0, |_| 1) +
            self.lookup_policy.as_ref().map_or(0, |_| 1) +
            self.tags.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.docker_image_repository {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageRepository", value)?;
        }
        if let Some(value) = &self.lookup_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lookupPolicy", value)?;
        }
        if let Some(value) = &self.tags {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tags", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
