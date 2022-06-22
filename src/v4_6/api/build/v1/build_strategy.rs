// Generated from definition com.github.openshift.api.build.v1.BuildStrategy

/// BuildStrategy contains the details of how to perform a build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildStrategy {
    /// customStrategy holds the parameters to the Custom build strategy
    pub custom_strategy: Option<crate::api::build::v1::CustomBuildStrategy>,

    /// dockerStrategy holds the parameters to the container image build strategy.
    pub docker_strategy: Option<crate::api::build::v1::DockerBuildStrategy>,

    /// JenkinsPipelineStrategy holds the parameters to the Jenkins Pipeline build strategy.
    pub jenkins_pipeline_strategy: Option<crate::api::build::v1::JenkinsPipelineBuildStrategy>,

    /// sourceStrategy holds the parameters to the Source build strategy.
    pub source_strategy: Option<crate::api::build::v1::SourceBuildStrategy>,

    /// type is the kind of build strategy.
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for BuildStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_custom_strategy,
            Key_docker_strategy,
            Key_jenkins_pipeline_strategy,
            Key_source_strategy,
            Key_type_,
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
                            "customStrategy" => Field::Key_custom_strategy,
                            "dockerStrategy" => Field::Key_docker_strategy,
                            "jenkinsPipelineStrategy" => Field::Key_jenkins_pipeline_strategy,
                            "sourceStrategy" => Field::Key_source_strategy,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_custom_strategy: Option<crate::api::build::v1::CustomBuildStrategy> = None;
                let mut value_docker_strategy: Option<crate::api::build::v1::DockerBuildStrategy> = None;
                let mut value_jenkins_pipeline_strategy: Option<crate::api::build::v1::JenkinsPipelineBuildStrategy> = None;
                let mut value_source_strategy: Option<crate::api::build::v1::SourceBuildStrategy> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_custom_strategy => value_custom_strategy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_strategy => value_docker_strategy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_jenkins_pipeline_strategy => value_jenkins_pipeline_strategy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source_strategy => value_source_strategy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildStrategy {
                    custom_strategy: value_custom_strategy,
                    docker_strategy: value_docker_strategy,
                    jenkins_pipeline_strategy: value_jenkins_pipeline_strategy,
                    source_strategy: value_source_strategy,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildStrategy",
            &[
                "customStrategy",
                "dockerStrategy",
                "jenkinsPipelineStrategy",
                "sourceStrategy",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildStrategy",
            1 +
            self.custom_strategy.as_ref().map_or(0, |_| 1) +
            self.docker_strategy.as_ref().map_or(0, |_| 1) +
            self.jenkins_pipeline_strategy.as_ref().map_or(0, |_| 1) +
            self.source_strategy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.custom_strategy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "customStrategy", value)?;
        }
        if let Some(value) = &self.docker_strategy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerStrategy", value)?;
        }
        if let Some(value) = &self.jenkins_pipeline_strategy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "jenkinsPipelineStrategy", value)?;
        }
        if let Some(value) = &self.source_strategy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "sourceStrategy", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
