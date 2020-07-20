// Generated from definition com.github.openshift.api.build.v1.BuildTriggerPolicy

/// BuildTriggerPolicy describes a policy for a single trigger that results in a new Build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildTriggerPolicy {
    /// BitbucketWebHook contains the parameters for a Bitbucket webhook type of trigger
    pub bitbucket: Option<crate::api::build::v1::WebHookTrigger>,

    /// generic contains the parameters for a Generic webhook type of trigger
    pub generic: Option<crate::api::build::v1::WebHookTrigger>,

    /// github contains the parameters for a GitHub webhook type of trigger
    pub github: Option<crate::api::build::v1::WebHookTrigger>,

    /// GitLabWebHook contains the parameters for a GitLab webhook type of trigger
    pub gitlab: Option<crate::api::build::v1::WebHookTrigger>,

    /// imageChange contains parameters for an ImageChange type of trigger
    pub image_change: Option<crate::api::build::v1::ImageChangeTrigger>,

    /// type is the type of build trigger
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for BuildTriggerPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bitbucket,
            Key_generic,
            Key_github,
            Key_gitlab,
            Key_image_change,
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
                            "bitbucket" => Field::Key_bitbucket,
                            "generic" => Field::Key_generic,
                            "github" => Field::Key_github,
                            "gitlab" => Field::Key_gitlab,
                            "imageChange" => Field::Key_image_change,
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
            type Value = BuildTriggerPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildTriggerPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_bitbucket: Option<crate::api::build::v1::WebHookTrigger> = None;
                let mut value_generic: Option<crate::api::build::v1::WebHookTrigger> = None;
                let mut value_github: Option<crate::api::build::v1::WebHookTrigger> = None;
                let mut value_gitlab: Option<crate::api::build::v1::WebHookTrigger> = None;
                let mut value_image_change: Option<crate::api::build::v1::ImageChangeTrigger> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bitbucket => value_bitbucket = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generic => value_generic = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_github => value_github = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gitlab => value_gitlab = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_change => value_image_change = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildTriggerPolicy {
                    bitbucket: value_bitbucket,
                    generic: value_generic,
                    github: value_github,
                    gitlab: value_gitlab,
                    image_change: value_image_change,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildTriggerPolicy",
            &[
                "bitbucket",
                "generic",
                "github",
                "gitlab",
                "imageChange",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildTriggerPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildTriggerPolicy",
            1 +
            self.bitbucket.as_ref().map_or(0, |_| 1) +
            self.generic.as_ref().map_or(0, |_| 1) +
            self.github.as_ref().map_or(0, |_| 1) +
            self.gitlab.as_ref().map_or(0, |_| 1) +
            self.image_change.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bitbucket {
            serde::ser::SerializeStruct::serialize_field(&mut state, "bitbucket", value)?;
        }
        if let Some(value) = &self.generic {
            serde::ser::SerializeStruct::serialize_field(&mut state, "generic", value)?;
        }
        if let Some(value) = &self.github {
            serde::ser::SerializeStruct::serialize_field(&mut state, "github", value)?;
        }
        if let Some(value) = &self.gitlab {
            serde::ser::SerializeStruct::serialize_field(&mut state, "gitlab", value)?;
        }
        if let Some(value) = &self.image_change {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageChange", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
