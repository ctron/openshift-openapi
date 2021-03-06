// Generated from definition com.github.openshift.api.security.v1.FSGroupStrategyOptions

/// FSGroupStrategyOptions defines the strategy type and options used to create the strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FSGroupStrategyOptions {
    /// Ranges are the allowed ranges of fs groups.  If you would like to force a single fs group then supply a single range with the same start and end.
    pub ranges: Option<Vec<crate::api::security::v1::IDRange>>,

    /// Type is the strategy that will dictate what FSGroup is used in the SecurityContext.
    pub type_: Option<String>,
}

impl<'de> serde::Deserialize<'de> for FSGroupStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ranges,
            Key_type_,
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
                            "ranges" => Field::Key_ranges,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FSGroupStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FSGroupStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ranges: Option<Vec<crate::api::security::v1::IDRange>> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ranges => value_ranges = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FSGroupStrategyOptions {
                    ranges: value_ranges,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "FSGroupStrategyOptions",
            &[
                "ranges",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for FSGroupStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FSGroupStrategyOptions",
            self.ranges.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ranges {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ranges", value)?;
        }
        if let Some(value) = &self.type_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
