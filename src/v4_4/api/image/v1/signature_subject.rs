// Generated from definition com.github.openshift.api.image.v1.SignatureSubject

/// SignatureSubject holds information about a person or entity who created the signature.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SignatureSubject {
    /// Common name (e.g. openshift-signing-service).
    pub common_name: Option<String>,

    /// Organization name.
    pub organization: Option<String>,

    /// If present, it is a human readable key id of public key belonging to the subject used to verify image signature. It should contain at least 64 lowest bits of public key's fingerprint (e.g. 0x685ebe62bf278440).
    pub public_key_id: String,
}

impl<'de> serde::Deserialize<'de> for SignatureSubject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_common_name,
            Key_organization,
            Key_public_key_id,
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
                            "publicKeyID" => Field::Key_public_key_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignatureSubject;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SignatureSubject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_common_name: Option<String> = None;
                let mut value_organization: Option<String> = None;
                let mut value_public_key_id: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_common_name => value_common_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_organization => value_organization = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_public_key_id => value_public_key_id = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SignatureSubject {
                    common_name: value_common_name,
                    organization: value_organization,
                    public_key_id: value_public_key_id.ok_or_else(|| serde::de::Error::missing_field("publicKeyID"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SignatureSubject",
            &[
                "commonName",
                "organization",
                "publicKeyID",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SignatureSubject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SignatureSubject",
            1 +
            self.common_name.as_ref().map_or(0, |_| 1) +
            self.organization.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.common_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "commonName", value)?;
        }
        if let Some(value) = &self.organization {
            serde::ser::SerializeStruct::serialize_field(&mut state, "organization", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "publicKeyID", &self.public_key_id)?;
        serde::ser::SerializeStruct::end(state)
    }
}
