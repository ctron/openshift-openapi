// Generated from definition com.github.openshift.api.image.v1.NamedTagEventList

/// NamedTagEventList relates a tag to its image history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamedTagEventList {
    /// Conditions is an array of conditions that apply to the tag event list.
    pub conditions: Option<Vec<crate::api::image::v1::TagEventCondition>>,

    /// Standard object's metadata.
    pub items: Vec<crate::api::image::v1::TagEvent>,

    /// Tag is the tag for which the history is recorded
    pub tag: String,
}

impl<'de> serde::Deserialize<'de> for NamedTagEventList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_items,
            Key_tag,
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
                            "conditions" => Field::Key_conditions,
                            "items" => Field::Key_items,
                            "tag" => Field::Key_tag,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NamedTagEventList;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NamedTagEventList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::image::v1::TagEventCondition>> = None;
                let mut value_items: Option<Vec<crate::api::image::v1::TagEvent>> = None;
                let mut value_tag: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_tag => value_tag = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamedTagEventList {
                    conditions: value_conditions,
                    items: value_items.ok_or_else(|| serde::de::Error::missing_field("items"))?,
                    tag: value_tag.ok_or_else(|| serde::de::Error::missing_field("tag"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NamedTagEventList",
            &[
                "conditions",
                "items",
                "tag",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for NamedTagEventList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamedTagEventList",
            2 +
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "items", &self.items)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "tag", &self.tag)?;
        serde::ser::SerializeStruct::end(state)
    }
}
