// Generated from definition com.github.openshift.api.image.v1.ImageStreamLayers

/// ImageStreamLayers describes information about the layers referenced by images in this image stream.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageStreamLayers {
    /// blobs is a map of blob name to metadata about the blob.
    pub blobs: std::collections::BTreeMap<String, crate::api::image::v1::ImageLayerData>,

    /// images is a map between an image name and the names of the blobs and config that comprise the image.
    pub images: std::collections::BTreeMap<String, crate::api::image::v1::ImageBlobReferences>,

    /// Standard object's metadata.
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}

// Begin image.openshift.io/v1/ImageStreamLayers

// Generated from operation readImageOpenshiftIoV1NamespacedImageStreamLayers

impl ImageStreamLayers {
    /// read layers of the specified ImageStream
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadNamespacedImageStreamLayersResponse`]`>` constructor, or [`ReadNamespacedImageStreamLayersResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ImageStreamLayers
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_image_stream_layers(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedImageStreamLayersOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadNamespacedImageStreamLayersResponse>), k8s_openapi::RequestError> {
        let ReadNamespacedImageStreamLayersOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/image.openshift.io/v1/namespaces/{namespace}/imagestreams/{name}/layers?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = k8s_openapi::percent_encoding::percent_encode(namespace.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`ImageStreamLayers::read_namespaced_image_stream_layers`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedImageStreamLayersOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedImageStreamLayersResponse as Response>::try_from_parts` to parse the HTTP response body of [`ImageStreamLayers::read_namespaced_image_stream_layers`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedImageStreamLayersResponse {
    Ok(crate::api::image::v1::ImageStreamLayers),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadNamespacedImageStreamLayersResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedImageStreamLayersResponse::Ok(result), buf.len()))
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
                Ok((ReadNamespacedImageStreamLayersResponse::Other(result), read))
            },
        }
    }
}

// End image.openshift.io/v1/ImageStreamLayers

impl k8s_openapi::Resource for ImageStreamLayers {
    const API_VERSION: &'static str = "image.openshift.io/v1";
    const GROUP: &'static str = "image.openshift.io";
    const KIND: &'static str = "ImageStreamLayers";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::Metadata for ImageStreamLayers {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for ImageStreamLayers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_blobs,
            Key_images,
            Key_metadata,
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
                            "blobs" => Field::Key_blobs,
                            "images" => Field::Key_images,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageStreamLayers;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_blobs: Option<std::collections::BTreeMap<String, crate::api::image::v1::ImageLayerData>> = None;
                let mut value_images: Option<std::collections::BTreeMap<String, crate::api::image::v1::ImageBlobReferences>> = None;
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;

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
                        Field::Key_blobs => value_blobs = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_images => value_images = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metadata => value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImageStreamLayers {
                    blobs: value_blobs.ok_or_else(|| serde::de::Error::missing_field("blobs"))?,
                    images: value_images.ok_or_else(|| serde::de::Error::missing_field("images"))?,
                    metadata: value_metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "blobs",
                "images",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ImageStreamLayers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            5,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "blobs", &self.blobs)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "images", &self.images)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        serde::ser::SerializeStruct::end(state)
    }
}
