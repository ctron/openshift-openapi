// Generated from definition com.github.openshift.api.route.v1.TLSConfig

/// TLSConfig defines config used to secure a route and provide termination
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TLSConfig {
    /// caCertificate provides the cert authority certificate contents
    pub ca_certificate: Option<String>,

    /// certificate provides certificate contents
    pub certificate: Option<String>,

    /// destinationCACertificate provides the contents of the ca certificate of the final destination.  When using reencrypt termination this file should be provided in order to have routers use it for health checks on the secure connection. If this field is not specified, the router may provide its own destination CA and perform hostname validation using the short service name (service.namespace.svc), which allows infrastructure generated certificates to automatically verify.
    pub destination_ca_certificate: Option<String>,

    /// insecureEdgeTerminationPolicy indicates the desired behavior for insecure connections to a route. While each router may make its own decisions on which ports to expose, this is normally port 80.
    ///
    /// * Allow - traffic is sent to the server on the insecure port (default) * Disable - no traffic is allowed on the insecure port. * Redirect - clients are redirected to the secure port.
    pub insecure_edge_termination_policy: Option<String>,

    /// key provides key file contents
    pub key: Option<String>,

    /// termination indicates termination type.
    pub termination: String,
}

impl<'de> serde::Deserialize<'de> for TLSConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ca_certificate,
            Key_certificate,
            Key_destination_ca_certificate,
            Key_insecure_edge_termination_policy,
            Key_key,
            Key_termination,
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
                            "caCertificate" => Field::Key_ca_certificate,
                            "certificate" => Field::Key_certificate,
                            "destinationCACertificate" => Field::Key_destination_ca_certificate,
                            "insecureEdgeTerminationPolicy" => Field::Key_insecure_edge_termination_policy,
                            "key" => Field::Key_key,
                            "termination" => Field::Key_termination,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TLSConfig;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TLSConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ca_certificate: Option<String> = None;
                let mut value_certificate: Option<String> = None;
                let mut value_destination_ca_certificate: Option<String> = None;
                let mut value_insecure_edge_termination_policy: Option<String> = None;
                let mut value_key: Option<String> = None;
                let mut value_termination: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ca_certificate => value_ca_certificate = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_certificate => value_certificate = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_destination_ca_certificate => value_destination_ca_certificate = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_insecure_edge_termination_policy => value_insecure_edge_termination_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination => value_termination = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TLSConfig {
                    ca_certificate: value_ca_certificate,
                    certificate: value_certificate,
                    destination_ca_certificate: value_destination_ca_certificate,
                    insecure_edge_termination_policy: value_insecure_edge_termination_policy,
                    key: value_key,
                    termination: value_termination.ok_or_else(|| serde::de::Error::missing_field("termination"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TLSConfig",
            &[
                "caCertificate",
                "certificate",
                "destinationCACertificate",
                "insecureEdgeTerminationPolicy",
                "key",
                "termination",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TLSConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TLSConfig",
            1 +
            self.ca_certificate.as_ref().map_or(0, |_| 1) +
            self.certificate.as_ref().map_or(0, |_| 1) +
            self.destination_ca_certificate.as_ref().map_or(0, |_| 1) +
            self.insecure_edge_termination_policy.as_ref().map_or(0, |_| 1) +
            self.key.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ca_certificate {
            serde::ser::SerializeStruct::serialize_field(&mut state, "caCertificate", value)?;
        }
        if let Some(value) = &self.certificate {
            serde::ser::SerializeStruct::serialize_field(&mut state, "certificate", value)?;
        }
        if let Some(value) = &self.destination_ca_certificate {
            serde::ser::SerializeStruct::serialize_field(&mut state, "destinationCACertificate", value)?;
        }
        if let Some(value) = &self.insecure_edge_termination_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "insecureEdgeTerminationPolicy", value)?;
        }
        if let Some(value) = &self.key {
            serde::ser::SerializeStruct::serialize_field(&mut state, "key", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "termination", &self.termination)?;
        serde::ser::SerializeStruct::end(state)
    }
}
