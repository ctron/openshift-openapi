// Generated from definition com.github.openshift.api.build.v1.SecretSpec

/// SecretSpec specifies a secret to be included in a build pod and its corresponding mount point
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecretSpec {
    /// mountPath is the path at which to mount the secret
    pub mount_path: String,

    /// secretSource is a reference to the secret
    pub secret_source: k8s_openapi::api::core::v1::LocalObjectReference,
}

impl<'de> serde::Deserialize<'de> for SecretSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_mount_path,
            Key_secret_source,
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
                            "mountPath" => Field::Key_mount_path,
                            "secretSource" => Field::Key_secret_source,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecretSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SecretSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_mount_path: Option<String> = None;
                let mut value_secret_source: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_mount_path => value_mount_path = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_secret_source => value_secret_source = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecretSpec {
                    mount_path: value_mount_path.ok_or_else(|| serde::de::Error::missing_field("mountPath"))?,
                    secret_source: value_secret_source.ok_or_else(|| serde::de::Error::missing_field("secretSource"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecretSpec",
            &[
                "mountPath",
                "secretSource",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SecretSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecretSpec",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "mountPath", &self.mount_path)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "secretSource", &self.secret_source)?;
        serde::ser::SerializeStruct::end(state)
    }
}
