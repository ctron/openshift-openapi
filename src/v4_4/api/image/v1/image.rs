// Generated from definition com.github.openshift.api.image.v1.Image

/// Image is an immutable representation of a container image and metadata at a point in time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Image {
    /// DockerImageConfig is a JSON blob that the runtime uses to set up the container. This is a part of manifest schema v2.
    pub docker_image_config: Option<String>,

    /// DockerImageLayers represents the layers in the image. May not be set if the image does not define that data.
    pub docker_image_layers: Vec<crate::api::image::v1::ImageLayer>,

    /// DockerImageManifest is the raw JSON of the manifest
    pub docker_image_manifest: Option<String>,

    /// DockerImageManifestMediaType specifies the mediaType of manifest. This is a part of manifest schema v2.
    pub docker_image_manifest_media_type: Option<String>,

    /// DockerImageMetadata contains metadata about this image
    pub docker_image_metadata: Option<k8s_openapi::apimachinery::pkg::runtime::RawExtension>,

    /// DockerImageMetadataVersion conveys the version of the object, which if empty defaults to "1.0"
    pub docker_image_metadata_version: Option<String>,

    /// DockerImageReference is the string that can be used to pull this image.
    pub docker_image_reference: Option<String>,

    /// DockerImageSignatures provides the signatures as opaque blobs. This is a part of manifest schema v1.
    pub docker_image_signatures: Option<Vec<k8s_openapi::ByteString>>,

    /// Standard object's metadata.
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Signatures holds all signatures of the image.
    pub signatures: Option<Vec<crate::api::image::v1::ImageSignature>>,
}

// Begin image.openshift.io/v1/Image

// Generated from operation createImageOpenshiftIoV1Image

impl Image {
    /// create an Image
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
    pub fn create_image(
        body: &crate::api::image::v1::Image,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/image.openshift.io/v1/images?".to_owned();
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

// Generated from operation deleteImageOpenshiftIoV1CollectionImage

impl Image {
    /// delete collection of Image
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>>` constructor, or [`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_image(
        delete_optional: k8s_openapi::DeleteOptional<'_>,
        list_optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>), k8s_openapi::RequestError> {
        let __url = "/apis/image.openshift.io/v1/images?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&delete_optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteImageOpenshiftIoV1Image

impl Image {
    /// delete an Image
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Image
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_image(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/image.openshift.io/v1/images/{name}",
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

// Generated from operation listImageOpenshiftIoV1Image

impl Image {
    /// list or watch objects of kind Image
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ListResponse`]`<Self>>` constructor, or [`k8s_openapi::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_image(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/image.openshift.io/v1/images?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchImageOpenshiftIoV1Image

impl Image {
    /// partially update the specified Image
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Image
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_image(
        name: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/image.openshift.io/v1/images/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation readImageOpenshiftIoV1Image

impl Image {
    /// read the specified Image
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadImageResponse`]`>` constructor, or [`ReadImageResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Image
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_image(
        name: &str,
        optional: ReadImageOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadImageResponse>), k8s_openapi::RequestError> {
        let ReadImageOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/image.openshift.io/v1/images/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Image::read_image`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadImageOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadImageResponse as Response>::try_from_parts` to parse the HTTP response body of [`Image::read_image`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadImageResponse {
    Ok(crate::api::image::v1::Image),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadImageResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadImageResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadImageResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceImageOpenshiftIoV1Image

impl Image {
    /// replace the specified Image
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Image
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_image(
        name: &str,
        body: &crate::api::image::v1::Image,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/image.openshift.io/v1/images/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchImageOpenshiftIoV1Image

impl Image {
    /// list or watch objects of kind Image
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::WatchResponse`]`<Self>>` constructor, or [`k8s_openapi::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_image(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/image.openshift.io/v1/images?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// End image.openshift.io/v1/Image

impl k8s_openapi::Resource for Image {
    const API_VERSION: &'static str = "image.openshift.io/v1";
    const GROUP: &'static str = "image.openshift.io";
    const KIND: &'static str = "Image";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::ListableResource for Image {
    const LIST_KIND: &'static str = concat!("Image", "List");
}

impl k8s_openapi::Metadata for Image {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_docker_image_config,
            Key_docker_image_layers,
            Key_docker_image_manifest,
            Key_docker_image_manifest_media_type,
            Key_docker_image_metadata,
            Key_docker_image_metadata_version,
            Key_docker_image_reference,
            Key_docker_image_signatures,
            Key_metadata,
            Key_signatures,
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
                            "dockerImageConfig" => Field::Key_docker_image_config,
                            "dockerImageLayers" => Field::Key_docker_image_layers,
                            "dockerImageManifest" => Field::Key_docker_image_manifest,
                            "dockerImageManifestMediaType" => Field::Key_docker_image_manifest_media_type,
                            "dockerImageMetadata" => Field::Key_docker_image_metadata,
                            "dockerImageMetadataVersion" => Field::Key_docker_image_metadata_version,
                            "dockerImageReference" => Field::Key_docker_image_reference,
                            "dockerImageSignatures" => Field::Key_docker_image_signatures,
                            "metadata" => Field::Key_metadata,
                            "signatures" => Field::Key_signatures,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Image;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_docker_image_config: Option<String> = None;
                let mut value_docker_image_layers: Option<Vec<crate::api::image::v1::ImageLayer>> = None;
                let mut value_docker_image_manifest: Option<String> = None;
                let mut value_docker_image_manifest_media_type: Option<String> = None;
                let mut value_docker_image_metadata: Option<k8s_openapi::apimachinery::pkg::runtime::RawExtension> = None;
                let mut value_docker_image_metadata_version: Option<String> = None;
                let mut value_docker_image_reference: Option<String> = None;
                let mut value_docker_image_signatures: Option<Vec<k8s_openapi::ByteString>> = None;
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_signatures: Option<Vec<crate::api::image::v1::ImageSignature>> = None;

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
                        Field::Key_docker_image_config => value_docker_image_config = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_image_layers => value_docker_image_layers = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_docker_image_manifest => value_docker_image_manifest = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_image_manifest_media_type => value_docker_image_manifest_media_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_image_metadata => value_docker_image_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_image_metadata_version => value_docker_image_metadata_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_image_reference => value_docker_image_reference = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_docker_image_signatures => value_docker_image_signatures = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_signatures => value_signatures = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Image {
                    docker_image_config: value_docker_image_config,
                    docker_image_layers: value_docker_image_layers.ok_or_else(|| serde::de::Error::missing_field("dockerImageLayers"))?,
                    docker_image_manifest: value_docker_image_manifest,
                    docker_image_manifest_media_type: value_docker_image_manifest_media_type,
                    docker_image_metadata: value_docker_image_metadata,
                    docker_image_metadata_version: value_docker_image_metadata_version,
                    docker_image_reference: value_docker_image_reference,
                    docker_image_signatures: value_docker_image_signatures,
                    metadata: value_metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    signatures: value_signatures,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "dockerImageConfig",
                "dockerImageLayers",
                "dockerImageManifest",
                "dockerImageManifestMediaType",
                "dockerImageMetadata",
                "dockerImageMetadataVersion",
                "dockerImageReference",
                "dockerImageSignatures",
                "metadata",
                "signatures",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            4 +
            self.docker_image_config.as_ref().map_or(0, |_| 1) +
            self.docker_image_manifest.as_ref().map_or(0, |_| 1) +
            self.docker_image_manifest_media_type.as_ref().map_or(0, |_| 1) +
            self.docker_image_metadata.as_ref().map_or(0, |_| 1) +
            self.docker_image_metadata_version.as_ref().map_or(0, |_| 1) +
            self.docker_image_reference.as_ref().map_or(0, |_| 1) +
            self.docker_image_signatures.as_ref().map_or(0, |_| 1) +
            self.signatures.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.docker_image_config {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageConfig", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageLayers", &self.docker_image_layers)?;
        if let Some(value) = &self.docker_image_manifest {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageManifest", value)?;
        }
        if let Some(value) = &self.docker_image_manifest_media_type {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageManifestMediaType", value)?;
        }
        if let Some(value) = &self.docker_image_metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageMetadata", value)?;
        }
        if let Some(value) = &self.docker_image_metadata_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageMetadataVersion", value)?;
        }
        if let Some(value) = &self.docker_image_reference {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageReference", value)?;
        }
        if let Some(value) = &self.docker_image_signatures {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dockerImageSignatures", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.signatures {
            serde::ser::SerializeStruct::serialize_field(&mut state, "signatures", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
