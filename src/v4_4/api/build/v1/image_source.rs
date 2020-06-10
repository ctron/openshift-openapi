// Generated from definition com.github.openshift.api.build.v1.ImageSource

/// ImageSource is used to describe build source that will be extracted from an image or used during a multi stage build. A reference of type ImageStreamTag, ImageStreamImage or DockerImage may be used. A pull secret can be specified to pull the image from an external registry or override the default service account secret if pulling from the internal registry. Image sources can either be used to extract content from an image and place it into the build context along with the repository source, or used directly during a multi-stage container image build to allow content to be copied without overwriting the contents of the repository source (see the 'paths' and 'as' fields).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageSource {
    /// A list of image names that this source will be used in place of during a multi-stage container image build. For instance, a Dockerfile that uses "COPY --from=nginx:latest" will first check for an image source that has "nginx:latest" in this field before attempting to pull directly. If the Dockerfile does not reference an image source it is ignored. This field and paths may both be set, in which case the contents will be used twice.
    pub r#as: Option<Vec<String>>,

    /// from is a reference to an ImageStreamTag, ImageStreamImage, or DockerImage to copy source from.
    pub from: k8s_openapi::api::core::v1::ObjectReference,

    /// paths is a list of source and destination paths to copy from the image. This content will be copied into the build context prior to starting the build. If no paths are set, the build context will not be altered.
    pub paths: Option<Vec<crate::api::build::v1::ImageSourcePath>>,

    /// pullSecret is a reference to a secret to be used to pull the image from a registry If the image is pulled from the OpenShift registry, this field does not need to be set.
    pub pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference>,
}

impl<'de> serde::Deserialize<'de> for ImageSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_as,
            Key_from,
            Key_paths,
            Key_pull_secret,
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
                            "as" => Field::Key_as,
                            "from" => Field::Key_from,
                            "paths" => Field::Key_paths,
                            "pullSecret" => Field::Key_pull_secret,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_as: Option<Vec<String>> = None;
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_paths: Option<Vec<crate::api::build::v1::ImageSourcePath>> = None;
                let mut value_pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_as => value_as = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_paths => value_paths = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pull_secret => value_pull_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageSource {
                    r#as: value_as,
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    paths: value_paths,
                    pull_secret: value_pull_secret,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageSource",
            &[
                "as",
                "from",
                "paths",
                "pullSecret",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageSource",
            1 +
            self.r#as.as_ref().map_or(0, |_| 1) +
            self.paths.as_ref().map_or(0, |_| 1) +
            self.pull_secret.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.r#as {
            serde::ser::SerializeStruct::serialize_field(&mut state, "as", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        if let Some(value) = &self.paths {
            serde::ser::SerializeStruct::serialize_field(&mut state, "paths", value)?;
        }
        if let Some(value) = &self.pull_secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pullSecret", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
