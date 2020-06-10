const MIN: usize = 4;
const MAX: usize = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let enabled_version = {
        let mut enabled_versions =
            (MIN..=MAX).filter(|v| std::env::var(format!("CARGO_FEATURE_V4_{}", v)).is_ok());
        let v1 = enabled_versions.next().expect("\n\
            None of the v4_* features are enabled on the openshift-openapi crate.\n\
            \n\
            The openshift-openapi crate requires a feature to be enabled to indicate which version of OpenShift it should support.\n\
            \n\
            If you're using openshift-openapi in a binary crate, enable the feature corresponding to the minimum version of API server that you want to support. \
            It may be possible that your binary crate does not directly depend on openshift-openapi. In this case, add a dependency on openshift-openapi, then enable \
            the corresponding feature.\n\
            \n\
            If you're using openshift-openapi in a library crate, add a dev-dependency on openshift-openapi and enable one of the features there. This way \
            the feature will be enabled when buildings tests and examples of your library, but not when building the library itself. \
            It may be possible that your library crate does not directly depend on openshift-openapi. In this case, add a dev-dependency on openshift-openapi, \
            then enable the corresponding feature.\n\
            \n\
            Library crates *must not* enable any features in their direct dependency on openshift-openapi, only in their dev-dependency. \
            The choice of Kubernetes version to support should be left to the final binary crate, so only the binary crate should enable a specific feature. \
            If library crates also enable features, it can cause multiple features to be enabled simultaneously, which openshift-openapi does not support.\n\
            \n\
            If you want to restrict your library crate to support only a single specific version or range of versions of Kubernetes, \
            please use the openshift_* version-specific macros to emit different code based on which feature gets enabled in the end.\
        ");
        if let Some(v2) = enabled_versions.next() {
            panic!(
                "\n\
                    Both v4_{} and v4_{} features are enabled on the openshift-openapi crate. Only one feature can be enabled at the same time.\n\
                    \n\
                    The feature indicates which version of Kubernetes the openshift-openapi crate should support.\n\
                    \n\
                    If you have enabled both of these features yourself, please remove one of them. If you are writing a library crate, \
                    do not enable any features at all. Library crates *must not* enable any features on the openshift-openapi crate.\n\
                    \n\
                    If you have not enabled one or both of these features yourself, then one of the library crates in your dependency graph *has*. \
                    Locate which library crates in your dependency graph depend on openshift-openapi and enable one of its features, and file a bug against them. \
                    You can search your Cargo.lock for \"openshift-openapi\" to discover these crates.\
                ",
                v1,
                v2,
            );
        }
        v1
    };

    println!(
        "cargo:version={}",
        0x00_04_00_00_u32 | ((enabled_version as u32) << 8)
    );

    let mut f = {
        let mut out_file: std::path::PathBuf = std::env::var_os("OUT_DIR")
            .ok_or_else(|| "OUT_DIR not set")?
            .into();
        out_file.push("conditional_compilation_macros.rs");
        std::io::BufWriter::new(std::fs::File::create(out_file)?)
    };

    for v in MIN..=MAX {
        writeln!(f, "/// This macro evaluates to its contents if the `v4_{}` feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "///")?;
        writeln!(f, "/// # Examples")?;
        writeln!(f, "///")?;
        writeln!(f, "/// ```rust")?;
        writeln!(f, "/// # #[macro_use] extern crate openshift_openapi;")?;
        writeln!(f, "/// openshift_if_4_{}! {{", v)?;
        writeln!(f, "///     use openshift_openapi::api::build::v1 as api;")?;
        writeln!(f, "/// }}")?;
        writeln!(f, "/// ```")?;
        if enabled_version == v {
            writeln!(
                f,
                "#[macro_export] macro_rules! openshift_if_4_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}",
                v
            )?;
        } else {
            writeln!(
                f,
                "#[macro_export] macro_rules! openshift_if_4_{} {{ ($($tt:tt)*) => {{ }}; }}",
                v
            )?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v4_{}` or higher feature is enabled, otherwise it evaluates to nothing.", v)?;
        if enabled_version >= v {
            writeln!(
                f,
                "#[macro_export] macro_rules! openshift_if_ge_4_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}",
                v
            )?;
        } else {
            writeln!(
                f,
                "#[macro_export] macro_rules! openshift_if_ge_4_{} {{ ($($tt:tt)*) => {{ }}; }}",
                v
            )?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v4_{}` or lower feature is enabled, otherwise it evaluates to nothing.", v)?;
        if enabled_version <= v {
            writeln!(
                f,
                "#[macro_export] macro_rules! openshift_if_le_4_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}",
                v
            )?;
        } else {
            writeln!(
                f,
                "#[macro_export] macro_rules! openshift_if_le_4_{} {{ ($($tt:tt)*) => {{ }}; }}",
                v
            )?;
        }
        writeln!(f)?;
    }

    writeln!(
        f,
        "/// A macro that emits a `match` expr with the given test expression and arms."
    )?;
    writeln!(f, "/// The match arms can be annotated with the other conditional compilation macros in this crate so that they're only emitted")?;
    writeln!(f, "/// if the predicate is true.")?;
    writeln!(f, "#[macro_export] macro_rules! openshift_match {{")?;
    writeln!(
        f,
        "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ }}) => {{"
    )?;
    writeln!(f, "        match $test {{ $($arms)* }}")?;
    writeln!(f, "    }};")?;

    for v in MIN..=MAX {
        writeln!(f)?;

        for (name_suffix, enabled) in &[
            ("", enabled_version == v),
            ("_ge", enabled_version >= v),
            ("_le", enabled_version <= v),
        ] {
            writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ openshift_if{}_1_{}!($($arm:tt)*), $($rest:tt)* }}) => {{", name_suffix, v)?;
            if *enabled {
                writeln!(f, "        openshift_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($arm)*, $($rest)* }})")?;
            } else {
                writeln!(
                    f,
                    "        openshift_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($rest)* }})"
                )?;
            }
            writeln!(f, "    }};")?;
        }
    }

    writeln!(f)?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ $next_pat:pat $(if $cond:expr)? => $next_expr:expr, $($rest:tt)* }}) => {{")?;
    writeln!(f, "        openshift_match!(@inner {{ $test }} {{ $($arms)* $next_pat $(if $cond)? => $next_expr, }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f)?;
    writeln!(f, "    ($test:expr, {{ $($rest:tt)* }}) => {{")?;
    writeln!(
        f,
        "        openshift_match!(@inner {{ $test }} {{ }} {{ $($rest)* }})"
    )?;
    writeln!(f, "    }};")?;
    writeln!(f, "}}")?;

    Ok(())
}
