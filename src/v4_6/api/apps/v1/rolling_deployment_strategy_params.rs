// Generated from definition com.github.openshift.api.apps.v1.RollingDeploymentStrategyParams

/// RollingDeploymentStrategyParams are the input to the Rolling deployment strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RollingDeploymentStrategyParams {
    /// IntervalSeconds is the time to wait between polling deployment status after update. If the value is nil, a default will be used.
    pub interval_seconds: Option<i64>,

    /// MaxSurge is the maximum number of pods that can be scheduled above the original number of pods. Value can be an absolute number (ex: 5) or a percentage of total pods at the start of the update (ex: 10%). Absolute number is calculated from percentage by rounding up.
    ///
    /// This cannot be 0 if MaxUnavailable is 0. By default, 25% is used.
    ///
    /// Example: when this is set to 30%, the new RC can be scaled up by 30% immediately when the rolling update starts. Once old pods have been killed, new RC can be scaled up further, ensuring that total number of pods running at any time during the update is atmost 130% of original pods.
    pub max_surge: Option<k8s_openapi::apimachinery::pkg::util::intstr::IntOrString>,

    /// MaxUnavailable is the maximum number of pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of total pods at the start of update (ex: 10%). Absolute number is calculated from percentage by rounding down.
    ///
    /// This cannot be 0 if MaxSurge is 0. By default, 25% is used.
    ///
    /// Example: when this is set to 30%, the old RC can be scaled down by 30% immediately when the rolling update starts. Once new pods are ready, old RC can be scaled down further, followed by scaling up the new RC, ensuring that at least 70% of original number of pods are available at all times during the update.
    pub max_unavailable: Option<k8s_openapi::apimachinery::pkg::util::intstr::IntOrString>,

    /// Post is a lifecycle hook which is executed after the strategy has finished all deployment logic. All LifecycleHookFailurePolicy values are supported.
    pub post: Option<crate::api::apps::v1::LifecycleHook>,

    /// Pre is a lifecycle hook which is executed before the deployment process begins. All LifecycleHookFailurePolicy values are supported.
    pub pre: Option<crate::api::apps::v1::LifecycleHook>,

    /// TimeoutSeconds is the time to wait for updates before giving up. If the value is nil, a default will be used.
    pub timeout_seconds: Option<i64>,

    /// UpdatePeriodSeconds is the time to wait between individual pod updates. If the value is nil, a default will be used.
    pub update_period_seconds: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for RollingDeploymentStrategyParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_interval_seconds,
            Key_max_surge,
            Key_max_unavailable,
            Key_post,
            Key_pre,
            Key_timeout_seconds,
            Key_update_period_seconds,
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
                            "intervalSeconds" => Field::Key_interval_seconds,
                            "maxSurge" => Field::Key_max_surge,
                            "maxUnavailable" => Field::Key_max_unavailable,
                            "post" => Field::Key_post,
                            "pre" => Field::Key_pre,
                            "timeoutSeconds" => Field::Key_timeout_seconds,
                            "updatePeriodSeconds" => Field::Key_update_period_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RollingDeploymentStrategyParams;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RollingDeploymentStrategyParams")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_interval_seconds: Option<i64> = None;
                let mut value_max_surge: Option<k8s_openapi::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_max_unavailable: Option<k8s_openapi::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_post: Option<crate::api::apps::v1::LifecycleHook> = None;
                let mut value_pre: Option<crate::api::apps::v1::LifecycleHook> = None;
                let mut value_timeout_seconds: Option<i64> = None;
                let mut value_update_period_seconds: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_interval_seconds => value_interval_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_surge => value_max_surge = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_unavailable => value_max_unavailable = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_post => value_post = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pre => value_pre = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout_seconds => value_timeout_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_update_period_seconds => value_update_period_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RollingDeploymentStrategyParams {
                    interval_seconds: value_interval_seconds,
                    max_surge: value_max_surge,
                    max_unavailable: value_max_unavailable,
                    post: value_post,
                    pre: value_pre,
                    timeout_seconds: value_timeout_seconds,
                    update_period_seconds: value_update_period_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "RollingDeploymentStrategyParams",
            &[
                "intervalSeconds",
                "maxSurge",
                "maxUnavailable",
                "post",
                "pre",
                "timeoutSeconds",
                "updatePeriodSeconds",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RollingDeploymentStrategyParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RollingDeploymentStrategyParams",
            self.interval_seconds.as_ref().map_or(0, |_| 1) +
            self.max_surge.as_ref().map_or(0, |_| 1) +
            self.max_unavailable.as_ref().map_or(0, |_| 1) +
            self.post.as_ref().map_or(0, |_| 1) +
            self.pre.as_ref().map_or(0, |_| 1) +
            self.timeout_seconds.as_ref().map_or(0, |_| 1) +
            self.update_period_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.interval_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "intervalSeconds", value)?;
        }
        if let Some(value) = &self.max_surge {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxSurge", value)?;
        }
        if let Some(value) = &self.max_unavailable {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxUnavailable", value)?;
        }
        if let Some(value) = &self.post {
            serde::ser::SerializeStruct::serialize_field(&mut state, "post", value)?;
        }
        if let Some(value) = &self.pre {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pre", value)?;
        }
        if let Some(value) = &self.timeout_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutSeconds", value)?;
        }
        if let Some(value) = &self.update_period_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "updatePeriodSeconds", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
