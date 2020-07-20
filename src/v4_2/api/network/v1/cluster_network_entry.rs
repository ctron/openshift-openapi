// Generated from definition com.github.openshift.api.network.v1.ClusterNetworkEntry

/// ClusterNetworkEntry defines an individual cluster network. The CIDRs cannot overlap with other cluster network CIDRs, CIDRs reserved for external ips, CIDRs reserved for service networks, and CIDRs reserved for ingress ips.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterNetworkEntry {
    /// CIDR defines the total range of a cluster networks address space.
    pub cidr: String,

    /// HostSubnetLength is the number of bits of the accompanying CIDR address to allocate to each node. eg, 8 would mean that each node would have a /24 slice of the overlay network for its pods.
    pub host_subnet_length: i64,
}

impl<'de> serde::Deserialize<'de> for ClusterNetworkEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cidr,
            Key_host_subnet_length,
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
                            "CIDR" => Field::Key_cidr,
                            "hostSubnetLength" => Field::Key_host_subnet_length,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterNetworkEntry;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ClusterNetworkEntry")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_cidr: Option<String> = None;
                let mut value_host_subnet_length: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cidr => value_cidr = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_host_subnet_length => value_host_subnet_length = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterNetworkEntry {
                    cidr: value_cidr.ok_or_else(|| serde::de::Error::missing_field("CIDR"))?,
                    host_subnet_length: value_host_subnet_length.ok_or_else(|| serde::de::Error::missing_field("hostSubnetLength"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterNetworkEntry",
            &[
                "CIDR",
                "hostSubnetLength",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterNetworkEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterNetworkEntry",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "CIDR", &self.cidr)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "hostSubnetLength", &self.host_subnet_length)?;
        serde::ser::SerializeStruct::end(state)
    }
}
