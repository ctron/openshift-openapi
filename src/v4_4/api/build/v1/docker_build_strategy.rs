// Generated from definition com.github.openshift.api.build.v1.DockerBuildStrategy

/// DockerBuildStrategy defines input parameters specific to container image build.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DockerBuildStrategy {
    /// buildArgs contains build arguments that will be resolved in the Dockerfile.  See https://docs.docker.com/engine/reference/builder/#/arg for more details.
    pub build_args: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// dockerfilePath is the path of the Dockerfile that will be used to build the container image, relative to the root of the context (contextDir).
    pub dockerfile_path: Option<String>,

    /// env contains additional environment variables you want to pass into a builder container.
    pub env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>>,

    /// forcePull describes if the builder should pull the images from registry prior to building.
    pub force_pull: Option<bool>,

    /// from is reference to an DockerImage, ImageStreamTag, or ImageStreamImage from which the container image should be pulled the resulting image will be used in the FROM line of the Dockerfile for this build.
    pub from: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// imageOptimizationPolicy describes what optimizations the system can use when building images to reduce the final size or time spent building the image. The default policy is 'None' which means the final build image will be equivalent to an image created by the container image build API. The experimental policy 'SkipLayers' will avoid commiting new layers in between each image step, and will fail if the Dockerfile cannot provide compatibility with the 'None' policy. An additional experimental policy 'SkipLayersAndWarn' is the same as 'SkipLayers' but simply warns if compatibility cannot be preserved.
    pub image_optimization_policy: Option<String>,

    /// noCache if set to true indicates that the container image build must be executed with the --no-cache=true flag
    pub no_cache: Option<bool>,

    /// pullSecret is the name of a Secret that would be used for setting up the authentication for pulling the container images from the private Docker registries
    pub pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference>,
}

impl<'de> serde::Deserialize<'de> for DockerBuildStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_build_args,
            Key_dockerfile_path,
            Key_env,
            Key_force_pull,
            Key_from,
            Key_image_optimization_policy,
            Key_no_cache,
            Key_pull_secret,
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
                            "buildArgs" => Field::Key_build_args,
                            "dockerfilePath" => Field::Key_dockerfile_path,
                            "env" => Field::Key_env,
                            "forcePull" => Field::Key_force_pull,
                            "from" => Field::Key_from,
                            "imageOptimizationPolicy" => Field::Key_image_optimization_policy,
                            "noCache" => Field::Key_no_cache,
                            "pullSecret" => Field::Key_pull_secret,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DockerBuildStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DockerBuildStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_build_args: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_dockerfile_path: Option<String> = None;
                let mut value_env: Option<Vec<k8s_openapi::api::core::v1::EnvVar>> = None;
                let mut value_force_pull: Option<bool> = None;
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_image_optimization_policy: Option<String> = None;
                let mut value_no_cache: Option<bool> = None;
                let mut value_pull_secret: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_build_args => value_build_args = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dockerfile_path => value_dockerfile_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env => value_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_force_pull => value_force_pull = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_optimization_policy => value_image_optimization_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_no_cache => value_no_cache = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pull_secret => value_pull_secret = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DockerBuildStrategy {
                    build_args: value_build_args,
                    dockerfile_path: value_dockerfile_path,
                    env: value_env,
                    force_pull: value_force_pull,
                    from: value_from,
                    image_optimization_policy: value_image_optimization_policy,
                    no_cache: value_no_cache,
                    pull_secret: value_pull_secret,
                })
            }
        }

        deserializer.deserialize_struct(
            "DockerBuildStrategy",
            &[
                "buildArgs",
                "dockerfilePath",
                "env",
                "forcePull",
                "from",
                "imageOptimizationPolicy",
                "noCache",
                "pullSecret",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DockerBuildStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DockerBuildStrategy",
            self.build_args.as_ref().map_or(0, |_| 1) +
            self.dockerfile_path.as_ref().map_or(0, |_| 1) +
            self.env.as_ref().map_or(0, |_| 1) +
            self.force_pull.as_ref().map_or(0, |_| 1) +
            self.from.as_ref().map_or(0, |_| 1) +
            self.image_optimization_policy.as_ref().map_or(0, |_| 1) +
            self.no_cache.as_ref().map_or(0, |_| 1) +
            self.pull_secret.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.build_args {
            serde::ser::SerializeStruct::serialize_field(&mut state, "buildArgs", value)?;
        }
        if let Some(value) = &self.dockerfile_path {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerfilePath", value)?;
        }
        if let Some(value) = &self.env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.force_pull {
            serde::ser::SerializeStruct::serialize_field(&mut state, "forcePull", value)?;
        }
        if let Some(value) = &self.from {
            serde::ser::SerializeStruct::serialize_field(&mut state, "from", value)?;
        }
        if let Some(value) = &self.image_optimization_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageOptimizationPolicy", value)?;
        }
        if let Some(value) = &self.no_cache {
            serde::ser::SerializeStruct::serialize_field(&mut state, "noCache", value)?;
        }
        if let Some(value) = &self.pull_secret {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pullSecret", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
