// Generated from definition com.github.openshift.api.build.v1.BuildSource

/// BuildSource is the SCM used for the build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuildSource {
    /// binary builds accept a binary as their input. The binary is generally assumed to be a tar, gzipped tar, or zip file depending on the strategy. For container image builds, this is the build context and an optional Dockerfile may be specified to override any Dockerfile in the build context. For Source builds, this is assumed to be an archive as described above. For Source and container image builds, if binary.asFile is set the build will receive a directory with a single file. contextDir may be used when an archive is provided. Custom builds will receive this binary as input on STDIN.
    pub binary: Option<crate::api::build::v1::BinaryBuildSource>,

    /// configMaps represents a list of configMaps and their destinations that will be used for the build.
    pub config_maps: Option<Vec<crate::api::build::v1::ConfigMapBuildSource>>,

    /// contextDir specifies the sub-directory where the source code for the application exists. This allows to have buildable sources in directory other than root of repository.
    pub context_dir: Option<String>,

    /// dockerfile is the raw contents of a Dockerfile which should be built. When this option is specified, the FROM may be modified based on your strategy base image and additional ENV stanzas from your strategy environment will be added after the FROM, but before the rest of your Dockerfile stanzas. The Dockerfile source type may be used with other options like git - in those cases the Git repo will have any innate Dockerfile replaced in the context dir.
    pub dockerfile: Option<String>,

    /// git contains optional information about git build source
    pub git: Option<crate::api::build::v1::GitBuildSource>,

    /// images describes a set of images to be used to provide source for the build
    pub images: Option<Vec<crate::api::build::v1::ImageSource>>,

    /// secrets represents a list of secrets and their destinations that will be used only for the build.
    pub secrets: Option<Vec<crate::api::build::v1::SecretBuildSource>>,

    /// sourceSecret is the name of a Secret that would be used for setting up the authentication for cloning private repository. The secret contains valid credentials for remote repository, where the data's key represent the authentication method to be used and value is the base64 encoded credentials. Supported auth methods are: ssh-privatekey.
    pub source_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference>,

    /// type of build input to accept
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for BuildSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_binary,
            Key_config_maps,
            Key_context_dir,
            Key_dockerfile,
            Key_git,
            Key_images,
            Key_secrets,
            Key_source_secret,
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
                            "binary" => Field::Key_binary,
                            "configMaps" => Field::Key_config_maps,
                            "contextDir" => Field::Key_context_dir,
                            "dockerfile" => Field::Key_dockerfile,
                            "git" => Field::Key_git,
                            "images" => Field::Key_images,
                            "secrets" => Field::Key_secrets,
                            "sourceSecret" => Field::Key_source_secret,
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
            type Value = BuildSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BuildSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_binary: Option<crate::api::build::v1::BinaryBuildSource> = None;
                let mut value_config_maps: Option<Vec<crate::api::build::v1::ConfigMapBuildSource>> = None;
                let mut value_context_dir: Option<String> = None;
                let mut value_dockerfile: Option<String> = None;
                let mut value_git: Option<crate::api::build::v1::GitBuildSource> = None;
                let mut value_images: Option<Vec<crate::api::build::v1::ImageSource>> = None;
                let mut value_secrets: Option<Vec<crate::api::build::v1::SecretBuildSource>> = None;
                let mut value_source_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_binary => value_binary = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config_maps => value_config_maps = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_context_dir => value_context_dir = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dockerfile => value_dockerfile = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_git => value_git = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_images => value_images = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secrets => value_secrets = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source_secret => value_source_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BuildSource {
                    binary: value_binary,
                    config_maps: value_config_maps,
                    context_dir: value_context_dir,
                    dockerfile: value_dockerfile,
                    git: value_git,
                    images: value_images,
                    secrets: value_secrets,
                    source_secret: value_source_secret,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BuildSource",
            &[
                "binary",
                "configMaps",
                "contextDir",
                "dockerfile",
                "git",
                "images",
                "secrets",
                "sourceSecret",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BuildSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BuildSource",
            1 +
            self.binary.as_ref().map_or(0, |_| 1) +
            self.config_maps.as_ref().map_or(0, |_| 1) +
            self.context_dir.as_ref().map_or(0, |_| 1) +
            self.dockerfile.as_ref().map_or(0, |_| 1) +
            self.git.as_ref().map_or(0, |_| 1) +
            self.images.as_ref().map_or(0, |_| 1) +
            self.secrets.as_ref().map_or(0, |_| 1) +
            self.source_secret.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.binary {
            serde::ser::SerializeStruct::serialize_field(&mut state, "binary", value)?;
        }
        if let Some(value) = &self.config_maps {
            serde::ser::SerializeStruct::serialize_field(&mut state, "configMaps", value)?;
        }
        if let Some(value) = &self.context_dir {
            serde::ser::SerializeStruct::serialize_field(&mut state, "contextDir", value)?;
        }
        if let Some(value) = &self.dockerfile {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerfile", value)?;
        }
        if let Some(value) = &self.git {
            serde::ser::SerializeStruct::serialize_field(&mut state, "git", value)?;
        }
        if let Some(value) = &self.images {
            serde::ser::SerializeStruct::serialize_field(&mut state, "images", value)?;
        }
        if let Some(value) = &self.secrets {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secrets", value)?;
        }
        if let Some(value) = &self.source_secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "sourceSecret", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
