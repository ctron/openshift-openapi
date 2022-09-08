// Generated from definition com.github.openshift.api.build.v1.BuildStatusOutputTo

/// BuildStatusOutputTo describes the status of the built image with regards to image registry to which it was supposed to be pushed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildStatusOutputTo {
    /// imageDigest is the digest of the built container image. The digest uniquely identifies the image in the registry to which it was pushed.
    ///
    /// Please note that this field may not always be set even if the push completes successfully - e.g. when the registry returns no digest or returns it in a format that the builder doesn't understand.
    pub image_digest: Option<String>,
}

impl<'de> serde::Deserialize<'de> for BuildStatusOutputTo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_image_digest,
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
                            "imageDigest" => Field::Key_image_digest,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildStatusOutputTo;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildStatusOutputTo")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_image_digest: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_image_digest => value_image_digest = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildStatusOutputTo {
                    image_digest: value_image_digest,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildStatusOutputTo",
            &[
                "imageDigest",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildStatusOutputTo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildStatusOutputTo",
            self.image_digest.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.image_digest {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageDigest", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
