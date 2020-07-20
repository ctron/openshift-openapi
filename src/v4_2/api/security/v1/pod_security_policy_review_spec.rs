// Generated from definition com.github.openshift.api.security.v1.PodSecurityPolicyReviewSpec

/// PodSecurityPolicyReviewSpec defines specification for PodSecurityPolicyReview
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicyReviewSpec {
    /// serviceAccountNames is an optional set of ServiceAccounts to run the check with. If serviceAccountNames is empty, the template.spec.serviceAccountName is used, unless it's empty, in which case "default" is used instead. If serviceAccountNames is specified, template.spec.serviceAccountName is ignored.
    pub service_account_names: Option<Vec<String>>,

    /// template is the PodTemplateSpec to check. The template.spec.serviceAccountName field is used if serviceAccountNames is empty, unless the template.spec.serviceAccountName is empty, in which case "default" is used. If serviceAccountNames is specified, template.spec.serviceAccountName is ignored.
    pub template: k8s_openapi::api::core::v1::PodTemplateSpec,
}

impl<'de> serde::Deserialize<'de> for PodSecurityPolicyReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_service_account_names,
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
                            "serviceAccountNames" => Field::Key_service_account_names,
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
            type Value = PodSecurityPolicyReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSecurityPolicyReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_service_account_names: Option<Vec<String>> = None;
                let mut value_template: Option<k8s_openapi::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_service_account_names => value_service_account_names = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityPolicyReviewSpec {
                    service_account_names: value_service_account_names,
                    template: value_template.ok_or_else(|| serde::de::Error::missing_field("template"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityPolicyReviewSpec",
            &[
                "serviceAccountNames",
                "template",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodSecurityPolicyReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityPolicyReviewSpec",
            1 +
            self.service_account_names.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.service_account_names {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccountNames", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        serde::ser::SerializeStruct::end(state)
    }
}
