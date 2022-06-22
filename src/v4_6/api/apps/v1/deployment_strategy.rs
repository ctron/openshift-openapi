// Generated from definition com.github.openshift.api.apps.v1.DeploymentStrategy

/// DeploymentStrategy describes how to perform a deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentStrategy {
    /// ActiveDeadlineSeconds is the duration in seconds that the deployer pods for this deployment config may be active on a node before the system actively tries to terminate them.
    pub active_deadline_seconds: Option<i64>,

    /// Annotations is a set of key, value pairs added to custom deployer and lifecycle pre/post hook pods.
    pub annotations: Option<std::collections::BTreeMap<String, String>>,

    /// CustomParams are the input to the Custom deployment strategy, and may also be specified for the Recreate and Rolling strategies to customize the execution process that runs the deployment.
    pub custom_params: Option<crate::api::apps::v1::CustomDeploymentStrategyParams>,

    /// Labels is a set of key, value pairs added to custom deployer and lifecycle pre/post hook pods.
    pub labels: Option<std::collections::BTreeMap<String, String>>,

    /// RecreateParams are the input to the Recreate deployment strategy.
    pub recreate_params: Option<crate::api::apps::v1::RecreateDeploymentStrategyParams>,

    /// Resources contains resource requirements to execute the deployment and any hooks.
    pub resources: Option<k8s_openapi::api::core::v1::ResourceRequirements>,

    /// RollingParams are the input to the Rolling deployment strategy.
    pub rolling_params: Option<crate::api::apps::v1::RollingDeploymentStrategyParams>,

    /// Type is the name of a deployment strategy.
    pub type_: Option<String>,
}

impl<'de> serde::Deserialize<'de> for DeploymentStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active_deadline_seconds,
            Key_annotations,
            Key_custom_params,
            Key_labels,
            Key_recreate_params,
            Key_resources,
            Key_rolling_params,
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
                            "activeDeadlineSeconds" => Field::Key_active_deadline_seconds,
                            "annotations" => Field::Key_annotations,
                            "customParams" => Field::Key_custom_params,
                            "labels" => Field::Key_labels,
                            "recreateParams" => Field::Key_recreate_params,
                            "resources" => Field::Key_resources,
                            "rollingParams" => Field::Key_rolling_params,
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
            type Value = DeploymentStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_active_deadline_seconds: Option<i64> = None;
                let mut value_annotations: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_custom_params: Option<crate::api::apps::v1::CustomDeploymentStrategyParams> = None;
                let mut value_labels: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_recreate_params: Option<crate::api::apps::v1::RecreateDeploymentStrategyParams> = None;
                let mut value_resources: Option<k8s_openapi::api::core::v1::ResourceRequirements> = None;
                let mut value_rolling_params: Option<crate::api::apps::v1::RollingDeploymentStrategyParams> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active_deadline_seconds => value_active_deadline_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_annotations => value_annotations = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_custom_params => value_custom_params = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_labels => value_labels = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_recreate_params => value_recreate_params = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rolling_params => value_rolling_params = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentStrategy {
                    active_deadline_seconds: value_active_deadline_seconds,
                    annotations: value_annotations,
                    custom_params: value_custom_params,
                    labels: value_labels,
                    recreate_params: value_recreate_params,
                    resources: value_resources,
                    rolling_params: value_rolling_params,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentStrategy",
            &[
                "activeDeadlineSeconds",
                "annotations",
                "customParams",
                "labels",
                "recreateParams",
                "resources",
                "rollingParams",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentStrategy",
            self.active_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.annotations.as_ref().map_or(0, |_| 1) +
            self.custom_params.as_ref().map_or(0, |_| 1) +
            self.labels.as_ref().map_or(0, |_| 1) +
            self.recreate_params.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.rolling_params.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active_deadline_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "activeDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.annotations {
            serde::ser::SerializeStruct::serialize_field(&mut state, "annotations", value)?;
        }
        if let Some(value) = &self.custom_params {
            serde::ser::SerializeStruct::serialize_field(&mut state, "customParams", value)?;
        }
        if let Some(value) = &self.labels {
            serde::ser::SerializeStruct::serialize_field(&mut state, "labels", value)?;
        }
        if let Some(value) = &self.recreate_params {
            serde::ser::SerializeStruct::serialize_field(&mut state, "recreateParams", value)?;
        }
        if let Some(value) = &self.resources {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.rolling_params {
            serde::ser::SerializeStruct::serialize_field(&mut state, "rollingParams", value)?;
        }
        if let Some(value) = &self.type_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
