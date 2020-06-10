// Generated from definition com.github.openshift.api.build.v1.GitBuildSource

/// GitBuildSource defines the parameters of a Git SCM
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GitBuildSource {
    /// httpProxy is a proxy used to reach the git repository over http
    pub http_proxy: Option<String>,

    /// httpsProxy is a proxy used to reach the git repository over https
    pub https_proxy: Option<String>,

    /// noProxy is the list of domains for which the proxy should not be used
    pub no_proxy: Option<String>,

    /// ref is the branch/tag/ref to build.
    pub r#ref: Option<String>,

    /// uri points to the source that will be built. The structure of the source will depend on the type of build to run
    pub uri: String,
}

impl<'de> serde::Deserialize<'de> for GitBuildSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_http_proxy,
            Key_https_proxy,
            Key_no_proxy,
            Key_ref,
            Key_uri,
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
                            "httpProxy" => Field::Key_http_proxy,
                            "httpsProxy" => Field::Key_https_proxy,
                            "noProxy" => Field::Key_no_proxy,
                            "ref" => Field::Key_ref,
                            "uri" => Field::Key_uri,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GitBuildSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GitBuildSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_http_proxy: Option<String> = None;
                let mut value_https_proxy: Option<String> = None;
                let mut value_no_proxy: Option<String> = None;
                let mut value_ref: Option<String> = None;
                let mut value_uri: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_http_proxy => value_http_proxy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_https_proxy => value_https_proxy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_no_proxy => value_no_proxy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ref => value_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uri => value_uri = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GitBuildSource {
                    http_proxy: value_http_proxy,
                    https_proxy: value_https_proxy,
                    no_proxy: value_no_proxy,
                    r#ref: value_ref,
                    uri: value_uri.ok_or_else(|| serde::de::Error::missing_field("uri"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "GitBuildSource",
            &[
                "httpProxy",
                "httpsProxy",
                "noProxy",
                "ref",
                "uri",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for GitBuildSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GitBuildSource",
            1 +
            self.http_proxy.as_ref().map_or(0, |_| 1) +
            self.https_proxy.as_ref().map_or(0, |_| 1) +
            self.no_proxy.as_ref().map_or(0, |_| 1) +
            self.r#ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.http_proxy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "httpProxy", value)?;
        }
        if let Some(value) = &self.https_proxy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "httpsProxy", value)?;
        }
        if let Some(value) = &self.no_proxy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "noProxy", value)?;
        }
        if let Some(value) = &self.r#ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ref", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "uri", &self.uri)?;
        serde::ser::SerializeStruct::end(state)
    }
}
