// Generated from definition com.github.openshift.api.authorization.v1.ResourceAccessReview

/// ResourceAccessReview is a means to request a list of which users and groups are authorized to perform the action specified by spec
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceAccessReview {
    /// Content is the actual content of the request for create and update
    pub content: Option<k8s_openapi::apimachinery::pkg::runtime::RawExtension>,

    /// IsNonResourceURL is true if this is a request for a non-resource URL (outside of the resource hieraarchy)
    pub is_non_resource_url: bool,

    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces
    pub namespace: String,

    /// Path is the path of a non resource URL
    pub path: String,

    /// Resource is one of the existing resource types
    pub resource: String,

    /// Group is the API group of the resource Serialized as resourceAPIGroup to avoid confusion with the 'groups' field when inlined
    pub resource_api_group: String,

    /// Version is the API version of the resource Serialized as resourceAPIVersion to avoid confusion with TypeMeta.apiVersion and ObjectMeta.resourceVersion when inlined
    pub resource_api_version: String,

    /// ResourceName is the name of the resource being requested for a "get" or deleted for a "delete"
    pub resource_name: String,

    /// Verb is one of: get, list, watch, create, update, delete
    pub verb: String,
}

// Begin authorization.openshift.io/v1/ResourceAccessReview

// Generated from operation createAuthorizationOpenshiftIoV1ResourceAccessReview

impl ResourceAccessReview {
    /// create a ResourceAccessReview
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
    pub fn create_resource_access_review(
        body: &crate::api::authorization::v1::ResourceAccessReview,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/authorization.openshift.io/v1/resourceaccessreviews?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// End authorization.openshift.io/v1/ResourceAccessReview

impl k8s_openapi::Resource for ResourceAccessReview {
    const API_VERSION: &'static str = "authorization.openshift.io/v1";
    const GROUP: &'static str = "authorization.openshift.io";
    const KIND: &'static str = "ResourceAccessReview";
    const VERSION: &'static str = "v1";
    // fixed `Resource` impl
    const URL_PATH_SEGMENT: &'static str = "resourceaccessreviews";
    type Scope = k8s_openapi::ClusterResourceScope;
}

impl<'de> serde::Deserialize<'de> for ResourceAccessReview {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_content,
            Key_is_non_resource_url,
            Key_namespace,
            Key_path,
            Key_resource,
            Key_resource_api_group,
            Key_resource_api_version,
            Key_resource_name,
            Key_verb,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "content" => Field::Key_content,
                            "isNonResourceURL" => Field::Key_is_non_resource_url,
                            "namespace" => Field::Key_namespace,
                            "path" => Field::Key_path,
                            "resource" => Field::Key_resource,
                            "resourceAPIGroup" => Field::Key_resource_api_group,
                            "resourceAPIVersion" => Field::Key_resource_api_version,
                            "resourceName" => Field::Key_resource_name,
                            "verb" => Field::Key_verb,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourceAccessReview;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value_content: Option<
                    k8s_openapi::apimachinery::pkg::runtime::RawExtension,
                > = None;
                let mut value_is_non_resource_url: Option<bool> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_path: Option<String> = None;
                let mut value_resource: Option<String> = None;
                let mut value_resource_api_group: Option<String> = None;
                let mut value_resource_api_version: Option<String> = None;
                let mut value_resource_name: Option<String> = None;
                let mut value_verb: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String =
                                serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version
                                != <Self::Value as k8s_openapi::Resource>::API_VERSION
                            {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value_api_version),
                                    &<Self::Value as k8s_openapi::Resource>::API_VERSION,
                                ));
                            }
                        }
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as k8s_openapi::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value_kind),
                                    &<Self::Value as k8s_openapi::Resource>::KIND,
                                ));
                            }
                        }
                        Field::Key_content => {
                            value_content = serde::de::MapAccess::next_value(&mut map)?
                        }
                        Field::Key_is_non_resource_url => {
                            value_is_non_resource_url =
                                Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_namespace => {
                            value_namespace = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_path => {
                            value_path = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_resource => {
                            value_resource = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_resource_api_group => {
                            value_resource_api_group =
                                Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_resource_api_version => {
                            value_resource_api_version =
                                Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_resource_name => {
                            value_resource_name = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_verb => {
                            value_verb = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Other => {
                            let _: serde::de::IgnoredAny =
                                serde::de::MapAccess::next_value(&mut map)?;
                        }
                    }
                }

                Ok(ResourceAccessReview {
                    content: value_content,
                    is_non_resource_url: value_is_non_resource_url
                        .ok_or_else(|| serde::de::Error::missing_field("isNonResourceURL"))?,
                    namespace: value_namespace
                        .ok_or_else(|| serde::de::Error::missing_field("namespace"))?,
                    path: value_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    resource: value_resource
                        .ok_or_else(|| serde::de::Error::missing_field("resource"))?,
                    resource_api_group: value_resource_api_group
                        .ok_or_else(|| serde::de::Error::missing_field("resourceAPIGroup"))?,
                    resource_api_version: value_resource_api_version
                        .ok_or_else(|| serde::de::Error::missing_field("resourceAPIVersion"))?,
                    resource_name: value_resource_name
                        .ok_or_else(|| serde::de::Error::missing_field("resourceName"))?,
                    verb: value_verb.ok_or_else(|| serde::de::Error::missing_field("verb"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "content",
                "isNonResourceURL",
                "namespace",
                "path",
                "resource",
                "resourceAPIGroup",
                "resourceAPIVersion",
                "resourceName",
                "verb",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ResourceAccessReview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            10 + self.content.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "apiVersion",
            <Self as k8s_openapi::Resource>::API_VERSION,
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "kind",
            <Self as k8s_openapi::Resource>::KIND,
        )?;
        if let Some(value) = &self.content {
            serde::ser::SerializeStruct::serialize_field(&mut state, "content", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "isNonResourceURL",
            &self.is_non_resource_url,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", &self.namespace)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "resource", &self.resource)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "resourceAPIGroup",
            &self.resource_api_group,
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "resourceAPIVersion",
            &self.resource_api_version,
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "resourceName",
            &self.resource_name,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "verb", &self.verb)?;
        serde::ser::SerializeStruct::end(state)
    }
}
