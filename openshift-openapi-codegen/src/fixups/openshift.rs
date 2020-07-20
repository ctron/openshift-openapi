#![deny(unused)]

use k8s_openapi_codegen_common::swagger20::Method;

pub(crate) fn remove_legacy_gvk(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let mut found = false;

    for (k, v) in &mut spec.definitions {
        if !k.starts_with("com.github.openshift") {
            continue;
        }

        v.kubernetes_group_kind_versions.retain(|gvk| {
            // drop the legacy, empty group
            if gvk.group.is_empty() {
                found = true;
                false
            } else {
                true
            }
        });
    }

    if found {
        Ok(())
    } else {
        Err("never applied remove_legacy_gvk fixup".into())
    }
}

pub(crate) fn fix_imagestream_secrets_list(
    spec: &mut crate::swagger20::Spec,
) -> Result<(), crate::Error> {
    let mut found = false;

    spec.operations.retain(|op| {
        if op.path.0
            == "/apis/image.openshift.io/v1/namespaces/{namespace}/imagestreams/{name}/secrets"
            && op.method == Method::Get
        {
            found = true;
            false
        } else {
            true
        }
    });

    for (k, v) in &mut spec.definitions {
        if k.0 != "io.k8s.api.core.v1.SecretList" {
            continue;
        }

        v.kubernetes_group_kind_versions.retain(|gvk| {
            // drop the legacy, empty group
            if gvk.group == "image.openshift.io" && gvk.kind == "SecretList" {
                found = true;
                false
            } else {
                true
            }
        });
    }

    if found {
        Ok(())
    } else {
        Err("never applied remove_legacy_gvk fixup".into())
    }
}

// Path operation annotated with a "x-kubernetes-group-version-kind" that references a type that doesn't exist in the schema.
#[allow(clippy::if_same_then_else)]
pub(crate) fn connect_options_gvk(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let mut found = false;

    for operation in &mut spec.operations {
        if let Some(kubernetes_group_kind_version) = &mut operation.kubernetes_group_kind_version {
            if kubernetes_group_kind_version.group == "build.openshift.io"
                && kubernetes_group_kind_version.kind == "BinaryBuildRequestOptions"
                && kubernetes_group_kind_version.version == "v1"
            {
                kubernetes_group_kind_version.kind = "Build".to_string();
                found = true;
            }
        }
    }

    if found {
        Ok(())
    } else {
        Err("never applied connect options kubernetes_group_kind_version override".into())
    }
}
