// Generated from definition com.github.openshift.api.build.v1.BuildConfigSpec

/// BuildConfigSpec describes when and how builds are created
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildConfigSpec {
    /// completionDeadlineSeconds is an optional duration in seconds, counted from the time when a build pod gets scheduled in the system, that the build may be active on a node before the system actively tries to terminate the build; value must be positive integer
    pub completion_deadline_seconds: Option<i64>,

    /// failedBuildsHistoryLimit is the number of old failed builds to retain. If not specified, all failed builds are retained.
    pub failed_builds_history_limit: Option<i32>,

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

    /// RunPolicy describes how the new build created from this build configuration will be scheduled for execution. This is optional, if not specified we default to "Serial".
    pub run_policy: Option<String>,

    /// serviceAccount is the name of the ServiceAccount to use to run the pod created by this build. The pod will be allowed to use secrets referenced by the ServiceAccount
    pub service_account: Option<String>,

    /// source describes the SCM in use.
    pub source: Option<crate::api::build::v1::BuildSource>,

    /// strategy defines how to perform a build.
    pub strategy: crate::api::build::v1::BuildStrategy,

    /// successfulBuildsHistoryLimit is the number of old successful builds to retain. If not specified, all successful builds are retained.
    pub successful_builds_history_limit: Option<i32>,

    /// triggers determine how new Builds can be launched from a BuildConfig. If no triggers are defined, a new build can only occur as a result of an explicit client build creation.
    pub triggers: Vec<crate::api::build::v1::BuildTriggerPolicy>,
}

impl<'de> serde::Deserialize<'de> for BuildConfigSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_completion_deadline_seconds,
            Key_failed_builds_history_limit,
            Key_node_selector,
            Key_output,
            Key_post_commit,
            Key_resources,
            Key_revision,
            Key_run_policy,
            Key_service_account,
            Key_source,
            Key_strategy,
            Key_successful_builds_history_limit,
            Key_triggers,
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
                            "failedBuildsHistoryLimit" => Field::Key_failed_builds_history_limit,
                            "nodeSelector" => Field::Key_node_selector,
                            "output" => Field::Key_output,
                            "postCommit" => Field::Key_post_commit,
                            "resources" => Field::Key_resources,
                            "revision" => Field::Key_revision,
                            "runPolicy" => Field::Key_run_policy,
                            "serviceAccount" => Field::Key_service_account,
                            "source" => Field::Key_source,
                            "strategy" => Field::Key_strategy,
                            "successfulBuildsHistoryLimit" => Field::Key_successful_builds_history_limit,
                            "triggers" => Field::Key_triggers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildConfigSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildConfigSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_completion_deadline_seconds: Option<i64> = None;
                let mut value_failed_builds_history_limit: Option<i32> = None;
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_output: Option<crate::api::build::v1::BuildOutput> = None;
                let mut value_post_commit: Option<crate::api::build::v1::BuildPostCommitSpec> = None;
                let mut value_resources: Option<k8s_openapi::api::core::v1::ResourceRequirements> = None;
                let mut value_revision: Option<crate::api::build::v1::SourceRevision> = None;
                let mut value_run_policy: Option<String> = None;
                let mut value_service_account: Option<String> = None;
                let mut value_source: Option<crate::api::build::v1::BuildSource> = None;
                let mut value_strategy: Option<crate::api::build::v1::BuildStrategy> = None;
                let mut value_successful_builds_history_limit: Option<i32> = None;
                let mut value_triggers: Option<Vec<crate::api::build::v1::BuildTriggerPolicy>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_completion_deadline_seconds => value_completion_deadline_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed_builds_history_limit => value_failed_builds_history_limit = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_output => value_output = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_post_commit => value_post_commit = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision => value_revision = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_policy => value_run_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account => value_service_account = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source => value_source = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_successful_builds_history_limit => value_successful_builds_history_limit = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_triggers => value_triggers = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildConfigSpec {
                    completion_deadline_seconds: value_completion_deadline_seconds,
                    failed_builds_history_limit: value_failed_builds_history_limit,
                    node_selector: value_node_selector.ok_or_else(|| serde::de::Error::missing_field("nodeSelector"))?,
                    output: value_output,
                    post_commit: value_post_commit,
                    resources: value_resources,
                    revision: value_revision,
                    run_policy: value_run_policy,
                    service_account: value_service_account,
                    source: value_source,
                    strategy: value_strategy.ok_or_else(|| serde::de::Error::missing_field("strategy"))?,
                    successful_builds_history_limit: value_successful_builds_history_limit,
                    triggers: value_triggers.ok_or_else(|| serde::de::Error::missing_field("triggers"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildConfigSpec",
            &[
                "completionDeadlineSeconds",
                "failedBuildsHistoryLimit",
                "nodeSelector",
                "output",
                "postCommit",
                "resources",
                "revision",
                "runPolicy",
                "serviceAccount",
                "source",
                "strategy",
                "successfulBuildsHistoryLimit",
                "triggers",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildConfigSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildConfigSpec",
            3 +
            self.completion_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.failed_builds_history_limit.as_ref().map_or(0, |_| 1) +
            self.output.as_ref().map_or(0, |_| 1) +
            self.post_commit.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.revision.as_ref().map_or(0, |_| 1) +
            self.run_policy.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.source.as_ref().map_or(0, |_| 1) +
            self.successful_builds_history_limit.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.completion_deadline_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "completionDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.failed_builds_history_limit {
            serde::ser::SerializeStruct::serialize_field(&mut state, "failedBuildsHistoryLimit", value)?;
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
        if let Some(value) = &self.run_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "runPolicy", value)?;
        }
        if let Some(value) = &self.service_account {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccount", value)?;
        }
        if let Some(value) = &self.source {
            serde::ser::SerializeStruct::serialize_field(&mut state, "source", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", &self.strategy)?;
        if let Some(value) = &self.successful_builds_history_limit {
            serde::ser::SerializeStruct::serialize_field(&mut state, "successfulBuildsHistoryLimit", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "triggers", &self.triggers)?;
        serde::ser::SerializeStruct::end(state)
    }
}
