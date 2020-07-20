// Generated from definition com.github.openshift.api.authorization.v1.ServiceAccountRestriction

/// ServiceAccountRestriction matches a service account by a string match on either the service-account name or the name of the service account's namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountRestriction {
    /// Namespaces specifies a list of literal namespace names.
    pub namespaces: Vec<String>,

    /// ServiceAccounts specifies a list of literal service-account names.
    pub serviceaccounts: Vec<crate::api::authorization::v1::ServiceAccountReference>,
}

impl<'de> serde::Deserialize<'de> for ServiceAccountRestriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_namespaces,
            Key_serviceaccounts,
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
                            "namespaces" => Field::Key_namespaces,
                            "serviceaccounts" => Field::Key_serviceaccounts,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceAccountRestriction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceAccountRestriction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_namespaces: Option<Vec<String>> = None;
                let mut value_serviceaccounts: Option<Vec<crate::api::authorization::v1::ServiceAccountReference>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_namespaces => value_namespaces = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_serviceaccounts => value_serviceaccounts = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceAccountRestriction {
                    namespaces: value_namespaces.ok_or_else(|| serde::de::Error::missing_field("namespaces"))?,
                    serviceaccounts: value_serviceaccounts.ok_or_else(|| serde::de::Error::missing_field("serviceaccounts"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceAccountRestriction",
            &[
                "namespaces",
                "serviceaccounts",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ServiceAccountRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceAccountRestriction",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "namespaces", &self.namespaces)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "serviceaccounts", &self.serviceaccounts)?;
        serde::ser::SerializeStruct::end(state)
    }
}
