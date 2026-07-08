include!(concat!(env!("OUT_DIR"), "/built.rs"));

#[derive(Debug, Clone, Copy)]
pub struct BuiltInfo {
    /// The name of the package.
    pub pkg_name: &'static str,
    /// The full version.
    pub pkg_version: &'static str,

    /// The target triple that was being compiled for.
    pub target: &'static str,
    /// The host triple of the rust compiler.
    pub host: &'static str,
    /// `release` for release builds, `debug` for other builds.
    pub profile: &'static str,

    /// The output of `rustc -V`
    pub rustc_version: &'static str,

    /// Value of OPT_LEVEL for the profile used during compilation.
    pub opt_level: &'static str,
    /// Value of DEBUG for the profile used during compilation.
    pub debug: bool,

    /// The features that were enabled during compilation.
    pub features: &'static [&'static str],

    /// The target architecture, given by `CARGO_CFG_TARGET_ARCH`.
    pub cfg_target_arch: &'static str,
    /// The endianness, given by `CARGO_CFG_TARGET_ENDIAN`.
    pub cfg_endian: &'static str,
    /// The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`.
    pub cfg_env: &'static str,
    /// The OS-family, given by `CARGO_CFG_TARGET_FAMILY`.
    pub cfg_family: &'static str,
    /// The operating system, given by `CARGO_CFG_TARGET_OS`.
    pub cfg_os: &'static str,
    /// The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`.
    pub cfg_pointer_width: &'static str,

    /// The override-variables that were used during compilation.
    pub override_variables_used: &'static [&'static str],

    /// If the crate was compiled from within a git-repository,
    /// `git_version` contains HEAD's tag. The short commit id is used
    /// if HEAD is not tagged.
    pub git_version: Option<&'static str>,
    /// If the repository had dirty/staged files.
    pub git_dirty: Option<bool>,
    /// If the crate was compiled from within a git-repository,
    /// `git_head_ref` contains the full name of the reference pointed to
    /// by HEAD (e.g. `refs/heads/master`). `None` if HEAD is detached or
    /// the branch name is not valid UTF-8.
    pub git_head_ref: Option<&'static str>,
    /// If the crate was compiled from within a git-repository,
    /// `git_commit_hash` contains HEAD's full commit SHA-1 hash.
    pub git_commit_hash: Option<&'static str>,
}

impl BuiltInfo {
    pub fn new() -> Self {
        Self {
            pkg_name: PKG_NAME,
            pkg_version: PKG_VERSION,
            target: TARGET,
            host: HOST,
            profile: PROFILE,
            rustc_version: RUSTC_VERSION,
            opt_level: OPT_LEVEL,
            debug: DEBUG,
            features: &FEATURES_LOWERCASE,
            cfg_target_arch: CFG_TARGET_ARCH,
            cfg_endian: CFG_ENDIAN,
            cfg_env: CFG_ENV,
            cfg_family: CFG_FAMILY,
            cfg_os: CFG_OS,
            cfg_pointer_width: CFG_POINTER_WIDTH,
            override_variables_used: &OVERRIDE_VARIABLES_USED,

            git_version: GIT_VERSION,
            git_dirty: GIT_DIRTY,
            git_head_ref: GIT_HEAD_REF,
            git_commit_hash: GIT_COMMIT_HASH,
        }
    }
}

#[cfg(test)]
mod test {
    use super::BuiltInfo;

    #[test]
    fn print_built_info() {
        let info = BuiltInfo::new();
        eprintln!("{info:#?}");
    }
}
