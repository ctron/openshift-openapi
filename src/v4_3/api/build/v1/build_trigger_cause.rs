// Generated from definition com.github.openshift.api.build.v1.BuildTriggerCause

/// BuildTriggerCause holds information about a triggered build. It is used for displaying build trigger data for each build and build configuration in oc describe. It is also used to describe which triggers led to the most recent update in the build configuration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildTriggerCause {
    /// BitbucketWebHook represents data for a Bitbucket webhook that fired a specific build.
    pub bitbucket_web_hook: Option<crate::api::build::v1::BitbucketWebHookCause>,

    /// genericWebHook holds data about a builds generic webhook trigger.
    pub generic_web_hook: Option<crate::api::build::v1::GenericWebHookCause>,

    /// gitHubWebHook represents data for a GitHub webhook that fired a specific build.
    pub github_web_hook: Option<crate::api::build::v1::GitHubWebHookCause>,

    /// GitLabWebHook represents data for a GitLab webhook that fired a specific build.
    pub gitlab_web_hook: Option<crate::api::build::v1::GitLabWebHookCause>,

    /// imageChangeBuild stores information about an imagechange event that triggered a new build.
    pub image_change_build: Option<crate::api::build::v1::ImageChangeCause>,

    /// message is used to store a human readable message for why the build was triggered. E.g.: "Manually triggered by user", "Configuration change",etc.
    pub message: Option<String>,
}

impl<'de> serde::Deserialize<'de> for BuildTriggerCause {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bitbucket_web_hook,
            Key_generic_web_hook,
            Key_github_web_hook,
            Key_gitlab_web_hook,
            Key_image_change_build,
            Key_message,
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
                            "bitbucketWebHook" => Field::Key_bitbucket_web_hook,
                            "genericWebHook" => Field::Key_generic_web_hook,
                            "githubWebHook" => Field::Key_github_web_hook,
                            "gitlabWebHook" => Field::Key_gitlab_web_hook,
                            "imageChangeBuild" => Field::Key_image_change_build,
                            "message" => Field::Key_message,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildTriggerCause;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildTriggerCause")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_bitbucket_web_hook: Option<crate::api::build::v1::BitbucketWebHookCause> = None;
                let mut value_generic_web_hook: Option<crate::api::build::v1::GenericWebHookCause> = None;
                let mut value_github_web_hook: Option<crate::api::build::v1::GitHubWebHookCause> = None;
                let mut value_gitlab_web_hook: Option<crate::api::build::v1::GitLabWebHookCause> = None;
                let mut value_image_change_build: Option<crate::api::build::v1::ImageChangeCause> = None;
                let mut value_message: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bitbucket_web_hook => value_bitbucket_web_hook = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generic_web_hook => value_generic_web_hook = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_github_web_hook => value_github_web_hook = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gitlab_web_hook => value_gitlab_web_hook = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_change_build => value_image_change_build = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildTriggerCause {
                    bitbucket_web_hook: value_bitbucket_web_hook,
                    generic_web_hook: value_generic_web_hook,
                    github_web_hook: value_github_web_hook,
                    gitlab_web_hook: value_gitlab_web_hook,
                    image_change_build: value_image_change_build,
                    message: value_message,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildTriggerCause",
            &[
                "bitbucketWebHook",
                "genericWebHook",
                "githubWebHook",
                "gitlabWebHook",
                "imageChangeBuild",
                "message",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildTriggerCause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildTriggerCause",
            self.bitbucket_web_hook.as_ref().map_or(0, |_| 1) +
            self.generic_web_hook.as_ref().map_or(0, |_| 1) +
            self.github_web_hook.as_ref().map_or(0, |_| 1) +
            self.gitlab_web_hook.as_ref().map_or(0, |_| 1) +
            self.image_change_build.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bitbucket_web_hook {
            serde::ser::SerializeStruct::serialize_field(&mut state, "bitbucketWebHook", value)?;
        }
        if let Some(value) = &self.generic_web_hook {
            serde::ser::SerializeStruct::serialize_field(&mut state, "genericWebHook", value)?;
        }
        if let Some(value) = &self.github_web_hook {
            serde::ser::SerializeStruct::serialize_field(&mut state, "githubWebHook", value)?;
        }
        if let Some(value) = &self.gitlab_web_hook {
            serde::ser::SerializeStruct::serialize_field(&mut state, "gitlabWebHook", value)?;
        }
        if let Some(value) = &self.image_change_build {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageChangeBuild", value)?;
        }
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
