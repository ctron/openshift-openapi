// Generated from definition com.github.openshift.api.image.v1.ImageStreamStatus

/// ImageStreamStatus contains information about the state of this image stream.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageStreamStatus {
    /// DockerImageRepository represents the effective location this stream may be accessed at. May be empty until the server determines where the repository is located
    pub docker_image_repository: String,

    /// PublicDockerImageRepository represents the public location from where the image can be pulled outside the cluster. This field may be empty if the administrator has not exposed the integrated registry externally.
    pub public_docker_image_repository: Option<String>,

    /// Tags are a historical record of images associated with each tag. The first entry in the TagEvent array is the currently tagged image.
    pub tags: Option<Vec<crate::api::image::v1::NamedTagEventList>>,
}

impl<'de> serde::Deserialize<'de> for ImageStreamStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_docker_image_repository,
            Key_public_docker_image_repository,
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
                            "publicDockerImageRepository" => Field::Key_public_docker_image_repository,
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
            type Value = ImageStreamStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageStreamStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_docker_image_repository: Option<String> = None;
                let mut value_public_docker_image_repository: Option<String> = None;
                let mut value_tags: Option<Vec<crate::api::image::v1::NamedTagEventList>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_docker_image_repository => value_docker_image_repository = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_public_docker_image_repository => value_public_docker_image_repository = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tags => value_tags = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageStreamStatus {
                    docker_image_repository: value_docker_image_repository.ok_or_else(|| serde::de::Error::missing_field("dockerImageRepository"))?,
                    public_docker_image_repository: value_public_docker_image_repository,
                    tags: value_tags,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageStreamStatus",
            &[
                "dockerImageRepository",
                "publicDockerImageRepository",
                "tags",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageStreamStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageStreamStatus",
            1 +
            self.public_docker_image_repository.as_ref().map_or(0, |_| 1) +
            self.tags.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageRepository", &self.docker_image_repository)?;
        if let Some(value) = &self.public_docker_image_repository {
            serde::ser::SerializeStruct::serialize_field(&mut state, "publicDockerImageRepository", value)?;
        }
        if let Some(value) = &self.tags {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tags", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
