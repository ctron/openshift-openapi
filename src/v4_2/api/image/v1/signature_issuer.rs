// Generated from definition com.github.openshift.api.image.v1.SignatureIssuer

/// SignatureIssuer holds information about an issuer of signing certificate or key.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SignatureIssuer {
    /// Common name (e.g. openshift-signing-service).
    pub common_name: Option<String>,

    /// Organization name.
    pub organization: Option<String>,
}

impl<'de> serde::Deserialize<'de> for SignatureIssuer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_common_name,
            Key_organization,
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
                            "commonName" => Field::Key_common_name,
                            "organization" => Field::Key_organization,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignatureIssuer;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SignatureIssuer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_common_name: Option<String> = None;
                let mut value_organization: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_common_name => value_common_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_organization => value_organization = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SignatureIssuer {
                    common_name: value_common_name,
                    organization: value_organization,
                })
            }
        }

        deserializer.deserialize_struct(
            "SignatureIssuer",
            &[
                "commonName",
                "organization",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SignatureIssuer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SignatureIssuer",
            self.common_name.as_ref().map_or(0, |_| 1) +
            self.organization.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.common_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "commonName", value)?;
        }
        if let Some(value) = &self.organization {
            serde::ser::SerializeStruct::serialize_field(&mut state, "organization", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
