// Generated from definition com.github.openshift.api.build.v1.BinaryBuildSource

/// BinaryBuildSource describes a binary file to be used for the Docker and Source build strategies, where the file will be extracted and used as the build source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BinaryBuildSource {
    /// asFile indicates that the provided binary input should be considered a single file within the build input. For example, specifying "webapp.war" would place the provided binary as `/webapp.war` for the builder. If left empty, the Docker and Source build strategies assume this file is a zip, tar, or tar.gz file and extract it as the source. The custom strategy receives this binary as standard input. This filename may not contain slashes or be '..' or '.'.
    pub as_file: Option<String>,
}

impl<'de> serde::Deserialize<'de> for BinaryBuildSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_as_file,
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
                            "asFile" => Field::Key_as_file,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BinaryBuildSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BinaryBuildSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_as_file: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_as_file => value_as_file = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BinaryBuildSource {
                    as_file: value_as_file,
                })
            }
        }

        deserializer.deserialize_struct(
            "BinaryBuildSource",
            &[
                "asFile",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BinaryBuildSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BinaryBuildSource",
            self.as_file.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.as_file {
            serde::ser::SerializeStruct::serialize_field(&mut state, "asFile", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
