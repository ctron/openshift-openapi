// Generated from definition com.github.openshift.api.quota.v1.ResourceQuotaStatusByNamespace

/// ResourceQuotaStatusByNamespace gives status for a particular project
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceQuotaStatusByNamespace {
    /// Namespace the project this status applies to
    pub namespace: String,

    /// Status indicates how many resources have been consumed by this project
    pub status: k8s_openapi::api::core::v1::ResourceQuotaStatus,
}

impl<'de> serde::Deserialize<'de> for ResourceQuotaStatusByNamespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_namespace,
            Key_status,
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
                            "namespace" => Field::Key_namespace,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourceQuotaStatusByNamespace;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceQuotaStatusByNamespace")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_namespace: Option<String> = None;
                let mut value_status: Option<k8s_openapi::api::core::v1::ResourceQuotaStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_namespace => value_namespace = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_status => value_status = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceQuotaStatusByNamespace {
                    namespace: value_namespace.ok_or_else(|| serde::de::Error::missing_field("namespace"))?,
                    status: value_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceQuotaStatusByNamespace",
            &[
                "namespace",
                "status",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ResourceQuotaStatusByNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceQuotaStatusByNamespace",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", &self.namespace)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        serde::ser::SerializeStruct::end(state)
    }
}
