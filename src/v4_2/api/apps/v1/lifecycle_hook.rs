// Generated from definition com.github.openshift.api.apps.v1.LifecycleHook

/// LifecycleHook defines a specific deployment lifecycle action. Only one type of action may be specified at any time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LifecycleHook {
    /// ExecNewPod specifies the options for a lifecycle hook backed by a pod.
    pub exec_new_pod: Option<crate::api::apps::v1::ExecNewPodHook>,

    /// FailurePolicy specifies what action to take if the hook fails.
    pub failure_policy: String,

    /// TagImages instructs the deployer to tag the current image referenced under a container onto an image stream tag.
    pub tag_images: Option<Vec<crate::api::apps::v1::TagImageHook>>,
}

impl<'de> serde::Deserialize<'de> for LifecycleHook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_exec_new_pod,
            Key_failure_policy,
            Key_tag_images,
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
                            "execNewPod" => Field::Key_exec_new_pod,
                            "failurePolicy" => Field::Key_failure_policy,
                            "tagImages" => Field::Key_tag_images,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LifecycleHook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LifecycleHook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_exec_new_pod: Option<crate::api::apps::v1::ExecNewPodHook> = None;
                let mut value_failure_policy: Option<String> = None;
                let mut value_tag_images: Option<Vec<crate::api::apps::v1::TagImageHook>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_exec_new_pod => value_exec_new_pod = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failure_policy => value_failure_policy = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_tag_images => value_tag_images = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LifecycleHook {
                    exec_new_pod: value_exec_new_pod,
                    failure_policy: value_failure_policy.ok_or_else(|| serde::de::Error::missing_field("failurePolicy"))?,
                    tag_images: value_tag_images,
                })
            }
        }

        deserializer.deserialize_struct(
            "LifecycleHook",
            &[
                "execNewPod",
                "failurePolicy",
                "tagImages",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for LifecycleHook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LifecycleHook",
            1 +
            self.exec_new_pod.as_ref().map_or(0, |_| 1) +
            self.tag_images.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.exec_new_pod {
            serde::ser::SerializeStruct::serialize_field(&mut state, "execNewPod", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", &self.failure_policy)?;
        if let Some(value) = &self.tag_images {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tagImages", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
