// Generated from definition com.github.openshift.api.quota.v1.ClusterResourceQuotaSpec

/// ClusterResourceQuotaSpec defines the desired quota restrictions
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterResourceQuotaSpec {
    /// Quota defines the desired quota
    pub quota: k8s_openapi::api::core::v1::ResourceQuotaSpec,

    /// Selector is the selector used to match projects. It should only select active projects on the scale of dozens (though it can select many more less active projects).  These projects will contend on object creation through this resource.
    pub selector: crate::api::quota::v1::ClusterResourceQuotaSelector,
}

impl<'de> serde::Deserialize<'de> for ClusterResourceQuotaSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_quota,
            Key_selector,
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
                            "quota" => Field::Key_quota,
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterResourceQuotaSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ClusterResourceQuotaSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_quota: Option<k8s_openapi::api::core::v1::ResourceQuotaSpec> = None;
                let mut value_selector: Option<crate::api::quota::v1::ClusterResourceQuotaSelector> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_quota => value_quota = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_selector => value_selector = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterResourceQuotaSpec {
                    quota: value_quota.ok_or_else(|| serde::de::Error::missing_field("quota"))?,
                    selector: value_selector.ok_or_else(|| serde::de::Error::missing_field("selector"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterResourceQuotaSpec",
            &[
                "quota",
                "selector",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterResourceQuotaSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterResourceQuotaSpec",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "quota", &self.quota)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        serde::ser::SerializeStruct::end(state)
    }
}
