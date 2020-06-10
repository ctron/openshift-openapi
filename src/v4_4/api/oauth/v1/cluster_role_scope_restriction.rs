// Generated from definition com.github.openshift.api.oauth.v1.ClusterRoleScopeRestriction

/// ClusterRoleScopeRestriction describes restrictions on cluster role scopes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRoleScopeRestriction {
    /// AllowEscalation indicates whether you can request roles and their escalating resources
    pub allow_escalation: bool,

    /// Namespaces is the list of namespaces that can be referenced.  * means any of them (including *)
    pub namespaces: Vec<String>,

    /// RoleNames is the list of cluster roles that can referenced.  * means anything
    pub role_names: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for ClusterRoleScopeRestriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_escalation,
            Key_namespaces,
            Key_role_names,
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
                            "allowEscalation" => Field::Key_allow_escalation,
                            "namespaces" => Field::Key_namespaces,
                            "roleNames" => Field::Key_role_names,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRoleScopeRestriction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ClusterRoleScopeRestriction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allow_escalation: Option<bool> = None;
                let mut value_namespaces: Option<Vec<String>> = None;
                let mut value_role_names: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_escalation => value_allow_escalation = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespaces => value_namespaces = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_role_names => value_role_names = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterRoleScopeRestriction {
                    allow_escalation: value_allow_escalation.ok_or_else(|| serde::de::Error::missing_field("allowEscalation"))?,
                    namespaces: value_namespaces.ok_or_else(|| serde::de::Error::missing_field("namespaces"))?,
                    role_names: value_role_names.ok_or_else(|| serde::de::Error::missing_field("roleNames"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterRoleScopeRestriction",
            &[
                "allowEscalation",
                "namespaces",
                "roleNames",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterRoleScopeRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterRoleScopeRestriction",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowEscalation", &self.allow_escalation)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "namespaces", &self.namespaces)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "roleNames", &self.role_names)?;
        serde::ser::SerializeStruct::end(state)
    }
}
