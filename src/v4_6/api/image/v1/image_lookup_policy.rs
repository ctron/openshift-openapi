// Generated from definition com.github.openshift.api.image.v1.ImageLookupPolicy

/// ImageLookupPolicy describes how an image stream can be used to override the image references used by pods, builds, and other resources in a namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageLookupPolicy {
    /// local will change the docker short image references (like "mysql" or "php:latest") on objects in this namespace to the image ID whenever they match this image stream, instead of reaching out to a remote registry. The name will be fully qualified to an image ID if found. The tag's referencePolicy is taken into account on the replaced value. Only works within the current namespace.
    pub local: bool,
}

impl<'de> serde::Deserialize<'de> for ImageLookupPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_local,
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
                            "local" => Field::Key_local,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageLookupPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageLookupPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_local: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_local => value_local = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageLookupPolicy {
                    local: value_local.ok_or_else(|| serde::de::Error::missing_field("local"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageLookupPolicy",
            &[
                "local",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageLookupPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageLookupPolicy",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "local", &self.local)?;
        serde::ser::SerializeStruct::end(state)
    }
}
