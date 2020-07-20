// Generated from definition com.github.openshift.api.network.v1.EgressNetworkPolicyRule

/// EgressNetworkPolicyRule contains a single egress network policy rule
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EgressNetworkPolicyRule {
    /// to is the target that traffic is allowed/denied to
    pub to: crate::api::network::v1::EgressNetworkPolicyPeer,

    /// type marks this as an "Allow" or "Deny" rule
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for EgressNetworkPolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_to,
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
                            "to" => Field::Key_to,
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
            type Value = EgressNetworkPolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EgressNetworkPolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_to: Option<crate::api::network::v1::EgressNetworkPolicyPeer> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_to => value_to = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EgressNetworkPolicyRule {
                    to: value_to.ok_or_else(|| serde::de::Error::missing_field("to"))?,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "EgressNetworkPolicyRule",
            &[
                "to",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EgressNetworkPolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EgressNetworkPolicyRule",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "to", &self.to)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
