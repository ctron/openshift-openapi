// Generated from definition com.github.openshift.api.template.v1.TemplateInstanceRequester

/// TemplateInstanceRequester holds the identity of an agent requesting a template instantiation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TemplateInstanceRequester {
    /// extra holds additional information provided by the authenticator.
    pub extra: Option<std::collections::BTreeMap<String, Vec<String>>>,

    /// groups represent the groups this user is a part of.
    pub groups: Option<Vec<String>>,

    /// uid is a unique value that identifies this user across time; if this user is deleted and another user by the same name is added, they will have different UIDs.
    pub uid: Option<String>,

    /// username uniquely identifies this user among all active users.
    pub username: Option<String>,
}

impl<'de> serde::Deserialize<'de> for TemplateInstanceRequester {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_extra,
            Key_groups,
            Key_uid,
            Key_username,
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
                            "extra" => Field::Key_extra,
                            "groups" => Field::Key_groups,
                            "uid" => Field::Key_uid,
                            "username" => Field::Key_username,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TemplateInstanceRequester;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TemplateInstanceRequester")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_extra: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_uid: Option<String> = None;
                let mut value_username: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_username => value_username = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TemplateInstanceRequester {
                    extra: value_extra,
                    groups: value_groups,
                    uid: value_uid,
                    username: value_username,
                })
            }
        }

        deserializer.deserialize_struct(
            "TemplateInstanceRequester",
            &[
                "extra",
                "groups",
                "uid",
                "username",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TemplateInstanceRequester {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TemplateInstanceRequester",
            self.extra.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.username.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.extra {
            serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.groups {
            serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        if let Some(value) = &self.uid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.username {
            serde::ser::SerializeStruct::serialize_field(&mut state, "username", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
