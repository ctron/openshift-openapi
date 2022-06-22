// Generated from definition com.github.openshift.api.image.v1.TagReference

/// TagReference specifies optional annotations for images using this tag and an optional reference to an ImageStreamTag, ImageStreamImage, or DockerImage this tag should track.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagReference {
    /// Optional; if specified, annotations that are applied to images retrieved via ImageStreamTags.
    pub annotations: Option<std::collections::BTreeMap<String, String>>,

    /// Optional; if specified, a reference to another image that this tag should point to. Valid values are ImageStreamTag, ImageStreamImage, and DockerImage.  ImageStreamTag references can only reference a tag within this same ImageStream.
    pub from: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// Generation is a counter that tracks mutations to the spec tag (user intent). When a tag reference is changed the generation is set to match the current stream generation (which is incremented every time spec is changed). Other processes in the system like the image importer observe that the generation of spec tag is newer than the generation recorded in the status and use that as a trigger to import the newest remote tag. To trigger a new import, clients may set this value to zero which will reset the generation to the latest stream generation. Legacy clients will send this value as nil which will be merged with the current tag generation.
    pub generation: Option<i64>,

    /// ImportPolicy is information that controls how images may be imported by the server.
    pub import_policy: Option<crate::api::image::v1::TagImportPolicy>,

    /// Name of the tag
    pub name: String,

    /// Reference states if the tag will be imported. Default value is false, which means the tag will be imported.
    pub reference: Option<bool>,

    /// ReferencePolicy defines how other components should consume the image.
    pub reference_policy: Option<crate::api::image::v1::TagReferencePolicy>,
}

impl<'de> serde::Deserialize<'de> for TagReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_annotations,
            Key_from,
            Key_generation,
            Key_import_policy,
            Key_name,
            Key_reference,
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
                            "annotations" => Field::Key_annotations,
                            "from" => Field::Key_from,
                            "generation" => Field::Key_generation,
                            "importPolicy" => Field::Key_import_policy,
                            "name" => Field::Key_name,
                            "reference" => Field::Key_reference,
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
            type Value = TagReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TagReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_annotations: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_generation: Option<i64> = None;
                let mut value_import_policy: Option<crate::api::image::v1::TagImportPolicy> = None;
                let mut value_name: Option<String> = None;
                let mut value_reference: Option<bool> = None;
                let mut value_reference_policy: Option<crate::api::image::v1::TagReferencePolicy> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_annotations => value_annotations = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generation => value_generation = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_import_policy => value_import_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reference => value_reference = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reference_policy => value_reference_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TagReference {
                    annotations: value_annotations,
                    from: value_from,
                    generation: value_generation,
                    import_policy: value_import_policy,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    reference: value_reference,
                    reference_policy: value_reference_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "TagReference",
            &[
                "annotations",
                "from",
                "generation",
                "importPolicy",
                "name",
                "reference",
                "referencePolicy",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TagReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TagReference",
            1 +
            self.annotations.as_ref().map_or(0, |_| 1) +
            self.from.as_ref().map_or(0, |_| 1) +
            self.generation.as_ref().map_or(0, |_| 1) +
            self.import_policy.as_ref().map_or(0, |_| 1) +
            self.reference.as_ref().map_or(0, |_| 1) +
            self.reference_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.annotations {
            serde::ser::SerializeStruct::serialize_field(&mut state, "annotations", value)?;
        }
        if let Some(value) = &self.from {
            serde::ser::SerializeStruct::serialize_field(&mut state, "from", value)?;
        }
        if let Some(value) = &self.generation {
            serde::ser::SerializeStruct::serialize_field(&mut state, "generation", value)?;
        }
        if let Some(value) = &self.import_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "importPolicy", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.reference {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reference", value)?;
        }
        if let Some(value) = &self.reference_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "referencePolicy", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
