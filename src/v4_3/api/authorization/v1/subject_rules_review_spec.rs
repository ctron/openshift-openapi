// Generated from definition com.github.openshift.api.authorization.v1.SubjectRulesReviewSpec

/// SubjectRulesReviewSpec adds information about how to conduct the check
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectRulesReviewSpec {
    /// Groups is optional.  Groups is the list of groups to which the User belongs.  At least one of User and Groups must be specified.
    pub groups: Vec<String>,

    /// Scopes to use for the evaluation.  Empty means "use the unscoped (full) permissions of the user/groups".
    pub scopes: Vec<String>,

    /// User is optional.  At least one of User and Groups must be specified.
    pub user: String,
}

impl<'de> serde::Deserialize<'de> for SubjectRulesReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_groups,
            Key_scopes,
            Key_user,
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
                            "groups" => Field::Key_groups,
                            "scopes" => Field::Key_scopes,
                            "user" => Field::Key_user,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubjectRulesReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SubjectRulesReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_scopes: Option<Vec<String>> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_groups => value_groups = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_scopes => value_scopes = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_user => value_user = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectRulesReviewSpec {
                    groups: value_groups.ok_or_else(|| serde::de::Error::missing_field("groups"))?,
                    scopes: value_scopes.ok_or_else(|| serde::de::Error::missing_field("scopes"))?,
                    user: value_user.ok_or_else(|| serde::de::Error::missing_field("user"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SubjectRulesReviewSpec",
            &[
                "groups",
                "scopes",
                "user",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SubjectRulesReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectRulesReviewSpec",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "groups", &self.groups)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "scopes", &self.scopes)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "user", &self.user)?;
        serde::ser::SerializeStruct::end(state)
    }
}
