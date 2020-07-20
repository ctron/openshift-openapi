// Generated from definition com.github.openshift.api.network.v1.EgressNetworkPolicyPeer

/// EgressNetworkPolicyPeer specifies a target to apply egress network policy to
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EgressNetworkPolicyPeer {
    /// cidrSelector is the CIDR range to allow/deny traffic to. If this is set, dnsName must be unset
    pub cidr_selector: Option<String>,

    /// dnsName is the domain name to allow/deny traffic to. If this is set, cidrSelector must be unset
    pub dns_name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for EgressNetworkPolicyPeer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cidr_selector,
            Key_dns_name,
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
                            "cidrSelector" => Field::Key_cidr_selector,
                            "dnsName" => Field::Key_dns_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EgressNetworkPolicyPeer;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EgressNetworkPolicyPeer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_cidr_selector: Option<String> = None;
                let mut value_dns_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cidr_selector => value_cidr_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dns_name => value_dns_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EgressNetworkPolicyPeer {
                    cidr_selector: value_cidr_selector,
                    dns_name: value_dns_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "EgressNetworkPolicyPeer",
            &[
                "cidrSelector",
                "dnsName",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EgressNetworkPolicyPeer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EgressNetworkPolicyPeer",
            self.cidr_selector.as_ref().map_or(0, |_| 1) +
            self.dns_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cidr_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "cidrSelector", value)?;
        }
        if let Some(value) = &self.dns_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dnsName", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
