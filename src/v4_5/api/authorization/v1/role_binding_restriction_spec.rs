// Generated from definition com.github.openshift.api.authorization.v1.RoleBindingRestrictionSpec

/// RoleBindingRestrictionSpec defines a rolebinding restriction.  Exactly one field must be non-nil.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoleBindingRestrictionSpec {
    /// GroupRestriction matches against group subjects.
    pub grouprestriction: crate::api::authorization::v1::GroupRestriction,

    /// ServiceAccountRestriction matches against service-account subjects.
    pub serviceaccountrestriction: crate::api::authorization::v1::ServiceAccountRestriction,

    /// UserRestriction matches against user subjects.
    pub userrestriction: crate::api::authorization::v1::UserRestriction,
}

impl<'de> serde::Deserialize<'de> for RoleBindingRestrictionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_grouprestriction,
            Key_serviceaccountrestriction,
            Key_userrestriction,
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
                            "grouprestriction" => Field::Key_grouprestriction,
                            "serviceaccountrestriction" => Field::Key_serviceaccountrestriction,
                            "userrestriction" => Field::Key_userrestriction,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RoleBindingRestrictionSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RoleBindingRestrictionSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_grouprestriction: Option<crate::api::authorization::v1::GroupRestriction> = None;
                let mut value_serviceaccountrestriction: Option<crate::api::authorization::v1::ServiceAccountRestriction> = None;
                let mut value_userrestriction: Option<crate::api::authorization::v1::UserRestriction> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_grouprestriction => value_grouprestriction = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_serviceaccountrestriction => value_serviceaccountrestriction = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_userrestriction => value_userrestriction = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RoleBindingRestrictionSpec {
                    grouprestriction: value_grouprestriction.ok_or_else(|| serde::de::Error::missing_field("grouprestriction"))?,
                    serviceaccountrestriction: value_serviceaccountrestriction.ok_or_else(|| serde::de::Error::missing_field("serviceaccountrestriction"))?,
                    userrestriction: value_userrestriction.ok_or_else(|| serde::de::Error::missing_field("userrestriction"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RoleBindingRestrictionSpec",
            &[
                "grouprestriction",
                "serviceaccountrestriction",
                "userrestriction",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RoleBindingRestrictionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RoleBindingRestrictionSpec",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "grouprestriction", &self.grouprestriction)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "serviceaccountrestriction", &self.serviceaccountrestriction)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "userrestriction", &self.userrestriction)?;
        serde::ser::SerializeStruct::end(state)
    }
}
