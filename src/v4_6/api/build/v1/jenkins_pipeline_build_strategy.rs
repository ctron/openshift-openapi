// Generated from definition com.github.openshift.api.build.v1.JenkinsPipelineBuildStrategy

/// JenkinsPipelineBuildStrategy holds parameters specific to a Jenkins Pipeline build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JenkinsPipelineBuildStrategy {
    /// env contains additional environment variables you want to pass into a build pipeline.
    pub env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// Jenkinsfile defines the optional raw contents of a Jenkinsfile which defines a Jenkins pipeline build.
    pub jenkinsfile: Option<String>,

    /// JenkinsfilePath is the optional path of the Jenkinsfile that will be used to configure the pipeline relative to the root of the context (contextDir). If both JenkinsfilePath & Jenkinsfile are both not specified, this defaults to Jenkinsfile in the root of the specified contextDir.
    pub jenkinsfile_path: Option<String>,
}

impl<'de> serde::Deserialize<'de> for JenkinsPipelineBuildStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_env,
            Key_jenkinsfile,
            Key_jenkinsfile_path,
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
                            "env" => Field::Key_env,
                            "jenkinsfile" => Field::Key_jenkinsfile,
                            "jenkinsfilePath" => Field::Key_jenkinsfile_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JenkinsPipelineBuildStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JenkinsPipelineBuildStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_jenkinsfile: Option<String> = None;
                let mut value_jenkinsfile_path: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_env => value_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_jenkinsfile => value_jenkinsfile = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_jenkinsfile_path => value_jenkinsfile_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JenkinsPipelineBuildStrategy {
                    env: value_env,
                    jenkinsfile: value_jenkinsfile,
                    jenkinsfile_path: value_jenkinsfile_path,
                })
            }
        }

        deserializer.deserialize_struct(
            "JenkinsPipelineBuildStrategy",
            &[
                "env",
                "jenkinsfile",
                "jenkinsfilePath",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for JenkinsPipelineBuildStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JenkinsPipelineBuildStrategy",
            self.env.as_ref().map_or(0, |_| 1) +
            self.jenkinsfile.as_ref().map_or(0, |_| 1) +
            self.jenkinsfile_path.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.jenkinsfile {
            serde::ser::SerializeStruct::serialize_field(&mut state, "jenkinsfile", value)?;
        }
        if let Some(value) = &self.jenkinsfile_path {
            serde::ser::SerializeStruct::serialize_field(&mut state, "jenkinsfilePath", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
