// Generated from definition com.github.openshift.api.apps.v1.ExecNewPodHook

/// ExecNewPodHook is a hook implementation which runs a command in a new pod based on the specified container which is assumed to be part of the deployment template.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExecNewPodHook {
    /// Command is the action command and its arguments.
    pub command: Vec<String>,

    /// ContainerName is the name of a container in the deployment pod template whose container image will be used for the hook pod's container.
    pub container_name: String,

    /// Env is a set of environment variables to supply to the hook pod's container.
    pub env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// Volumes is a list of named volumes from the pod template which should be copied to the hook pod. Volumes names not found in pod spec are ignored. An empty list means no volumes will be copied.
    pub volumes: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for ExecNewPodHook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_command,
            Key_container_name,
            Key_env,
            Key_volumes,
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
                            "containerName" => Field::Key_container_name,
                            "env" => Field::Key_env,
                            "volumes" => Field::Key_volumes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExecNewPodHook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ExecNewPodHook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_command: Option<Vec<String>> = None;
                let mut value_container_name: Option<String> = None;
                let mut value_env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_volumes: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_command => value_command = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_container_name => value_container_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_env => value_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExecNewPodHook {
                    command: value_command.ok_or_else(|| serde::de::Error::missing_field("command"))?,
                    container_name: value_container_name.ok_or_else(|| serde::de::Error::missing_field("containerName"))?,
                    env: value_env,
                    volumes: value_volumes,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExecNewPodHook",
            &[
                "command",
                "containerName",
                "env",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ExecNewPodHook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExecNewPodHook",
            2 +
            self.env.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "command", &self.command)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "containerName", &self.container_name)?;
        if let Some(value) = &self.env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.volumes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
