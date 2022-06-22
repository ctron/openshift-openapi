// Generated from definition com.github.openshift.api.apps.v1.RecreateDeploymentStrategyParams

/// RecreateDeploymentStrategyParams are the input to the Recreate deployment strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecreateDeploymentStrategyParams {
    /// Mid is a lifecycle hook which is executed while the deployment is scaled down to zero before the first new pod is created. All LifecycleHookFailurePolicy values are supported.
    pub mid: Option<crate::api::apps::v1::LifecycleHook>,

    /// Post is a lifecycle hook which is executed after the strategy has finished all deployment logic. All LifecycleHookFailurePolicy values are supported.
    pub post: Option<crate::api::apps::v1::LifecycleHook>,

    /// Pre is a lifecycle hook which is executed before the strategy manipulates the deployment. All LifecycleHookFailurePolicy values are supported.
    pub pre: Option<crate::api::apps::v1::LifecycleHook>,

    /// TimeoutSeconds is the time to wait for updates before giving up. If the value is nil, a default will be used.
    pub timeout_seconds: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for RecreateDeploymentStrategyParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_mid,
            Key_post,
            Key_pre,
            Key_timeout_seconds,
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
                            "mid" => Field::Key_mid,
                            "post" => Field::Key_post,
                            "pre" => Field::Key_pre,
                            "timeoutSeconds" => Field::Key_timeout_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RecreateDeploymentStrategyParams;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RecreateDeploymentStrategyParams")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_mid: Option<crate::api::apps::v1::LifecycleHook> = None;
                let mut value_post: Option<crate::api::apps::v1::LifecycleHook> = None;
                let mut value_pre: Option<crate::api::apps::v1::LifecycleHook> = None;
                let mut value_timeout_seconds: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_mid => value_mid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_post => value_post = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pre => value_pre = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout_seconds => value_timeout_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RecreateDeploymentStrategyParams {
                    mid: value_mid,
                    post: value_post,
                    pre: value_pre,
                    timeout_seconds: value_timeout_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "RecreateDeploymentStrategyParams",
            &[
                "mid",
                "post",
                "pre",
                "timeoutSeconds",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RecreateDeploymentStrategyParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RecreateDeploymentStrategyParams",
            self.mid.as_ref().map_or(0, |_| 1) +
            self.post.as_ref().map_or(0, |_| 1) +
            self.pre.as_ref().map_or(0, |_| 1) +
            self.timeout_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.mid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "mid", value)?;
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
        serde::ser::SerializeStruct::end(state)
    }
}
