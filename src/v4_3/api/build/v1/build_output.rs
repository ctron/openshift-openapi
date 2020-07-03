// Generated from definition com.github.openshift.api.build.v1.BuildOutput

/// BuildOutput is input to a build strategy and describes the container image that the strategy should produce.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildOutput {
    /// imageLabels define a list of labels that are applied to the resulting image. If there are multiple labels with the same name then the last one in the list is used.
    pub image_labels: Option<Vec<crate::api::build::v1::ImageLabel>>,

    /// PushSecret is the name of a Secret that would be used for setting up the authentication for executing the Docker push to authentication enabled Docker Registry (or Docker Hub).
    pub push_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference>,

    /// to defines an optional location to push the output of this build to. Kind must be one of 'ImageStreamTag' or 'DockerImage'. This value will be used to look up a container image repository to push to. In the case of an ImageStreamTag, the ImageStreamTag will be looked for in the namespace of the build unless Namespace is specified.
    pub to: Option<k8s_openapi::api::core::v1::ObjectReference>,
}

impl<'de> serde::Deserialize<'de> for BuildOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_image_labels,
            Key_push_secret,
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
                            "imageLabels" => Field::Key_image_labels,
                            "pushSecret" => Field::Key_push_secret,
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
            type Value = BuildOutput;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildOutput")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_image_labels: Option<Vec<crate::api::build::v1::ImageLabel>> = None;
                let mut value_push_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;
                let mut value_to: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_image_labels => value_image_labels = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_push_secret => value_push_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_to => value_to = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildOutput {
                    image_labels: value_image_labels,
                    push_secret: value_push_secret,
                    to: value_to,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildOutput",
            &[
                "imageLabels",
                "pushSecret",
                "to",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildOutput",
            self.image_labels.as_ref().map_or(0, |_| 1) +
            self.push_secret.as_ref().map_or(0, |_| 1) +
            self.to.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.image_labels {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageLabels", value)?;
        }
        if let Some(value) = &self.push_secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pushSecret", value)?;
        }
        if let Some(value) = &self.to {
            serde::ser::SerializeStruct::serialize_field(&mut state, "to", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
