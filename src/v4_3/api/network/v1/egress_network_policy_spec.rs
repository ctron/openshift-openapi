// Generated from definition com.github.openshift.api.network.v1.EgressNetworkPolicySpec

/// EgressNetworkPolicySpec provides a list of policies on outgoing network traffic
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EgressNetworkPolicySpec {
    /// egress contains the list of egress policy rules
    pub egress: Vec<crate::api::network::v1::EgressNetworkPolicyRule>,
}

impl<'de> serde::Deserialize<'de> for EgressNetworkPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_egress,
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
                            "egress" => Field::Key_egress,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EgressNetworkPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EgressNetworkPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_egress: Option<Vec<crate::api::network::v1::EgressNetworkPolicyRule>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_egress => value_egress = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EgressNetworkPolicySpec {
                    egress: value_egress.ok_or_else(|| serde::de::Error::missing_field("egress"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "EgressNetworkPolicySpec",
            &[
                "egress",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EgressNetworkPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EgressNetworkPolicySpec",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "egress", &self.egress)?;
        serde::ser::SerializeStruct::end(state)
    }
}
