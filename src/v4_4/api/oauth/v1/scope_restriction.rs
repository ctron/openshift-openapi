// Generated from definition com.github.openshift.api.oauth.v1.ScopeRestriction

/// ScopeRestriction describe one restriction on scopes.  Exactly one option must be non-nil.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScopeRestriction {
    /// ClusterRole describes a set of restrictions for cluster role scoping.
    pub cluster_role: Option<crate::api::oauth::v1::ClusterRoleScopeRestriction>,

    /// ExactValues means the scope has to match a particular set of strings exactly
    pub literals: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for ScopeRestriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cluster_role,
            Key_literals,
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
                            "clusterRole" => Field::Key_cluster_role,
                            "literals" => Field::Key_literals,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScopeRestriction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ScopeRestriction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_cluster_role: Option<crate::api::oauth::v1::ClusterRoleScopeRestriction> = None;
                let mut value_literals: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cluster_role => value_cluster_role = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_literals => value_literals = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScopeRestriction {
                    cluster_role: value_cluster_role,
                    literals: value_literals,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScopeRestriction",
            &[
                "clusterRole",
                "literals",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ScopeRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScopeRestriction",
            self.cluster_role.as_ref().map_or(0, |_| 1) +
            self.literals.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cluster_role {
            serde::ser::SerializeStruct::serialize_field(&mut state, "clusterRole", value)?;
        }
        if let Some(value) = &self.literals {
            serde::ser::SerializeStruct::serialize_field(&mut state, "literals", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
