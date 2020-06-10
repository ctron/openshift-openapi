// Generated from definition com.github.openshift.api.security.v1.ServiceAccountPodSecurityPolicyReviewStatus

/// ServiceAccountPodSecurityPolicyReviewStatus represents ServiceAccount name and related review status
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountPodSecurityPolicyReviewStatus {
    /// allowedBy is a reference to the rule that allows the PodTemplateSpec. A rule can be a SecurityContextConstraint or a PodSecurityPolicy A `nil`, indicates that it was denied.
    pub allowed_by: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// name contains the allowed and the denied ServiceAccount name
    pub name: String,

    /// A machine-readable description of why this operation is in the "Failure" status. If this value is empty there is no information available.
    pub reason: Option<String>,

    /// template is the PodTemplateSpec after the defaulting is applied.
    pub template: Option<k8s_openapi::api::core::v1::PodTemplateSpec>,
}

impl<'de> serde::Deserialize<'de> for ServiceAccountPodSecurityPolicyReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allowed_by,
            Key_name,
            Key_reason,
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
                            "allowedBy" => Field::Key_allowed_by,
                            "name" => Field::Key_name,
                            "reason" => Field::Key_reason,
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
            type Value = ServiceAccountPodSecurityPolicyReviewStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceAccountPodSecurityPolicyReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allowed_by: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_name: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_template: Option<k8s_openapi::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allowed_by => value_allowed_by = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceAccountPodSecurityPolicyReviewStatus {
                    allowed_by: value_allowed_by,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    reason: value_reason,
                    template: value_template,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceAccountPodSecurityPolicyReviewStatus",
            &[
                "allowedBy",
                "name",
                "reason",
                "template",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ServiceAccountPodSecurityPolicyReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceAccountPodSecurityPolicyReviewStatus",
            1 +
            self.allowed_by.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.template.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allowed_by {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowedBy", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.template {
            serde::ser::SerializeStruct::serialize_field(&mut state, "template", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
