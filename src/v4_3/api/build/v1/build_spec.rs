// Generated from definition com.github.openshift.api.build.v1.BuildSpec

/// BuildSpec has the information to represent a build and also additional information about a build
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildSpec {
    /// completionDeadlineSeconds is an optional duration in seconds, counted from the time when a build pod gets scheduled in the system, that the build may be active on a node before the system actively tries to terminate the build; value must be positive integer
    pub completion_deadline_seconds: Option<i64>,

    /// nodeSelector is a selector which must be true for the build pod to fit on a node If nil, it can be overridden by default build nodeselector values for the cluster. If set to an empty map or a map with any values, default build nodeselector values are ignored.
    pub node_selector: std::collections::BTreeMap<String, String>,

    /// output describes the container image the Strategy should produce.
    pub output: Option<crate::api::build::v1::BuildOutput>,

    /// postCommit is a build hook executed after the build output image is committed, before it is pushed to a registry.
    pub post_commit: Option<crate::api::build::v1::BuildPostCommitSpec>,

    /// resources computes resource requirements to execute the build.
    pub resources: Option<k8s_openapi::api::core::v1::ResourceRequirements>,

    /// revision is the information from the source for a specific repo snapshot. This is optional.
    pub revision: Option<crate::api::build::v1::SourceRevision>,

    /// serviceAccount is the name of the ServiceAccount to use to run the pod created by this build. The pod will be allowed to use secrets referenced by the ServiceAccount
    pub service_account: Option<String>,

    /// source describes the SCM in use.
    pub source: Option<crate::api::build::v1::BuildSource>,

    /// strategy defines how to perform a build.
    pub strategy: crate::api::build::v1::BuildStrategy,

    /// triggeredBy describes which triggers started the most recent update to the build configuration and contains information about those triggers.
    pub triggered_by: Vec<crate::api::build::v1::BuildTriggerCause>,
}

impl<'de> serde::Deserialize<'de> for BuildSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_completion_deadline_seconds,
            Key_node_selector,
            Key_output,
            Key_post_commit,
            Key_resources,
            Key_revision,
            Key_service_account,
            Key_source,
            Key_strategy,
            Key_triggered_by,
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
                            "completionDeadlineSeconds" => Field::Key_completion_deadline_seconds,
                            "nodeSelector" => Field::Key_node_selector,
                            "output" => Field::Key_output,
                            "postCommit" => Field::Key_post_commit,
                            "resources" => Field::Key_resources,
                            "revision" => Field::Key_revision,
                            "serviceAccount" => Field::Key_service_account,
                            "source" => Field::Key_source,
                            "strategy" => Field::Key_strategy,
                            "triggeredBy" => Field::Key_triggered_by,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_completion_deadline_seconds: Option<i64> = None;
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_output: Option<crate::api::build::v1::BuildOutput> = None;
                let mut value_post_commit: Option<crate::api::build::v1::BuildPostCommitSpec> = None;
                let mut value_resources: Option<k8s_openapi::api::core::v1::ResourceRequirements> = None;
                let mut value_revision: Option<crate::api::build::v1::SourceRevision> = None;
                let mut value_service_account: Option<String> = None;
                let mut value_source: Option<crate::api::build::v1::BuildSource> = None;
                let mut value_strategy: Option<crate::api::build::v1::BuildStrategy> = None;
                let mut value_triggered_by: Option<Vec<crate::api::build::v1::BuildTriggerCause>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_completion_deadline_seconds => value_completion_deadline_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_output => value_output = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_post_commit => value_post_commit = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision => value_revision = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account => value_service_account = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source => value_source = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_triggered_by => value_triggered_by = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildSpec {
                    completion_deadline_seconds: value_completion_deadline_seconds,
                    node_selector: value_node_selector.ok_or_else(|| serde::de::Error::missing_field("nodeSelector"))?,
                    output: value_output,
                    post_commit: value_post_commit,
                    resources: value_resources,
                    revision: value_revision,
                    service_account: value_service_account,
                    source: value_source,
                    strategy: value_strategy.ok_or_else(|| serde::de::Error::missing_field("strategy"))?,
                    triggered_by: value_triggered_by.ok_or_else(|| serde::de::Error::missing_field("triggeredBy"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildSpec",
            &[
                "completionDeadlineSeconds",
                "nodeSelector",
                "output",
                "postCommit",
                "resources",
                "revision",
                "serviceAccount",
                "source",
                "strategy",
                "triggeredBy",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildSpec",
            3 +
            self.completion_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.output.as_ref().map_or(0, |_| 1) +
            self.post_commit.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.revision.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.source.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.completion_deadline_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "completionDeadlineSeconds", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", &self.node_selector)?;
        if let Some(value) = &self.output {
            serde::ser::SerializeStruct::serialize_field(&mut state, "output", value)?;
        }
        if let Some(value) = &self.post_commit {
            serde::ser::SerializeStruct::serialize_field(&mut state, "postCommit", value)?;
        }
        if let Some(value) = &self.resources {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.revision {
            serde::ser::SerializeStruct::serialize_field(&mut state, "revision", value)?;
        }
        if let Some(value) = &self.service_account {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccount", value)?;
        }
        if let Some(value) = &self.source {
            serde::ser::SerializeStruct::serialize_field(&mut state, "source", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", &self.strategy)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "triggeredBy", &self.triggered_by)?;
        serde::ser::SerializeStruct::end(state)
    }
}
