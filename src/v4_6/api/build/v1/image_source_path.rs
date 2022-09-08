// Generated from definition com.github.openshift.api.build.v1.ImageSourcePath

/// ImageSourcePath describes a path to be copied from a source image and its destination within the build directory.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageSourcePath {
    /// destinationDir is the relative directory within the build directory where files copied from the image are placed.
    pub destination_dir: String,

    /// sourcePath is the absolute path of the file or directory inside the image to copy to the build directory.  If the source path ends in /. then the content of the directory will be copied, but the directory itself will not be created at the destination.
    pub source_path: String,
}

impl<'de> serde::Deserialize<'de> for ImageSourcePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_destination_dir,
            Key_source_path,
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
                            "destinationDir" => Field::Key_destination_dir,
                            "sourcePath" => Field::Key_source_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageSourcePath;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageSourcePath")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_destination_dir: Option<String> = None;
                let mut value_source_path: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_destination_dir => value_destination_dir = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_source_path => value_source_path = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageSourcePath {
                    destination_dir: value_destination_dir.ok_or_else(|| serde::de::Error::missing_field("destinationDir"))?,
                    source_path: value_source_path.ok_or_else(|| serde::de::Error::missing_field("sourcePath"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageSourcePath",
            &[
                "destinationDir",
                "sourcePath",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageSourcePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageSourcePath",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "destinationDir", &self.destination_dir)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "sourcePath", &self.source_path)?;
        serde::ser::SerializeStruct::end(state)
    }
}
