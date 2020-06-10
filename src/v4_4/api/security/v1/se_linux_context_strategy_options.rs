// Generated from definition com.github.openshift.api.security.v1.SELinuxContextStrategyOptions

/// SELinuxContextStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SELinuxContextStrategyOptions {
    /// seLinuxOptions required to run as; required for MustRunAs
    pub se_linux_options: Option<k8s_openapi::api::core::v1::SELinuxOptions>,

    /// Type is the strategy that will dictate what SELinux context is used in the SecurityContext.
    pub type_: Option<String>,
}

impl<'de> serde::Deserialize<'de> for SELinuxContextStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_se_linux_options,
            Key_type_,
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
                            "seLinuxOptions" => Field::Key_se_linux_options,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SELinuxContextStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SELinuxContextStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_se_linux_options: Option<k8s_openapi::api::core::v1::SELinuxOptions> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_se_linux_options => value_se_linux_options = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SELinuxContextStrategyOptions {
                    se_linux_options: value_se_linux_options,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "SELinuxContextStrategyOptions",
            &[
                "seLinuxOptions",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SELinuxContextStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SELinuxContextStrategyOptions",
            self.se_linux_options.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.se_linux_options {
            serde::ser::SerializeStruct::serialize_field(&mut state, "seLinuxOptions", value)?;
        }
        if let Some(value) = &self.type_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
