// Generated from definition com.github.openshift.api.route.v1.RoutePort

/// RoutePort defines a port mapping from a router to an endpoint in the service endpoints.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoutePort {
    /// The target port on pods selected by the service this route points to. If this is a string, it will be looked up as a named port in the target endpoints port list. Required
    pub target_port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
}

impl<'de> serde::Deserialize<'de> for RoutePort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_target_port,
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
                            "targetPort" => Field::Key_target_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RoutePort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RoutePort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_target_port: Option<k8s_openapi::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_target_port => value_target_port = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RoutePort {
                    target_port: value_target_port.ok_or_else(|| serde::de::Error::missing_field("targetPort"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RoutePort",
            &[
                "targetPort",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RoutePort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RoutePort",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "targetPort", &self.target_port)?;
        serde::ser::SerializeStruct::end(state)
    }
}
