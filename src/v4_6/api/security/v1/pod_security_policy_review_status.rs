// Generated from definition com.github.openshift.api.security.v1.PodSecurityPolicyReviewStatus

/// PodSecurityPolicyReviewStatus represents the status of PodSecurityPolicyReview.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicyReviewStatus {
    /// allowedServiceAccounts returns the list of service accounts in *this* namespace that have the power to create the PodTemplateSpec.
    pub allowed_service_accounts: Vec<crate::api::security::v1::ServiceAccountPodSecurityPolicyReviewStatus>,
}

impl<'de> serde::Deserialize<'de> for PodSecurityPolicyReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allowed_service_accounts,
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
                            "allowedServiceAccounts" => Field::Key_allowed_service_accounts,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodSecurityPolicyReviewStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSecurityPolicyReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allowed_service_accounts: Option<Vec<crate::api::security::v1::ServiceAccountPodSecurityPolicyReviewStatus>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allowed_service_accounts => value_allowed_service_accounts = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityPolicyReviewStatus {
                    allowed_service_accounts: value_allowed_service_accounts.ok_or_else(|| serde::de::Error::missing_field("allowedServiceAccounts"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityPolicyReviewStatus",
            &[
                "allowedServiceAccounts",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodSecurityPolicyReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityPolicyReviewStatus",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowedServiceAccounts", &self.allowed_service_accounts)?;
        serde::ser::SerializeStruct::end(state)
    }
}
