// Generated from definition com.github.openshift.api.security.v1.RunAsUserStrategyOptions

/// RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunAsUserStrategyOptions {
    /// Type is the strategy that will dictate what RunAsUser is used in the SecurityContext.
    pub type_: Option<String>,

    /// UID is the user id that containers must run as.  Required for the MustRunAs strategy if not using namespace/service account allocated uids.
    pub uid: Option<i64>,

    /// UIDRangeMax defines the max value for a strategy that allocates by range.
    pub uid_range_max: Option<i64>,

    /// UIDRangeMin defines the min value for a strategy that allocates by range.
    pub uid_range_min: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for RunAsUserStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_type_,
            Key_uid,
            Key_uid_range_max,
            Key_uid_range_min,
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
                            "type" => Field::Key_type_,
                            "uid" => Field::Key_uid,
                            "uidRangeMax" => Field::Key_uid_range_max,
                            "uidRangeMin" => Field::Key_uid_range_min,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunAsUserStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RunAsUserStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_type_: Option<String> = None;
                let mut value_uid: Option<i64> = None;
                let mut value_uid_range_max: Option<i64> = None;
                let mut value_uid_range_min: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_type_ => value_type_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid_range_max => value_uid_range_max = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid_range_min => value_uid_range_min = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RunAsUserStrategyOptions {
                    type_: value_type_,
                    uid: value_uid,
                    uid_range_max: value_uid_range_max,
                    uid_range_min: value_uid_range_min,
                })
            }
        }

        deserializer.deserialize_struct(
            "RunAsUserStrategyOptions",
            &[
                "type",
                "uid",
                "uidRangeMax",
                "uidRangeMin",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RunAsUserStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RunAsUserStrategyOptions",
            self.type_.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.uid_range_max.as_ref().map_or(0, |_| 1) +
            self.uid_range_min.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.type_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        if let Some(value) = &self.uid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.uid_range_max {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uidRangeMax", value)?;
        }
        if let Some(value) = &self.uid_range_min {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uidRangeMin", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
