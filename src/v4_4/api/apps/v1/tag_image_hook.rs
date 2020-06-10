// Generated from definition com.github.openshift.api.apps.v1.TagImageHook

/// TagImageHook is a request to tag the image in a particular container onto an ImageStreamTag.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagImageHook {
    /// ContainerName is the name of a container in the deployment config whose image value will be used as the source of the tag. If there is only a single container this value will be defaulted to the name of that container.
    pub container_name: String,

    /// To is the target ImageStreamTag to set the container's image onto.
    pub to: k8s_openapi::api::core::v1::ObjectReference,
}

impl<'de> serde::Deserialize<'de> for TagImageHook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_name,
            Key_to,
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
                            "containerName" => Field::Key_container_name,
                            "to" => Field::Key_to,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TagImageHook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TagImageHook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_container_name: Option<String> = None;
                let mut value_to: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_name => value_container_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_to => value_to = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TagImageHook {
                    container_name: value_container_name.ok_or_else(|| serde::de::Error::missing_field("containerName"))?,
                    to: value_to.ok_or_else(|| serde::de::Error::missing_field("to"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TagImageHook",
            &[
                "containerName",
                "to",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TagImageHook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TagImageHook",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "containerName", &self.container_name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "to", &self.to)?;
        serde::ser::SerializeStruct::end(state)
    }
}
