// Generated from definition com.github.openshift.api.apps.v1.CustomDeploymentStrategyParams

/// CustomDeploymentStrategyParams are the input to the Custom deployment strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomDeploymentStrategyParams {
    /// Command is optional and overrides CMD in the container Image.
    pub command: Option<Vec<String>>,

    /// Environment holds the environment which will be given to the container for Image.
    pub environment: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// Image specifies a container image which can carry out a deployment.
    pub image: Option<String>,
}

impl<'de> serde::Deserialize<'de> for CustomDeploymentStrategyParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_command,
            Key_environment,
            Key_image,
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
                            "command" => Field::Key_command,
                            "environment" => Field::Key_environment,
                            "image" => Field::Key_image,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomDeploymentStrategyParams;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomDeploymentStrategyParams")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_command: Option<Vec<String>> = None;
                let mut value_environment: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_image: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_command => value_command = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_environment => value_environment = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomDeploymentStrategyParams {
                    command: value_command,
                    environment: value_environment,
                    image: value_image,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomDeploymentStrategyParams",
            &[
                "command",
                "environment",
                "image",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CustomDeploymentStrategyParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomDeploymentStrategyParams",
            self.command.as_ref().map_or(0, |_| 1) +
            self.environment.as_ref().map_or(0, |_| 1) +
            self.image.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.command {
            serde::ser::SerializeStruct::serialize_field(&mut state, "command", value)?;
        }
        if let Some(value) = &self.environment {
            serde::ser::SerializeStruct::serialize_field(&mut state, "environment", value)?;
        }
        if let Some(value) = &self.image {
            serde::ser::SerializeStruct::serialize_field(&mut state, "image", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
