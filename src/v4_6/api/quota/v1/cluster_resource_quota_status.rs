// Generated from definition com.github.openshift.api.quota.v1.ClusterResourceQuotaStatus

/// ClusterResourceQuotaStatus defines the actual enforced quota and its current usage
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterResourceQuotaStatus {
    /// Namespaces slices the usage by project.  This division allows for quick resolution of deletion reconciliation inside of a single project without requiring a recalculation across all projects.  This can be used to pull the deltas for a given project.
    pub namespaces: Vec<crate::api::quota::v1::ResourceQuotaStatusByNamespace>,

    /// Total defines the actual enforced quota and its current usage across all projects
    pub total: k8s_openapi::api::core::v1::ResourceQuotaStatus,
}

impl<'de> serde::Deserialize<'de> for ClusterResourceQuotaStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_namespaces,
            Key_total,
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
                            "total" => Field::Key_total,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterResourceQuotaStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ClusterResourceQuotaStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_namespaces: Option<Vec<crate::api::quota::v1::ResourceQuotaStatusByNamespace>> = None;
                let mut value_total: Option<k8s_openapi::api::core::v1::ResourceQuotaStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_namespaces => value_namespaces = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_total => value_total = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterResourceQuotaStatus {
                    namespaces: value_namespaces.ok_or_else(|| serde::de::Error::missing_field("namespaces"))?,
                    total: value_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterResourceQuotaStatus",
            &[
                "namespaces",
                "total",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterResourceQuotaStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterResourceQuotaStatus",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "namespaces", &self.namespaces)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "total", &self.total)?;
        serde::ser::SerializeStruct::end(state)
    }
}
