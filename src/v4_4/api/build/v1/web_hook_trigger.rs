// Generated from definition com.github.openshift.api.build.v1.WebHookTrigger

/// WebHookTrigger is a trigger that gets invoked using a webhook type of post
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebHookTrigger {
    /// allowEnv determines whether the webhook can set environment variables; can only be set to true for GenericWebHook.
    pub allow_env: Option<bool>,

    /// secret used to validate requests. Deprecated: use SecretReference instead.
    pub secret: Option<String>,

    /// secretReference is a reference to a secret in the same namespace, containing the value to be validated when the webhook is invoked. The secret being referenced must contain a key named "WebHookSecretKey", the value of which will be checked against the value supplied in the webhook invocation.
    pub secret_reference: Option<crate::api::build::v1::SecretLocalReference>,
}

impl<'de> serde::Deserialize<'de> for WebHookTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_env,
            Key_secret,
            Key_secret_reference,
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
                            "allowEnv" => Field::Key_allow_env,
                            "secret" => Field::Key_secret,
                            "secretReference" => Field::Key_secret_reference,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebHookTrigger;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WebHookTrigger")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allow_env: Option<bool> = None;
                let mut value_secret: Option<String> = None;
                let mut value_secret_reference: Option<crate::api::build::v1::SecretLocalReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_env => value_allow_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_reference => value_secret_reference = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WebHookTrigger {
                    allow_env: value_allow_env,
                    secret: value_secret,
                    secret_reference: value_secret_reference,
                })
            }
        }

        deserializer.deserialize_struct(
            "WebHookTrigger",
            &[
                "allowEnv",
                "secret",
                "secretReference",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for WebHookTrigger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WebHookTrigger",
            self.allow_env.as_ref().map_or(0, |_| 1) +
            self.secret.as_ref().map_or(0, |_| 1) +
            self.secret_reference.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allow_env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowEnv", value)?;
        }
        if let Some(value) = &self.secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        if let Some(value) = &self.secret_reference {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secretReference", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
