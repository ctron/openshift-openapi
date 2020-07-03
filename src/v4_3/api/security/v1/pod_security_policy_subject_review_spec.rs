// Generated from definition com.github.openshift.api.security.v1.PodSecurityPolicySubjectReviewSpec

/// PodSecurityPolicySubjectReviewSpec defines specification for PodSecurityPolicySubjectReview
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicySubjectReviewSpec {
    /// groups is the groups you're testing for.
    pub groups: Option<Vec<String>>,

    /// template is the PodTemplateSpec to check. If template.spec.serviceAccountName is empty it will not be defaulted. If its non-empty, it will be checked.
    pub template: k8s_openapi::api::core::v1::PodTemplateSpec,

    /// user is the user you're testing for. If you specify "user" but not "group", then is it interpreted as "What if user were not a member of any groups. If user and groups are empty, then the check is performed using *only* the serviceAccountName in the template.
    pub user: Option<String>,
}

impl<'de> serde::Deserialize<'de> for PodSecurityPolicySubjectReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_groups,
            Key_template,
            Key_user,
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
                            "groups" => Field::Key_groups,
                            "template" => Field::Key_template,
                            "user" => Field::Key_user,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodSecurityPolicySubjectReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSecurityPolicySubjectReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_template: Option<k8s_openapi::api::core::v1::PodTemplateSpec> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_groups => value_groups = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_user => value_user = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityPolicySubjectReviewSpec {
                    groups: value_groups,
                    template: value_template.ok_or_else(|| serde::de::Error::missing_field("template"))?,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityPolicySubjectReviewSpec",
            &[
                "groups",
                "template",
                "user",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodSecurityPolicySubjectReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityPolicySubjectReviewSpec",
            1 +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.groups {
            serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        if let Some(value) = &self.user {
            serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
