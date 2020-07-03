// Generated from definition com.github.openshift.api.authorization.v1.GroupRestriction

/// GroupRestriction matches a group either by a string match on the group name or a label selector applied to group labels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroupRestriction {
    /// Groups is a list of groups used to match against an individual user's groups. If the user is a member of one of the whitelisted groups, the user is allowed to be bound to a role.
    pub groups: Vec<String>,

    /// Selectors specifies a list of label selectors over group labels.
    pub labels: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl<'de> serde::Deserialize<'de> for GroupRestriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_groups,
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
                            "groups" => Field::Key_groups,
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
            type Value = GroupRestriction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GroupRestriction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_labels: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_groups => value_groups = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_labels => value_labels = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GroupRestriction {
                    groups: value_groups.ok_or_else(|| serde::de::Error::missing_field("groups"))?,
                    labels: value_labels.ok_or_else(|| serde::de::Error::missing_field("labels"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "GroupRestriction",
            &[
                "groups",
                "labels",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for GroupRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GroupRestriction",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "groups", &self.groups)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "labels", &self.labels)?;
        serde::ser::SerializeStruct::end(state)
    }
}
