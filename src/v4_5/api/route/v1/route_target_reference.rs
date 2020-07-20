// Generated from definition com.github.openshift.api.route.v1.RouteTargetReference

/// RouteTargetReference specifies the target that resolve into endpoints. Only the 'Service' kind is allowed. Use 'weight' field to emphasize one over others.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteTargetReference {
    /// The kind of target that the route is referring to. Currently, only 'Service' is allowed
    pub kind: String,

    /// name of the service/target that is being referred to. e.g. name of the service
    pub name: String,

    /// weight as an integer between 0 and 256, default 1, that specifies the target's relative weight against other target reference objects. 0 suppresses requests to this backend.
    pub weight: i32,
}

impl<'de> serde::Deserialize<'de> for RouteTargetReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_kind,
            Key_name,
            Key_weight,
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
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "weight" => Field::Key_weight,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RouteTargetReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RouteTargetReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_weight: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_kind => value_kind = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_weight => value_weight = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RouteTargetReference {
                    kind: value_kind.ok_or_else(|| serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    weight: value_weight.ok_or_else(|| serde::de::Error::missing_field("weight"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RouteTargetReference",
            &[
                "kind",
                "name",
                "weight",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RouteTargetReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RouteTargetReference",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "weight", &self.weight)?;
        serde::ser::SerializeStruct::end(state)
    }
}
