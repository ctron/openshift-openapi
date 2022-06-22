// Generated from definition com.github.openshift.api.route.v1.RouteSpec

/// RouteSpec describes the hostname or path the route exposes, any security information, and one to four backends (services) the route points to. Requests are distributed among the backends depending on the weights assigned to each backend. When using roundrobin scheduling the portion of requests that go to each backend is the backend weight divided by the sum of all of the backend weights. When the backend has more than one endpoint the requests that end up on the backend are roundrobin distributed among the endpoints. Weights are between 0 and 256 with default 1. Weight 0 causes no requests to the backend. If all weights are zero the route will be considered to have no backends and return a standard 503 response.
///
/// The `tls` field is optional and allows specific certificates or behavior for the route. Routers typically configure a default certificate on a wildcard domain to terminate routes without explicit certificates, but custom hostnames usually must choose passthrough (send traffic directly to the backend via the TLS Server-Name- Indication field) or provide a certificate.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteSpec {
    /// alternateBackends allows up to 3 additional backends to be assigned to the route. Only the Service kind is allowed, and it will be defaulted to Service. Use the weight field in RouteTargetReference object to specify relative preference.
    pub alternate_backends: Option<Vec<crate::api::route::v1::RouteTargetReference>>,

    /// host is an alias/DNS that points to the service. Optional. If not specified a route name will typically be automatically chosen. Must follow DNS952 subdomain conventions.
    pub host: String,

    /// Path that the router watches for, to route traffic for to the service. Optional
    pub path: Option<String>,

    /// If specified, the port to be used by the router. Most routers will use all endpoints exposed by the service by default - set this value to instruct routers which port to use.
    pub port: Option<crate::api::route::v1::RoutePort>,

    /// The tls field provides the ability to configure certificates and termination for the route.
    pub tls: Option<crate::api::route::v1::TLSConfig>,

    /// to is an object the route should use as the primary backend. Only the Service kind is allowed, and it will be defaulted to Service. If the weight field (0-256 default 1) is set to zero, no traffic will be sent to this backend.
    pub to: crate::api::route::v1::RouteTargetReference,

    /// Wildcard policy if any for the route. Currently only 'Subdomain' or 'None' is allowed.
    pub wildcard_policy: Option<String>,
}

impl<'de> serde::Deserialize<'de> for RouteSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_alternate_backends,
            Key_host,
            Key_path,
            Key_port,
            Key_tls,
            Key_to,
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
                            "alternateBackends" => Field::Key_alternate_backends,
                            "host" => Field::Key_host,
                            "path" => Field::Key_path,
                            "port" => Field::Key_port,
                            "tls" => Field::Key_tls,
                            "to" => Field::Key_to,
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
            type Value = RouteSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RouteSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_alternate_backends: Option<Vec<crate::api::route::v1::RouteTargetReference>> = None;
                let mut value_host: Option<String> = None;
                let mut value_path: Option<String> = None;
                let mut value_port: Option<crate::api::route::v1::RoutePort> = None;
                let mut value_tls: Option<crate::api::route::v1::TLSConfig> = None;
                let mut value_to: Option<crate::api::route::v1::RouteTargetReference> = None;
                let mut value_wildcard_policy: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_alternate_backends => value_alternate_backends = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host => value_host = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_path => value_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tls => value_tls = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_to => value_to = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_wildcard_policy => value_wildcard_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RouteSpec {
                    alternate_backends: value_alternate_backends,
                    host: value_host.ok_or_else(|| serde::de::Error::missing_field("host"))?,
                    path: value_path,
                    port: value_port,
                    tls: value_tls,
                    to: value_to.ok_or_else(|| serde::de::Error::missing_field("to"))?,
                    wildcard_policy: value_wildcard_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "RouteSpec",
            &[
                "alternateBackends",
                "host",
                "path",
                "port",
                "tls",
                "to",
                "wildcardPolicy",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RouteSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RouteSpec",
            2 +
            self.alternate_backends.as_ref().map_or(0, |_| 1) +
            self.path.as_ref().map_or(0, |_| 1) +
            self.port.as_ref().map_or(0, |_| 1) +
            self.tls.as_ref().map_or(0, |_| 1) +
            self.wildcard_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.alternate_backends {
            serde::ser::SerializeStruct::serialize_field(&mut state, "alternateBackends", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "host", &self.host)?;
        if let Some(value) = &self.path {
            serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.port {
            serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        if let Some(value) = &self.tls {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tls", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "to", &self.to)?;
        if let Some(value) = &self.wildcard_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "wildcardPolicy", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
