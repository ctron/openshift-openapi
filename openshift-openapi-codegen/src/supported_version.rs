pub(crate) const ALL: &[SupportedVersion] = &[
    SupportedVersion::V4_2,
    SupportedVersion::V4_3,
    SupportedVersion::V4_4,
    SupportedVersion::V4_5,
    SupportedVersion::V4_6,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
    V4_2,
    V4_3,
    V4_4,
    V4_5,
    V4_6,
}

impl SupportedVersion {
    pub(crate) fn mod_root(self) -> &'static str {
        match self {
            SupportedVersion::V4_2 => "v4_2",
            SupportedVersion::V4_3 => "v4_3",
            SupportedVersion::V4_4 => "v4_4",
            SupportedVersion::V4_5 => "v4_5",
            SupportedVersion::V4_6 => "v4_6",
        }
    }

    pub(crate) fn spec_url(self) -> &'static str {
        match self {
            SupportedVersion::V4_2 => "https://raw.githubusercontent.com/openshift/origin/release-4.2/api/swagger-spec/openshift-openapi-spec.json",
            SupportedVersion::V4_3 => "https://raw.githubusercontent.com/openshift/origin/release-4.3/api/swagger-spec/openshift-openapi-spec.json",
            SupportedVersion::V4_4 => "https://raw.githubusercontent.com/openshift/origin/release-4.4/api/swagger-spec/openshift-openapi-spec.json",
            SupportedVersion::V4_5 => "https://raw.githubusercontent.com/openshift/origin/release-4.5/api/swagger-spec/openshift-openapi-spec.json",
            SupportedVersion::V4_6 => "https://raw.githubusercontent.com/openshift/origin/release-4.6/api/swagger-spec/openshift-openapi-spec.json",
        }
    }

    pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        #[allow(clippy::match_same_arms)]
        let upstream_bugs_fixups: &[fn(
            &mut crate::swagger20::Spec,
        ) -> Result<(), crate::Error>] = match self {
            SupportedVersion::V4_2 => &[
                crate::fixups::openshift::connect_options_gvk,
                crate::fixups::openshift::fix_imagestream_secrets_list,
                crate::fixups::openshift::remove_legacy_gvk,
            ],
            SupportedVersion::V4_3 => &[
                crate::fixups::openshift::connect_options_gvk,
                crate::fixups::openshift::fix_imagestream_secrets_list,
                crate::fixups::openshift::remove_legacy_gvk,
            ],
            SupportedVersion::V4_4 => &[
                crate::fixups::openshift::connect_options_gvk,
                crate::fixups::openshift::fix_imagestream_secrets_list,
                crate::fixups::openshift::remove_legacy_gvk,
            ],
            SupportedVersion::V4_5 => &[
                crate::fixups::openshift::connect_options_gvk,
                crate::fixups::openshift::fix_imagestream_secrets_list,
                crate::fixups::openshift::remove_legacy_gvk,
            ],
            SupportedVersion::V4_6 => &[
                crate::fixups::openshift::connect_options_gvk,
                crate::fixups::openshift::fix_imagestream_secrets_list,
                crate::fixups::openshift::remove_legacy_gvk,
            ],
        };

        let special_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = &[
            crate::fixups::special::create_delete_optional,
            crate::fixups::special::create_optionals,
            crate::fixups::special::patch,
            crate::fixups::special::remove_delete_collection_operations_query_parameters,
            crate::fixups::special::remove_delete_operations_query_parameters,
            crate::fixups::special::separate_watch_from_list_operations,
            crate::fixups::special::watch_event,
            crate::fixups::special::list, // Must run after separate_watch_from_list_operations
            crate::fixups::special::response_types,
            crate::fixups::special::resource_metadata_not_optional,
        ];

        for fixup in upstream_bugs_fixups.iter().chain(special_fixups) {
            fixup(spec)?;
        }

        Ok(())
    }
}
