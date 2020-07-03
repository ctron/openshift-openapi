// Generated from definition com.github.openshift.api.image.v1.ImageSignature

/// ImageSignature holds a signature of an image. It allows to verify image identity and possibly other claims as long as the signature is trusted. Based on this information it is possible to restrict runnable images to those matching cluster-wide policy. Mandatory fields should be parsed by clients doing image verification. The others are parsed from signature's content by the server. They serve just an informative purpose.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageSignature {
    /// Conditions represent the latest available observations of a signature's current state.
    pub conditions: Option<Vec<crate::api::image::v1::SignatureCondition>>,

    /// Required: An opaque binary string which is an image's signature.
    pub content: k8s_openapi::ByteString,

    /// If specified, it is the time of signature's creation.
    pub created: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time>,

    /// A human readable string representing image's identity. It could be a product name and version, or an image pull spec (e.g. "registry.access.redhat.com/rhel7/rhel:7.2").
    pub image_identity: Option<String>,

    /// If specified, it holds information about an issuer of signing certificate or key (a person or entity who signed the signing certificate or key).
    pub issued_by: Option<crate::api::image::v1::SignatureIssuer>,

    /// If specified, it holds information about a subject of signing certificate or key (a person or entity who signed the image).
    pub issued_to: Option<crate::api::image::v1::SignatureSubject>,

    /// Standard object's metadata.
    pub metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Contains claims from the signature.
    pub signed_claims: Option<std::collections::BTreeMap<String, String>>,

    /// Required: Describes a type of stored blob.
    pub type_: String,
}

// Begin image.openshift.io/v1/ImageSignature

// Generated from operation createImageOpenshiftIoV1ImageSignature

impl ImageSignature {
    /// create an ImageSignature
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::CreateResponse`]`<Self>>` constructor, or [`k8s_openapi::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_image_signature(
        body: &crate::api::image::v1::ImageSignature,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/image.openshift.io/v1/imagesignatures?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteImageOpenshiftIoV1ImageSignature

impl ImageSignature {
    /// delete an ImageSignature
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ImageSignature
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_image_signature(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/image.openshift.io/v1/imagesignatures/{name}",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// End image.openshift.io/v1/ImageSignature

impl k8s_openapi::Resource for ImageSignature {
    const API_VERSION: &'static str = "image.openshift.io/v1";
    const GROUP: &'static str = "image.openshift.io";
    const KIND: &'static str = "ImageSignature";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::Metadata for ImageSignature {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as k8s_openapi::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ImageSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_conditions,
            Key_content,
            Key_created,
            Key_image_identity,
            Key_issued_by,
            Key_issued_to,
            Key_metadata,
            Key_signed_claims,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "conditions" => Field::Key_conditions,
                            "content" => Field::Key_content,
                            "created" => Field::Key_created,
                            "imageIdentity" => Field::Key_image_identity,
                            "issuedBy" => Field::Key_issued_by,
                            "issuedTo" => Field::Key_issued_to,
                            "metadata" => Field::Key_metadata,
                            "signedClaims" => Field::Key_signed_claims,
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
            type Value = ImageSignature;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::image::v1::SignatureCondition>> = None;
                let mut value_content: Option<k8s_openapi::ByteString> = None;
                let mut value_created: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_image_identity: Option<String> = None;
                let mut value_issued_by: Option<crate::api::image::v1::SignatureIssuer> = None;
                let mut value_issued_to: Option<crate::api::image::v1::SignatureSubject> = None;
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_signed_claims: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as k8s_openapi::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as k8s_openapi::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as k8s_openapi::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as k8s_openapi::Resource>::KIND));
                            }
                        },
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_content => value_content = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_created => value_created = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_identity => value_image_identity = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_issued_by => value_issued_by = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_issued_to => value_issued_to = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signed_claims => value_signed_claims = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageSignature {
                    conditions: value_conditions,
                    content: value_content.ok_or_else(|| serde::de::Error::missing_field("content"))?,
                    created: value_created,
                    image_identity: value_image_identity,
                    issued_by: value_issued_by,
                    issued_to: value_issued_to,
                    metadata: value_metadata,
                    signed_claims: value_signed_claims,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "conditions",
                "content",
                "created",
                "imageIdentity",
                "issuedBy",
                "issuedTo",
                "metadata",
                "signedClaims",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            4 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.created.as_ref().map_or(0, |_| 1) +
            self.image_identity.as_ref().map_or(0, |_| 1) +
            self.issued_by.as_ref().map_or(0, |_| 1) +
            self.issued_to.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.signed_claims.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "content", &self.content)?;
        if let Some(value) = &self.created {
            serde::ser::SerializeStruct::serialize_field(&mut state, "created", value)?;
        }
        if let Some(value) = &self.image_identity {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imageIdentity", value)?;
        }
        if let Some(value) = &self.issued_by {
            serde::ser::SerializeStruct::serialize_field(&mut state, "issuedBy", value)?;
        }
        if let Some(value) = &self.issued_to {
            serde::ser::SerializeStruct::serialize_field(&mut state, "issuedTo", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.signed_claims {
            serde::ser::SerializeStruct::serialize_field(&mut state, "signedClaims", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
