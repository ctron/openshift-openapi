// Generated from definition com.github.openshift.api.security.v1.SecurityContextConstraints

/// SecurityContextConstraints governs the ability to make requests that affect the SecurityContext that will be applied to a container. For historical reasons SCC was exposed under the core Kubernetes API group. That exposure is deprecated and will be removed in a future release - users should instead use the security.openshift.io group to manage SecurityContextConstraints.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecurityContextConstraints {
    /// AllowHostDirVolumePlugin determines if the policy allow containers to use the HostDir volume plugin
    pub allow_host_dir_volume_plugin: bool,

    /// AllowHostIPC determines if the policy allows host ipc in the containers.
    pub allow_host_ipc: bool,

    /// AllowHostNetwork determines if the policy allows the use of HostNetwork in the pod spec.
    pub allow_host_network: bool,

    /// AllowHostPID determines if the policy allows host pid in the containers.
    pub allow_host_pid: bool,

    /// AllowHostPorts determines if the policy allows host ports in the containers.
    pub allow_host_ports: bool,

    /// AllowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true.
    pub allow_privilege_escalation: Option<bool>,

    /// AllowPrivilegedContainer determines if a container can request to be run as privileged.
    pub allow_privileged_container: bool,

    /// AllowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field maybe added at the pod author's discretion. You must not list a capability in both AllowedCapabilities and RequiredDropCapabilities. To allow all capabilities you may use '*'.
    pub allowed_capabilities: Vec<String>,

    /// AllowedFlexVolumes is a whitelist of allowed Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the "Volumes" field.
    pub allowed_flex_volumes: Option<Vec<crate::api::security::v1::AllowedFlexVolume>>,

    /// AllowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none. Each entry is either a plain sysctl name or ends in "*" in which case it is considered as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed. Kubelet has to whitelist all allowed unsafe sysctls explicitly to avoid rejection.
    ///
    /// Examples: e.g. "foo/*" allows "foo/bar", "foo/baz", etc. e.g. "foo.*" allows "foo.bar", "foo.baz", etc.
    pub allowed_unsafe_sysctls: Option<Vec<String>>,

    /// DefaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capabiility in both DefaultAddCapabilities and RequiredDropCapabilities.
    pub default_add_capabilities: Vec<String>,

    /// DefaultAllowPrivilegeEscalation controls the default setting for whether a process can gain more privileges than its parent process.
    pub default_allow_privilege_escalation: Option<bool>,

    /// ForbiddenSysctls is a list of explicitly forbidden sysctls, defaults to none. Each entry is either a plain sysctl name or ends in "*" in which case it is considered as a prefix of forbidden sysctls. Single * means all sysctls are forbidden.
    ///
    /// Examples: e.g. "foo/*" forbids "foo/bar", "foo/baz", etc. e.g. "foo.*" forbids "foo.bar", "foo.baz", etc.
    pub forbidden_sysctls: Option<Vec<String>>,

    /// FSGroup is the strategy that will dictate what fs group is used by the SecurityContext.
    pub fs_group: Option<crate::api::security::v1::FSGroupStrategyOptions>,

    /// The groups that have permission to use this security context constraints
    pub groups: Option<Vec<String>>,

    /// Standard object's metadata. More info: http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#metadata
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Priority influences the sort order of SCCs when evaluating which SCCs to try first for a given pod request based on access in the Users and Groups fields.  The higher the int, the higher priority. An unset value is considered a 0 priority. If scores for multiple SCCs are equal they will be sorted from most restrictive to least restrictive. If both priorities and restrictions are equal the SCCs will be sorted by name.
    pub priority: i32,

    /// ReadOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the SCC should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to.
    pub read_only_root_filesystem: bool,

    /// RequiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added.
    pub required_drop_capabilities: Vec<String>,

    /// RunAsUser is the strategy that will dictate what RunAsUser is used in the SecurityContext.
    pub run_as_user: Option<crate::api::security::v1::RunAsUserStrategyOptions>,

    /// SELinuxContext is the strategy that will dictate what labels will be set in the SecurityContext.
    pub se_linux_context: Option<crate::api::security::v1::SELinuxContextStrategyOptions>,

    /// SeccompProfiles lists the allowed profiles that may be set for the pod or container's seccomp annotations.  An unset (nil) or empty value means that no profiles may be specifid by the pod or container.    The wildcard '*' may be used to allow all profiles.  When used to generate a value for a pod the first non-wildcard profile will be used as the default.
    pub seccomp_profiles: Option<Vec<String>>,

    /// SupplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.
    pub supplemental_groups: Option<crate::api::security::v1::SupplementalGroupsStrategyOptions>,

    /// The users who have permissions to use this security context constraints
    pub users: Option<Vec<String>>,

    /// Volumes is a white list of allowed volume plugins.  FSType corresponds directly with the field names of a VolumeSource (azureFile, configMap, emptyDir).  To allow all volumes you may use "*". To allow no volumes, set to \["none"\].
    pub volumes: Vec<String>,
}

// Begin security.openshift.io/v1/SecurityContextConstraints

// Generated from operation createSecurityOpenshiftIoV1SecurityContextConstraints

impl SecurityContextConstraints {
    /// create SecurityContextConstraints
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
    pub fn create_security_context_constraints(
        body: &crate::api::security::v1::SecurityContextConstraints,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/security.openshift.io/v1/securitycontextconstraints?".to_owned();
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

// Generated from operation deleteSecurityOpenshiftIoV1CollectionSecurityContextConstraints

impl SecurityContextConstraints {
    /// delete collection of SecurityContextConstraints
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
    pub fn delete_collection_security_context_constraints(
        delete_optional: k8s_openapi::DeleteOptional<'_>,
        list_optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>), k8s_openapi::RequestError> {
        let __url = "/apis/security.openshift.io/v1/securitycontextconstraints?".to_owned();
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

// Generated from operation deleteSecurityOpenshiftIoV1SecurityContextConstraints

impl SecurityContextConstraints {
    /// delete SecurityContextConstraints
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the SecurityContextConstraints
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_security_context_constraints(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/security.openshift.io/v1/securitycontextconstraints/{name}",
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

// Generated from operation listSecurityOpenshiftIoV1SecurityContextConstraints

impl SecurityContextConstraints {
    /// list or watch objects of kind SecurityContextConstraints
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
    pub fn list_security_context_constraints(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/security.openshift.io/v1/securitycontextconstraints?".to_owned();
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

// Generated from operation patchSecurityOpenshiftIoV1SecurityContextConstraints

impl SecurityContextConstraints {
    /// partially update the specified SecurityContextConstraints
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the SecurityContextConstraints
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_security_context_constraints(
        name: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/security.openshift.io/v1/securitycontextconstraints/{name}?",
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

// Generated from operation readSecurityOpenshiftIoV1SecurityContextConstraints

impl SecurityContextConstraints {
    /// read the specified SecurityContextConstraints
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadSecurityContextConstraintsResponse`]`>` constructor, or [`ReadSecurityContextConstraintsResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the SecurityContextConstraints
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_security_context_constraints(
        name: &str,
        optional: ReadSecurityContextConstraintsOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadSecurityContextConstraintsResponse>), k8s_openapi::RequestError> {
        let ReadSecurityContextConstraintsOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/security.openshift.io/v1/securitycontextconstraints/{name}?",
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

/// Optional parameters of [`SecurityContextConstraints::read_security_context_constraints`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadSecurityContextConstraintsOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadSecurityContextConstraintsResponse as Response>::try_from_parts` to parse the HTTP response body of [`SecurityContextConstraints::read_security_context_constraints`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadSecurityContextConstraintsResponse {
    Ok(crate::api::security::v1::SecurityContextConstraints),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadSecurityContextConstraintsResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadSecurityContextConstraintsResponse::Ok(result), buf.len()))
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
                Ok((ReadSecurityContextConstraintsResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceSecurityOpenshiftIoV1SecurityContextConstraints

impl SecurityContextConstraints {
    /// replace the specified SecurityContextConstraints
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the SecurityContextConstraints
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_security_context_constraints(
        name: &str,
        body: &crate::api::security::v1::SecurityContextConstraints,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/security.openshift.io/v1/securitycontextconstraints/{name}?",
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

// Generated from operation watchSecurityOpenshiftIoV1SecurityContextConstraints

impl SecurityContextConstraints {
    /// list or watch objects of kind SecurityContextConstraints
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
    pub fn watch_security_context_constraints(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/security.openshift.io/v1/securitycontextconstraints?".to_owned();
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

// End security.openshift.io/v1/SecurityContextConstraints

impl k8s_openapi::Resource for SecurityContextConstraints {
    const API_VERSION: &'static str = "security.openshift.io/v1";
    const GROUP: &'static str = "security.openshift.io";
    const KIND: &'static str = "SecurityContextConstraints";
    const VERSION: &'static str = "v1";
    // fixed `Resource` impl
    const URL_PATH_SEGMENT: &'static str = "securitycontextconstraints";
    type Scope = k8s_openapi::ClusterResourceScope;
}

impl k8s_openapi::ListableResource for SecurityContextConstraints {
    const LIST_KIND: &'static str = concat!("SecurityContextConstraints", "List");
}

impl k8s_openapi::Metadata for SecurityContextConstraints {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for SecurityContextConstraints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_allow_host_dir_volume_plugin,
            Key_allow_host_ipc,
            Key_allow_host_network,
            Key_allow_host_pid,
            Key_allow_host_ports,
            Key_allow_privilege_escalation,
            Key_allow_privileged_container,
            Key_allowed_capabilities,
            Key_allowed_flex_volumes,
            Key_allowed_unsafe_sysctls,
            Key_default_add_capabilities,
            Key_default_allow_privilege_escalation,
            Key_forbidden_sysctls,
            Key_fs_group,
            Key_groups,
            Key_metadata,
            Key_priority,
            Key_read_only_root_filesystem,
            Key_required_drop_capabilities,
            Key_run_as_user,
            Key_se_linux_context,
            Key_seccomp_profiles,
            Key_supplemental_groups,
            Key_users,
            Key_volumes,
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
                            "allowHostDirVolumePlugin" => Field::Key_allow_host_dir_volume_plugin,
                            "allowHostIPC" => Field::Key_allow_host_ipc,
                            "allowHostNetwork" => Field::Key_allow_host_network,
                            "allowHostPID" => Field::Key_allow_host_pid,
                            "allowHostPorts" => Field::Key_allow_host_ports,
                            "allowPrivilegeEscalation" => Field::Key_allow_privilege_escalation,
                            "allowPrivilegedContainer" => Field::Key_allow_privileged_container,
                            "allowedCapabilities" => Field::Key_allowed_capabilities,
                            "allowedFlexVolumes" => Field::Key_allowed_flex_volumes,
                            "allowedUnsafeSysctls" => Field::Key_allowed_unsafe_sysctls,
                            "defaultAddCapabilities" => Field::Key_default_add_capabilities,
                            "defaultAllowPrivilegeEscalation" => Field::Key_default_allow_privilege_escalation,
                            "forbiddenSysctls" => Field::Key_forbidden_sysctls,
                            "fsGroup" => Field::Key_fs_group,
                            "groups" => Field::Key_groups,
                            "metadata" => Field::Key_metadata,
                            "priority" => Field::Key_priority,
                            "readOnlyRootFilesystem" => Field::Key_read_only_root_filesystem,
                            "requiredDropCapabilities" => Field::Key_required_drop_capabilities,
                            "runAsUser" => Field::Key_run_as_user,
                            "seLinuxContext" => Field::Key_se_linux_context,
                            "seccompProfiles" => Field::Key_seccomp_profiles,
                            "supplementalGroups" => Field::Key_supplemental_groups,
                            "users" => Field::Key_users,
                            "volumes" => Field::Key_volumes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecurityContextConstraints;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allow_host_dir_volume_plugin: Option<bool> = None;
                let mut value_allow_host_ipc: Option<bool> = None;
                let mut value_allow_host_network: Option<bool> = None;
                let mut value_allow_host_pid: Option<bool> = None;
                let mut value_allow_host_ports: Option<bool> = None;
                let mut value_allow_privilege_escalation: Option<bool> = None;
                let mut value_allow_privileged_container: Option<bool> = None;
                let mut value_allowed_capabilities: Option<Vec<String>> = None;
                let mut value_allowed_flex_volumes: Option<Vec<crate::api::security::v1::AllowedFlexVolume>> = None;
                let mut value_allowed_unsafe_sysctls: Option<Vec<String>> = None;
                let mut value_default_add_capabilities: Option<Vec<String>> = None;
                let mut value_default_allow_privilege_escalation: Option<bool> = None;
                let mut value_forbidden_sysctls: Option<Vec<String>> = None;
                let mut value_fs_group: Option<crate::api::security::v1::FSGroupStrategyOptions> = None;
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_read_only_root_filesystem: Option<bool> = None;
                let mut value_required_drop_capabilities: Option<Vec<String>> = None;
                let mut value_run_as_user: Option<crate::api::security::v1::RunAsUserStrategyOptions> = None;
                let mut value_se_linux_context: Option<crate::api::security::v1::SELinuxContextStrategyOptions> = None;
                let mut value_seccomp_profiles: Option<Vec<String>> = None;
                let mut value_supplemental_groups: Option<crate::api::security::v1::SupplementalGroupsStrategyOptions> = None;
                let mut value_users: Option<Vec<String>> = None;
                let mut value_volumes: Option<Vec<String>> = None;

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
                        Field::Key_allow_host_dir_volume_plugin => value_allow_host_dir_volume_plugin = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_allow_host_ipc => value_allow_host_ipc = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_allow_host_network => value_allow_host_network = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_allow_host_pid => value_allow_host_pid = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_allow_host_ports => value_allow_host_ports = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_allow_privilege_escalation => value_allow_privilege_escalation = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allow_privileged_container => value_allow_privileged_container = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_allowed_capabilities => value_allowed_capabilities = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_allowed_flex_volumes => value_allowed_flex_volumes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_unsafe_sysctls => value_allowed_unsafe_sysctls = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_add_capabilities => value_default_add_capabilities = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_default_allow_privilege_escalation => value_default_allow_privilege_escalation = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_forbidden_sysctls => value_forbidden_sysctls = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group => value_fs_group = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_priority => value_priority = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_read_only_root_filesystem => value_read_only_root_filesystem = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_required_drop_capabilities => value_required_drop_capabilities = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_run_as_user => value_run_as_user = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_se_linux_context => value_se_linux_context = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_seccomp_profiles => value_seccomp_profiles = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_supplemental_groups => value_supplemental_groups = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_users => value_users = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecurityContextConstraints {
                    allow_host_dir_volume_plugin: value_allow_host_dir_volume_plugin.ok_or_else(|| serde::de::Error::missing_field("allowHostDirVolumePlugin"))?,
                    allow_host_ipc: value_allow_host_ipc.ok_or_else(|| serde::de::Error::missing_field("allowHostIPC"))?,
                    allow_host_network: value_allow_host_network.ok_or_else(|| serde::de::Error::missing_field("allowHostNetwork"))?,
                    allow_host_pid: value_allow_host_pid.ok_or_else(|| serde::de::Error::missing_field("allowHostPID"))?,
                    allow_host_ports: value_allow_host_ports.ok_or_else(|| serde::de::Error::missing_field("allowHostPorts"))?,
                    allow_privilege_escalation: value_allow_privilege_escalation,
                    allow_privileged_container: value_allow_privileged_container.ok_or_else(|| serde::de::Error::missing_field("allowPrivilegedContainer"))?,
                    allowed_capabilities: value_allowed_capabilities.ok_or_else(|| serde::de::Error::missing_field("allowedCapabilities"))?,
                    allowed_flex_volumes: value_allowed_flex_volumes,
                    allowed_unsafe_sysctls: value_allowed_unsafe_sysctls,
                    default_add_capabilities: value_default_add_capabilities.ok_or_else(|| serde::de::Error::missing_field("defaultAddCapabilities"))?,
                    default_allow_privilege_escalation: value_default_allow_privilege_escalation,
                    forbidden_sysctls: value_forbidden_sysctls,
                    fs_group: value_fs_group,
                    groups: value_groups,
                    metadata: value_metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    priority: value_priority.ok_or_else(|| serde::de::Error::missing_field("priority"))?,
                    read_only_root_filesystem: value_read_only_root_filesystem.ok_or_else(|| serde::de::Error::missing_field("readOnlyRootFilesystem"))?,
                    required_drop_capabilities: value_required_drop_capabilities.ok_or_else(|| serde::de::Error::missing_field("requiredDropCapabilities"))?,
                    run_as_user: value_run_as_user,
                    se_linux_context: value_se_linux_context,
                    seccomp_profiles: value_seccomp_profiles,
                    supplemental_groups: value_supplemental_groups,
                    users: value_users,
                    volumes: value_volumes.ok_or_else(|| serde::de::Error::missing_field("volumes"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "allowHostDirVolumePlugin",
                "allowHostIPC",
                "allowHostNetwork",
                "allowHostPID",
                "allowHostPorts",
                "allowPrivilegeEscalation",
                "allowPrivilegedContainer",
                "allowedCapabilities",
                "allowedFlexVolumes",
                "allowedUnsafeSysctls",
                "defaultAddCapabilities",
                "defaultAllowPrivilegeEscalation",
                "forbiddenSysctls",
                "fsGroup",
                "groups",
                "metadata",
                "priority",
                "readOnlyRootFilesystem",
                "requiredDropCapabilities",
                "runAsUser",
                "seLinuxContext",
                "seccompProfiles",
                "supplementalGroups",
                "users",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SecurityContextConstraints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            15 +
            self.allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.allowed_flex_volumes.as_ref().map_or(0, |_| 1) +
            self.allowed_unsafe_sysctls.as_ref().map_or(0, |_| 1) +
            self.default_allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.forbidden_sysctls.as_ref().map_or(0, |_| 1) +
            self.fs_group.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.run_as_user.as_ref().map_or(0, |_| 1) +
            self.se_linux_context.as_ref().map_or(0, |_| 1) +
            self.seccomp_profiles.as_ref().map_or(0, |_| 1) +
            self.supplemental_groups.as_ref().map_or(0, |_| 1) +
            self.users.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowHostDirVolumePlugin", &self.allow_host_dir_volume_plugin)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowHostIPC", &self.allow_host_ipc)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowHostNetwork", &self.allow_host_network)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowHostPID", &self.allow_host_pid)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowHostPorts", &self.allow_host_ports)?;
        if let Some(value) = &self.allow_privilege_escalation {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowPrivilegeEscalation", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowPrivilegedContainer", &self.allow_privileged_container)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowedCapabilities", &self.allowed_capabilities)?;
        if let Some(value) = &self.allowed_flex_volumes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowedFlexVolumes", value)?;
        }
        if let Some(value) = &self.allowed_unsafe_sysctls {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowedUnsafeSysctls", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "defaultAddCapabilities", &self.default_add_capabilities)?;
        if let Some(value) = &self.default_allow_privilege_escalation {
            serde::ser::SerializeStruct::serialize_field(&mut state, "defaultAllowPrivilegeEscalation", value)?;
        }
        if let Some(value) = &self.forbidden_sysctls {
            serde::ser::SerializeStruct::serialize_field(&mut state, "forbiddenSysctls", value)?;
        }
        if let Some(value) = &self.fs_group {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fsGroup", value)?;
        }
        if let Some(value) = &self.groups {
            serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "priority", &self.priority)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "readOnlyRootFilesystem", &self.read_only_root_filesystem)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "requiredDropCapabilities", &self.required_drop_capabilities)?;
        if let Some(value) = &self.run_as_user {
            serde::ser::SerializeStruct::serialize_field(&mut state, "runAsUser", value)?;
        }
        if let Some(value) = &self.se_linux_context {
            serde::ser::SerializeStruct::serialize_field(&mut state, "seLinuxContext", value)?;
        }
        if let Some(value) = &self.seccomp_profiles {
            serde::ser::SerializeStruct::serialize_field(&mut state, "seccompProfiles", value)?;
        }
        if let Some(value) = &self.supplemental_groups {
            serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroups", value)?;
        }
        if let Some(value) = &self.users {
            serde::ser::SerializeStruct::serialize_field(&mut state, "users", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", &self.volumes)?;
        serde::ser::SerializeStruct::end(state)
    }
}
