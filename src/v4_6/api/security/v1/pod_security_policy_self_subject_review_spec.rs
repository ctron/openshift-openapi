// Generated from definition com.github.openshift.api.security.v1.PodSecurityPolicySelfSubjectReviewSpec

/// PodSecurityPolicySelfSubjectReviewSpec contains specification for PodSecurityPolicySelfSubjectReview.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicySelfSubjectReviewSpec {
    /// template is the PodTemplateSpec to check.
    pub template: k8s_openapi::api::core::v1::PodTemplateSpec,
}

impl<'de> serde::Deserialize<'de> for PodSecurityPolicySelfSubjectReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_template,
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
                            "template" => Field::Key_template,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodSecurityPolicySelfSubjectReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSecurityPolicySelfSubjectReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_template: Option<k8s_openapi::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_template => value_template = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityPolicySelfSubjectReviewSpec {
                    template: value_template.ok_or_else(|| serde::de::Error::missing_field("template"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityPolicySelfSubjectReviewSpec",
            &[
                "template",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodSecurityPolicySelfSubjectReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityPolicySelfSubjectReviewSpec",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        serde::ser::SerializeStruct::end(state)
    }
}
