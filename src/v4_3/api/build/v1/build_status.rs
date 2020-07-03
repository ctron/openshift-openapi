// Generated from definition com.github.openshift.api.build.v1.BuildStatus

/// BuildStatus contains the status of a build
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildStatus {
    /// cancelled describes if a cancel event was triggered for the build.
    pub cancelled: Option<bool>,

    /// completionTimestamp is a timestamp representing the server time when this Build was finished, whether that build failed or succeeded.  It reflects the time at which the Pod running the Build terminated. It is represented in RFC3339 form and is in UTC.
    pub completion_timestamp: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time>,

    /// config is an ObjectReference to the BuildConfig this Build is based on.
    pub config: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// duration contains time.Duration object describing build time.
    pub duration: Option<i64>,

    /// logSnippet is the last few lines of the build log.  This value is only set for builds that failed.
    pub log_snippet: Option<String>,

    /// message is a human-readable message indicating details about why the build has this status.
    pub message: Option<String>,

    /// output describes the container image the build has produced.
    pub output: Option<crate::api::build::v1::BuildStatusOutput>,

    /// outputDockerImageReference contains a reference to the container image that will be built by this build. Its value is computed from Build.Spec.Output.To, and should include the registry address, so that it can be used to push and pull the image.
    pub output_docker_image_reference: Option<String>,

    /// phase is the point in the build lifecycle. Possible values are "New", "Pending", "Running", "Complete", "Failed", "Error", and "Cancelled".
    pub phase: String,

    /// reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
    pub reason: Option<String>,

    /// stages contains details about each stage that occurs during the build including start time, duration (in milliseconds), and the steps that occured within each stage.
    pub stages: Option<Vec<crate::api::build::v1::StageInfo>>,

    /// startTimestamp is a timestamp representing the server time when this Build started running in a Pod. It is represented in RFC3339 form and is in UTC.
    pub start_timestamp: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> serde::Deserialize<'de> for BuildStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cancelled,
            Key_completion_timestamp,
            Key_config,
            Key_duration,
            Key_log_snippet,
            Key_message,
            Key_output,
            Key_output_docker_image_reference,
            Key_phase,
            Key_reason,
            Key_stages,
            Key_start_timestamp,
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
                            "cancelled" => Field::Key_cancelled,
                            "completionTimestamp" => Field::Key_completion_timestamp,
                            "config" => Field::Key_config,
                            "duration" => Field::Key_duration,
                            "logSnippet" => Field::Key_log_snippet,
                            "message" => Field::Key_message,
                            "output" => Field::Key_output,
                            "outputDockerImageReference" => Field::Key_output_docker_image_reference,
                            "phase" => Field::Key_phase,
                            "reason" => Field::Key_reason,
                            "stages" => Field::Key_stages,
                            "startTimestamp" => Field::Key_start_timestamp,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_cancelled: Option<bool> = None;
                let mut value_completion_timestamp: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_config: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_duration: Option<i64> = None;
                let mut value_log_snippet: Option<String> = None;
                let mut value_message: Option<String> = None;
                let mut value_output: Option<crate::api::build::v1::BuildStatusOutput> = None;
                let mut value_output_docker_image_reference: Option<String> = None;
                let mut value_phase: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_stages: Option<Vec<crate::api::build::v1::StageInfo>> = None;
                let mut value_start_timestamp: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cancelled => value_cancelled = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completion_timestamp => value_completion_timestamp = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config => value_config = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_duration => value_duration = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_log_snippet => value_log_snippet = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_output => value_output = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_output_docker_image_reference => value_output_docker_image_reference = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stages => value_stages = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_timestamp => value_start_timestamp = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildStatus {
                    cancelled: value_cancelled,
                    completion_timestamp: value_completion_timestamp,
                    config: value_config,
                    duration: value_duration,
                    log_snippet: value_log_snippet,
                    message: value_message,
                    output: value_output,
                    output_docker_image_reference: value_output_docker_image_reference,
                    phase: value_phase.ok_or_else(|| serde::de::Error::missing_field("phase"))?,
                    reason: value_reason,
                    stages: value_stages,
                    start_timestamp: value_start_timestamp,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildStatus",
            &[
                "cancelled",
                "completionTimestamp",
                "config",
                "duration",
                "logSnippet",
                "message",
                "output",
                "outputDockerImageReference",
                "phase",
                "reason",
                "stages",
                "startTimestamp",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildStatus",
            1 +
            self.cancelled.as_ref().map_or(0, |_| 1) +
            self.completion_timestamp.as_ref().map_or(0, |_| 1) +
            self.config.as_ref().map_or(0, |_| 1) +
            self.duration.as_ref().map_or(0, |_| 1) +
            self.log_snippet.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.output.as_ref().map_or(0, |_| 1) +
            self.output_docker_image_reference.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.stages.as_ref().map_or(0, |_| 1) +
            self.start_timestamp.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cancelled {
            serde::ser::SerializeStruct::serialize_field(&mut state, "cancelled", value)?;
        }
        if let Some(value) = &self.completion_timestamp {
            serde::ser::SerializeStruct::serialize_field(&mut state, "completionTimestamp", value)?;
        }
        if let Some(value) = &self.config {
            serde::ser::SerializeStruct::serialize_field(&mut state, "config", value)?;
        }
        if let Some(value) = &self.duration {
            serde::ser::SerializeStruct::serialize_field(&mut state, "duration", value)?;
        }
        if let Some(value) = &self.log_snippet {
            serde::ser::SerializeStruct::serialize_field(&mut state, "logSnippet", value)?;
        }
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.output {
            serde::ser::SerializeStruct::serialize_field(&mut state, "output", value)?;
        }
        if let Some(value) = &self.output_docker_image_reference {
            serde::ser::SerializeStruct::serialize_field(&mut state, "outputDockerImageReference", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "phase", &self.phase)?;
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.stages {
            serde::ser::SerializeStruct::serialize_field(&mut state, "stages", value)?;
        }
        if let Some(value) = &self.start_timestamp {
            serde::ser::SerializeStruct::serialize_field(&mut state, "startTimestamp", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
