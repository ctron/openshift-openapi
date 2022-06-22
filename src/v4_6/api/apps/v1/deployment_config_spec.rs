// Generated from definition com.github.openshift.api.apps.v1.DeploymentConfigSpec

/// DeploymentConfigSpec represents the desired state of the deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentConfigSpec {
    /// MinReadySeconds is the minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    pub min_ready_seconds: Option<i32>,

    /// Paused indicates that the deployment config is paused resulting in no new deployments on template changes or changes in the template caused by other triggers.
    pub paused: Option<bool>,

    /// Replicas is the number of desired replicas.
    pub replicas: Option<i32>,

    /// RevisionHistoryLimit is the number of old ReplicationControllers to retain to allow for rollbacks. This field is a pointer to allow for differentiation between an explicit zero and not specified. Defaults to 10. (This only applies to DeploymentConfigs created via the new group API resource, not the legacy resource.)
    pub revision_history_limit: Option<i32>,

    /// Selector is a label query over pods that should match the Replicas count.
    pub selector: Option<std::collections::BTreeMap<String, String>>,

    /// Strategy describes how a deployment is executed.
    pub strategy: Option<crate::api::apps::v1::DeploymentStrategy>,

    /// Template is the object that describes the pod that will be created if insufficient replicas are detected.
    pub template: Option<k8s_openapi::api::core::v1::PodTemplateSpec>,

    /// Test ensures that this deployment config will have zero replicas except while a deployment is running. This allows the deployment config to be used as a continuous deployment test - triggering on images, running the deployment, and then succeeding or failing. Post strategy hooks and After actions can be used to integrate successful deployment with an action.
    pub test: Option<bool>,

    /// Triggers determine how updates to a DeploymentConfig result in new deployments. If no triggers are defined, a new deployment can only occur as a result of an explicit client update to the DeploymentConfig with a new LatestVersion. If null, defaults to having a config change trigger.
    pub triggers: Option<Vec<crate::api::apps::v1::DeploymentTriggerPolicy>>,
}

impl<'de> serde::Deserialize<'de> for DeploymentConfigSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_paused,
            Key_replicas,
            Key_revision_history_limit,
            Key_selector,
            Key_strategy,
            Key_template,
            Key_test,
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
                            "minReadySeconds" => Field::Key_min_ready_seconds,
                            "paused" => Field::Key_paused,
                            "replicas" => Field::Key_replicas,
                            "revisionHistoryLimit" => Field::Key_revision_history_limit,
                            "selector" => Field::Key_selector,
                            "strategy" => Field::Key_strategy,
                            "template" => Field::Key_template,
                            "test" => Field::Key_test,
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
            type Value = DeploymentConfigSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentConfigSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_paused: Option<bool> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_revision_history_limit: Option<i32> = None;
                let mut value_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_strategy: Option<crate::api::apps::v1::DeploymentStrategy> = None;
                let mut value_template: Option<k8s_openapi::api::core::v1::PodTemplateSpec> = None;
                let mut value_test: Option<bool> = None;
                let mut value_triggers: Option<Vec<crate::api::apps::v1::DeploymentTriggerPolicy>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_paused => value_paused = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision_history_limit => value_revision_history_limit = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_test => value_test = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_triggers => value_triggers = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentConfigSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    paused: value_paused,
                    replicas: value_replicas,
                    revision_history_limit: value_revision_history_limit,
                    selector: value_selector,
                    strategy: value_strategy,
                    template: value_template,
                    test: value_test,
                    triggers: value_triggers,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentConfigSpec",
            &[
                "minReadySeconds",
                "paused",
                "replicas",
                "revisionHistoryLimit",
                "selector",
                "strategy",
                "template",
                "test",
                "triggers",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentConfigSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentConfigSpec",
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.paused.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.revision_history_limit.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.strategy.as_ref().map_or(0, |_| 1) +
            self.template.as_ref().map_or(0, |_| 1) +
            self.test.as_ref().map_or(0, |_| 1) +
            self.triggers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.paused {
            serde::ser::SerializeStruct::serialize_field(&mut state, "paused", value)?;
        }
        if let Some(value) = &self.replicas {
            serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.revision_history_limit {
            serde::ser::SerializeStruct::serialize_field(&mut state, "revisionHistoryLimit", value)?;
        }
        if let Some(value) = &self.selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.strategy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", value)?;
        }
        if let Some(value) = &self.template {
            serde::ser::SerializeStruct::serialize_field(&mut state, "template", value)?;
        }
        if let Some(value) = &self.test {
            serde::ser::SerializeStruct::serialize_field(&mut state, "test", value)?;
        }
        if let Some(value) = &self.triggers {
            serde::ser::SerializeStruct::serialize_field(&mut state, "triggers", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
