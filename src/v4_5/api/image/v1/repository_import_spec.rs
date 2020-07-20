// Generated from definition com.github.openshift.api.image.v1.RepositoryImportSpec

/// RepositoryImportSpec describes a request to import images from a container image repository.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepositoryImportSpec {
    /// From is the source for the image repository to import; only kind DockerImage and a name of a container image repository is allowed
    pub from: k8s_openapi::api::core::v1::ObjectReference,

    /// ImportPolicy is the policy controlling how the image is imported
    pub import_policy: Option<crate::api::image::v1::TagImportPolicy>,

    /// IncludeManifest determines if the manifest for each image is returned in the response
    pub include_manifest: Option<bool>,

    /// ReferencePolicy defines how other components should consume the image
    pub reference_policy: Option<crate::api::image::v1::TagReferencePolicy>,
}

impl<'de> serde::Deserialize<'de> for RepositoryImportSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from,
            Key_import_policy,
            Key_include_manifest,
            Key_reference_policy,
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
                            "from" => Field::Key_from,
                            "importPolicy" => Field::Key_import_policy,
                            "includeManifest" => Field::Key_include_manifest,
                            "referencePolicy" => Field::Key_reference_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RepositoryImportSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RepositoryImportSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_import_policy: Option<crate::api::image::v1::TagImportPolicy> = None;
                let mut value_include_manifest: Option<bool> = None;
                let mut value_reference_policy: Option<crate::api::image::v1::TagReferencePolicy> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_import_policy => value_import_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_include_manifest => value_include_manifest = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reference_policy => value_reference_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RepositoryImportSpec {
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    import_policy: value_import_policy,
                    include_manifest: value_include_manifest,
                    reference_policy: value_reference_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "RepositoryImportSpec",
            &[
                "from",
                "importPolicy",
                "includeManifest",
                "referencePolicy",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RepositoryImportSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RepositoryImportSpec",
            1 +
            self.import_policy.as_ref().map_or(0, |_| 1) +
            self.include_manifest.as_ref().map_or(0, |_| 1) +
            self.reference_policy.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        if let Some(value) = &self.import_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "importPolicy", value)?;
        }
        if let Some(value) = &self.include_manifest {
            serde::ser::SerializeStruct::serialize_field(&mut state, "includeManifest", value)?;
        }
        if let Some(value) = &self.reference_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "referencePolicy", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
