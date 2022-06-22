// Generated from definition com.github.openshift.api.security.v1.IDRange

/// IDRange provides a min/max of an allowed range of IDs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IDRange {
    /// Max is the end of the range, inclusive.
    pub max: Option<i64>,

    /// Min is the start of the range, inclusive.
    pub min: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for IDRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max,
            Key_min,
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
                            "max" => Field::Key_max,
                            "min" => Field::Key_min,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IDRange;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IDRange")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_max: Option<i64> = None;
                let mut value_min: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max => value_max = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min => value_min = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IDRange {
                    max: value_max,
                    min: value_min,
                })
            }
        }

        deserializer.deserialize_struct(
            "IDRange",
            &[
                "max",
                "min",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for IDRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IDRange",
            self.max.as_ref().map_or(0, |_| 1) +
            self.min.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max {
            serde::ser::SerializeStruct::serialize_field(&mut state, "max", value)?;
        }
        if let Some(value) = &self.min {
            serde::ser::SerializeStruct::serialize_field(&mut state, "min", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
