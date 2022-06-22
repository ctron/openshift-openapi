// Generated from definition com.github.openshift.api.route.v1.RouteStatus

/// RouteStatus provides relevant info about the status of a route, including which routers acknowledge it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteStatus {
    /// ingress describes the places where the route may be exposed. The list of ingress points may contain duplicate Host or RouterName values. Routes are considered live once they are `Ready`
    pub ingress: Vec<crate::api::route::v1::RouteIngress>,
}

impl<'de> serde::Deserialize<'de> for RouteStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ingress,
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
                            "ingress" => Field::Key_ingress,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RouteStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RouteStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ingress: Option<Vec<crate::api::route::v1::RouteIngress>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ingress => value_ingress = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RouteStatus {
                    ingress: value_ingress.ok_or_else(|| serde::de::Error::missing_field("ingress"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RouteStatus",
            &[
                "ingress",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RouteStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RouteStatus",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", &self.ingress)?;
        serde::ser::SerializeStruct::end(state)
    }
}
