// Generated from definition com.github.openshift.api.quota.v1.ClusterResourceQuotaSelector

/// ClusterResourceQuotaSelector is used to select projects.  At least one of LabelSelector or AnnotationSelector must present.  If only one is present, it is the only selection criteria.  If both are specified, the project must match both restrictions.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterResourceQuotaSelector {
    /// AnnotationSelector is used to select projects by annotation.
    pub annotations: std::collections::BTreeMap<String, String>,

    /// LabelSelector is used to select projects by label.
    pub labels: k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
}

impl<'de> serde::Deserialize<'de> for ClusterResourceQuotaSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_annotations,
            Key_labels,
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
                            "annotations" => Field::Key_annotations,
                            "labels" => Field::Key_labels,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterResourceQuotaSelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ClusterResourceQuotaSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_annotations: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_labels: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_annotations => value_annotations = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_labels => value_labels = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterResourceQuotaSelector {
                    annotations: value_annotations.ok_or_else(|| serde::de::Error::missing_field("annotations"))?,
                    labels: value_labels.ok_or_else(|| serde::de::Error::missing_field("labels"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterResourceQuotaSelector",
            &[
                "annotations",
                "labels",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterResourceQuotaSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterResourceQuotaSelector",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "annotations", &self.annotations)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "labels", &self.labels)?;
        serde::ser::SerializeStruct::end(state)
    }
}
