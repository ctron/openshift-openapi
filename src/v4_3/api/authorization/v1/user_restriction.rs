// Generated from definition com.github.openshift.api.authorization.v1.UserRestriction

/// UserRestriction matches a user either by a string match on the user name, a string match on the name of a group to which the user belongs, or a label selector applied to the user labels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserRestriction {
    /// Groups specifies a list of literal group names.
    pub groups: Vec<String>,

    /// Selectors specifies a list of label selectors over user labels.
    pub labels: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Users specifies a list of literal user names.
    pub users: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for UserRestriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_groups,
            Key_labels,
            Key_users,
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
                            "labels" => Field::Key_labels,
                            "users" => Field::Key_users,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UserRestriction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("UserRestriction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_labels: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>> = None;
                let mut value_users: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_groups => value_groups = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_labels => value_labels = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_users => value_users = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(UserRestriction {
                    groups: value_groups.ok_or_else(|| serde::de::Error::missing_field("groups"))?,
                    labels: value_labels.ok_or_else(|| serde::de::Error::missing_field("labels"))?,
                    users: value_users.ok_or_else(|| serde::de::Error::missing_field("users"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "UserRestriction",
            &[
                "groups",
                "labels",
                "users",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for UserRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "UserRestriction",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "groups", &self.groups)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "labels", &self.labels)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "users", &self.users)?;
        serde::ser::SerializeStruct::end(state)
    }
}
