// Generated from definition com.github.openshift.api.image.v1.TagEvent

/// TagEvent is used by ImageStreamStatus to keep a historical record of images associated with a tag.
#[derive(Clone, Debug, PartialEq)]
pub struct TagEvent {
    /// Created holds the time the TagEvent was created
    pub created: k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,

    /// DockerImageReference is the string that can be used to pull this image
    pub docker_image_reference: String,

    /// Generation is the spec tag generation that resulted in this tag being updated
    pub generation: i64,

    /// Image is the image
    pub image: String,
}

impl<'de> serde::Deserialize<'de> for TagEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_created,
            Key_docker_image_reference,
            Key_generation,
            Key_image,
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
                            "created" => Field::Key_created,
                            "dockerImageReference" => Field::Key_docker_image_reference,
                            "generation" => Field::Key_generation,
                            "image" => Field::Key_image,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TagEvent;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TagEvent")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_created: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_docker_image_reference: Option<String> = None;
                let mut value_generation: Option<i64> = None;
                let mut value_image: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_created => value_created = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_docker_image_reference => value_docker_image_reference = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_generation => value_generation = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_image => value_image = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TagEvent {
                    created: value_created.ok_or_else(|| serde::de::Error::missing_field("created"))?,
                    docker_image_reference: value_docker_image_reference.ok_or_else(|| serde::de::Error::missing_field("dockerImageReference"))?,
                    generation: value_generation.ok_or_else(|| serde::de::Error::missing_field("generation"))?,
                    image: value_image.ok_or_else(|| serde::de::Error::missing_field("image"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TagEvent",
            &[
                "created",
                "dockerImageReference",
                "generation",
                "image",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TagEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TagEvent",
            4,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "created", &self.created)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageReference", &self.docker_image_reference)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "generation", &self.generation)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "image", &self.image)?;
        serde::ser::SerializeStruct::end(state)
    }
}
