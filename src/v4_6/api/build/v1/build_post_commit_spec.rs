// Generated from definition com.github.openshift.api.build.v1.BuildPostCommitSpec

/// A BuildPostCommitSpec holds a build post commit hook specification. The hook executes a command in a temporary container running the build output image, immediately after the last layer of the image is committed and before the image is pushed to a registry. The command is executed with the current working directory ($PWD) set to the image's WORKDIR.
///
/// The build will be marked as failed if the hook execution fails. It will fail if the script or command return a non-zero exit code, or if there is any other error related to starting the temporary container.
///
/// There are five different ways to configure the hook. As an example, all forms below are equivalent and will execute `rake test --verbose`.
///
/// 1. Shell script:
///
///   "postCommit": {
///          "script": "rake test --verbose",
///        }
///
///   The above is a convenient form which is equivalent to:
///
///   "postCommit": {
///          "command": \["/bin/sh", "-ic"\],
///          "args":    \["rake test --verbose"\]
///        }
///
/// 2. A command as the image entrypoint:
///
///   "postCommit": {
///          "commit": \["rake", "test", "--verbose"\]
///        }
///
///   Command overrides the image entrypoint in the exec form, as documented in
///     Docker: https://docs.docker.com/engine/reference/builder/#entrypoint.
///
/// 3. Pass arguments to the default entrypoint:
///
///   "postCommit": {
///               "args": \["rake", "test", "--verbose"\]
///           }
///
///   This form is only useful if the image entrypoint can handle arguments.
///
/// 4. Shell script with arguments:
///
///   "postCommit": {
///          "script": "rake test $1",
///          "args":   \["--verbose"\]
///        }
///
///   This form is useful if you need to pass arguments that would otherwise be
///     hard to quote properly in the shell script. In the script, $0 will be
///     "/bin/sh" and $1, $2, etc, are the positional arguments from Args.
///
/// 5. Command with arguments:
///
///   "postCommit": {
///          "command": \["rake", "test"\],
///          "args":    \["--verbose"\]
///        }
///
///   This form is equivalent to appending the arguments to the Command slice.
///
/// It is invalid to provide both Script and Command simultaneously. If none of the fields are specified, the hook is not executed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildPostCommitSpec {
    /// args is a list of arguments that are provided to either Command, Script or the container image's default entrypoint. The arguments are placed immediately after the command to be run.
    pub args: Option<Vec<String>>,

    /// command is the command to run. It may not be specified with Script. This might be needed if the image doesn't have `/bin/sh`, or if you do not want to use a shell. In all other cases, using Script might be more convenient.
    pub command: Option<Vec<String>>,

    /// script is a shell script to be run with `/bin/sh -ic`. It may not be specified with Command. Use Script when a shell script is appropriate to execute the post build hook, for example for running unit tests with `rake test`. If you need control over the image entrypoint, or if the image does not have `/bin/sh`, use Command and/or Args. The `-i` flag is needed to support CentOS and RHEL images that use Software Collections (SCL), in order to have the appropriate collections enabled in the shell. E.g., in the Ruby image, this is necessary to make `ruby`, `bundle` and other binaries available in the PATH.
    pub script: Option<String>,
}

impl<'de> serde::Deserialize<'de> for BuildPostCommitSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_args,
            Key_command,
            Key_script,
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
                            "args" => Field::Key_args,
                            "command" => Field::Key_command,
                            "script" => Field::Key_script,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BuildPostCommitSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildPostCommitSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_args: Option<Vec<String>> = None;
                let mut value_command: Option<Vec<String>> = None;
                let mut value_script: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_args => value_args = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_command => value_command = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_script => value_script = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildPostCommitSpec {
                    args: value_args,
                    command: value_command,
                    script: value_script,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildPostCommitSpec",
            &[
                "args",
                "command",
                "script",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildPostCommitSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildPostCommitSpec",
            self.args.as_ref().map_or(0, |_| 1) +
            self.command.as_ref().map_or(0, |_| 1) +
            self.script.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.args {
            serde::ser::SerializeStruct::serialize_field(&mut state, "args", value)?;
        }
        if let Some(value) = &self.command {
            serde::ser::SerializeStruct::serialize_field(&mut state, "command", value)?;
        }
        if let Some(value) = &self.script {
            serde::ser::SerializeStruct::serialize_field(&mut state, "script", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
