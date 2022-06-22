// Generated from definition com.github.openshift.api.image.v1.ImageImportSpec

/// ImageImportSpec describes a request to import a specific image.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageImportSpec {
    /// From is the source of an image to import; only kind DockerImage is allowed
    pub from: k8s_openapi::api::core::v1::ObjectReference,

    /// ImportPolicy is the policy controlling how the image is imported
    pub import_policy: Option<crate::api::image::v1::TagImportPolicy>,

    /// IncludeManifest determines if the manifest for each image is returned in the response
    pub include_manifest: Option<bool>,

    /// ReferencePolicy defines how other components should consume the image
    pub reference_policy: Option<crate::api::image::v1::TagReferencePolicy>,

    /// To is a tag in the current image stream to assign the imported image to, if name is not specified the default tag from from.name will be used
    pub to: Option<k8s_openapi::api::core::v1::LocalObjectReference>,
}

impl<'de> serde::Deserialize<'de> for ImageImportSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from,
            Key_import_policy,
            Key_include_manifest,
            Key_reference_policy,
            Key_to,
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
                            "to" => Field::Key_to,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageImportSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImageImportSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_import_policy: Option<crate::api::image::v1::TagImportPolicy> = None;
                let mut value_include_manifest: Option<bool> = None;
                let mut value_reference_policy: Option<crate::api::image::v1::TagReferencePolicy> = None;
                let mut value_to: Option<k8s_openapi::api::core::v1::LocalObjectReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_import_policy => value_import_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_include_manifest => value_include_manifest = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reference_policy => value_reference_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_to => value_to = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageImportSpec {
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    import_policy: value_import_policy,
                    include_manifest: value_include_manifest,
                    reference_policy: value_reference_policy,
                    to: value_to,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImageImportSpec",
            &[
                "from",
                "importPolicy",
                "includeManifest",
                "referencePolicy",
                "to",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageImportSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImageImportSpec",
            1 +
            self.import_policy.as_ref().map_or(0, |_| 1) +
            self.include_manifest.as_ref().map_or(0, |_| 1) +
            self.reference_policy.as_ref().map_or(0, |_| 1) +
            self.to.as_ref().map_or(0, |_| 1),
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
        if let Some(value) = &self.to {
            serde::ser::SerializeStruct::serialize_field(&mut state, "to", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
