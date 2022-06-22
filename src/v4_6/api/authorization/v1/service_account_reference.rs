// Generated from definition com.github.openshift.api.authorization.v1.ServiceAccountReference

/// ServiceAccountReference specifies a service account and namespace by their names.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountReference {
    /// Name is the name of the service account.
    pub name: String,

    /// Namespace is the namespace of the service account.  Service accounts from inside the whitelisted namespaces are allowed to be bound to roles.  If Namespace is empty, then the namespace of the RoleBindingRestriction in which the ServiceAccountReference is embedded is used.
    pub namespace: String,
}

impl<'de> serde::Deserialize<'de> for ServiceAccountReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_namespace,
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
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceAccountReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceAccountReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace => value_namespace = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceAccountReference {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    namespace: value_namespace.ok_or_else(|| serde::de::Error::missing_field("namespace"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceAccountReference",
            &[
                "name",
                "namespace",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ServiceAccountReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceAccountReference",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", &self.namespace)?;
        serde::ser::SerializeStruct::end(state)
    }
}
