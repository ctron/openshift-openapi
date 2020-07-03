// Generated from definition com.github.openshift.api.route.v1.RouteIngress

/// RouteIngress holds information about the places where a route is exposed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteIngress {
    /// Conditions is the state of the route, may be empty.
    pub conditions: Option<Vec<crate::api::route::v1::RouteIngressCondition>>,

    /// Host is the host string under which the route is exposed; this value is required
    pub host: Option<String>,

    /// CanonicalHostname is the external host name for the router that can be used as a CNAME for the host requested for this route. This value is optional and may not be set in all cases.
    pub router_canonical_hostname: Option<String>,

    /// Name is a name chosen by the router to identify itself; this value is required
    pub router_name: Option<String>,

    /// Wildcard policy is the wildcard policy that was allowed where this route is exposed.
    pub wildcard_policy: Option<String>,
}

impl<'de> serde::Deserialize<'de> for RouteIngress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_host,
            Key_router_canonical_hostname,
            Key_router_name,
            Key_wildcard_policy,
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
                            "host" => Field::Key_host,
                            "routerCanonicalHostname" => Field::Key_router_canonical_hostname,
                            "routerName" => Field::Key_router_name,
                            "wildcardPolicy" => Field::Key_wildcard_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RouteIngress;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RouteIngress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::route::v1::RouteIngressCondition>> = None;
                let mut value_host: Option<String> = None;
                let mut value_router_canonical_hostname: Option<String> = None;
                let mut value_router_name: Option<String> = None;
                let mut value_wildcard_policy: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host => value_host = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_router_canonical_hostname => value_router_canonical_hostname = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_router_name => value_router_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_wildcard_policy => value_wildcard_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RouteIngress {
                    conditions: value_conditions,
                    host: value_host,
                    router_canonical_hostname: value_router_canonical_hostname,
                    router_name: value_router_name,
                    wildcard_policy: value_wildcard_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "RouteIngress",
            &[
                "conditions",
                "host",
                "routerCanonicalHostname",
                "routerName",
                "wildcardPolicy",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RouteIngress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RouteIngress",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.host.as_ref().map_or(0, |_| 1) +
            self.router_canonical_hostname.as_ref().map_or(0, |_| 1) +
            self.router_name.as_ref().map_or(0, |_| 1) +
            self.wildcard_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.host {
            serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        if let Some(value) = &self.router_canonical_hostname {
            serde::ser::SerializeStruct::serialize_field(&mut state, "routerCanonicalHostname", value)?;
        }
        if let Some(value) = &self.router_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "routerName", value)?;
        }
        if let Some(value) = &self.wildcard_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "wildcardPolicy", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
