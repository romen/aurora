use std::env;
use std::error::Error;
use std::process::Command;

fn get_git_describe() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["describe", "--tags"])
        .output()
        .expect("Failed to execute git describe");

    if !output.status.success() {
        return Err(format!("git describe failed with status: {}", output.status).into());
    }

    let git_describe = match String::from_utf8(output.stdout) {
        Ok(str) => str.trim().to_string(),
        Err(e) => return Err(format!("Invalid UTF-8 output: {e:?}").into()),
    };

    Ok(git_describe)
}

#[cfg(feature = "_transcoders_deps")]
fn compile_with_rasn() -> Result<(), Box<dyn Error>> {
    use rasn_compiler::prelude::*;
    use rasn_compiler::OutputMode;
    use std::env;
    use std::path::PathBuf;

    let asn_files = [
        "data/asn1/X509-ML-DSA-2025.asn",
        "data/asn1/X509-Composite-ML-DSA-2025.asn",
        "data/asn1/X509-SLH-DSA-Module-2024.asn",
    ];
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_path.join("rasn-generated.rs");

    for path in &asn_files {
        println!("cargo::rerun-if-changed={path}");
    }

    eprintln!("rasn-compiler output to be written at {out_file:?}");

    // Initialize the compiler with the rust/rasn backend.
    match Compiler::<RasnBackend, _>::new()
        // add a single ASN1 source file
        //.add_asn_by_path(PathBuf::from("data/asn1/X509-ML-DSA-2025.asn"))
        // add several ASN1 source files
        .add_asn_sources_by_path(asn_files.iter())
        // set an output path for the generated rust code
        .set_output_mode(OutputMode::SingleFile(out_file))
        // you may also compile literal ASN1 snippets
        //.add_asn_literal(
        //    format!(
        //        "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
        //        "My-test-integer ::= INTEGER (1..128)"
        //    )
        //)
        .compile()
    {
        Ok(warnings) => {
            /* handle compilation warnings */
            for w in warnings {
                println!("cargo::warning=rasn-compiler issued {w:?}");
            }
            Ok(())
        }
        Err(error) => {
            /* handle unrecoverable compilation error */
            let msg = format!("rasn-compiler failed with: {error:?}");
            Err(msg.into())
        }
    }
}

fn check_profile(profile: &str) -> Result<(), Box<dyn Error>> {
    if profile != "debug" {
        // NOTE: no newlines for cargo::warning!
        let warning = format!(
            "This project is NOT production-ready. \
            (Requested PROFILE={profile:?})."
        );
        println!("cargo::warning={warning:}");
    }

    Ok(())
}

#[cfg(feature = "built_info")]
mod built_support {

    /// Return `Some(true)` if the working tree is dirty, `Some(false)`
    /// if clean, and `None` if we could not determine it (not a repo,
    /// or git2 error such as a shallow CI clone).
    /// Mirrors how `built` itself may end up with `None`.
    fn repo_is_dirty(repo: &git2::Repository) -> Option<bool> {
        // Exclude untracked and ignored files from the "dirty"
        // judgement to match the common definition (tracked-content
        // changes).
        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(false).include_ignored(false);

        let statuses = repo.statuses(Some(&mut opts)).ok()?;
        let dirty = !statuses.is_empty();

        Some(dirty)
    }

    /// Policy (a deliberate cost/accuracy tradeoff):
    ///
    ///  * Not in a git repo  -> emit no git-related rerun directive.
    ///    `built` writes None for the git fields; nothing to keep
    ///    fresh.
    ///    (Cargo still re-runs the script if a crate source file
    ///    changes, via its default package scan, so non-git metadata
    ///    stays correct.)
    ///
    ///  * In a repo          -> watch `.git/HEAD`.
    ///    Re-runs on commit / checkout / branch switch, refreshing
    ///    GIT_COMMIT_HASH and GIT_HEAD_REF.
    ///    We do NOT force an every-build rerun here, preserving
    ///    build caching.
    ///
    ///  * In a repo, dirty   -> force an unconditional rerun (via a
    ///    path that never exists, which Cargo always treats as
    ///    "changed").
    ///    Once dirty, we re-observe on every build so GIT_DIRTY
    ///    tracks further edits and the eventual return to clean.
    ///
    /// KNOWN LIMITATION (accepted): a clean -> dirty transition
    /// caused by editing a file that is NOT one of THIS crate's
    /// build inputs (e.g., only README.md, or a sibling crate) will
    /// not trigger a rerun, so GIT_DIRTY can read stale-clean until
    /// some build input or HEAD changes.
    /// Edits to this crate's compiled sources DO trigger Cargo's
    /// normal rebuild, which re-observes dirtiness.
    /// In other words, GIT_DIRTY is fresh relative to this crate's
    /// build inputs + HEAD, not relative to the entire work tree.
    pub fn configure_built_rerun() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR is always set for build scripts");

        // Discover the enclosing repo (walking up from `manifest_dir`).
        // `None` if not in a repo or git2 errors.
        if let Some(repo) = git2::Repository::discover(manifest_dir).ok() {
            // `.path()` is the resolved git dir (e.g., `.../.git/`, or
            // the real dir a worktree/submodule gitfile points at).
            let git_dir = repo.path().to_path_buf();

            let git_head = git_dir.join("HEAD");
            if git_head.exists() {
                // Ask Cargo to watch HEAD so commits/checkouts
                // refresh the commit hash without disabling caching.
                println!("cargo:rerun-if-changed={}", git_head.display());
            } else {
                // Fallback: watch the git dir itself so ref changes
                // still trigger a rerun, even under reftable / worktree
                // / bare layouts where HEAD isn't a loose file at
                // path()/HEAD.
                println!("cargo:rerun-if-changed={}", git_dir.display());
            }

            if Some(true) == repo_is_dirty(&repo) {
                // Dirty: force re-run every build to keep GIT_DIRTY live.
                println!("cargo:rerun-if-changed=__force_rerun_while_dirty__");
            }
        } else {
            // Nothing to emit, this is intentionally empty
        }
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    // Always rerun if the variable FORCE_REBUILD changes
    println!("cargo::rerun-if-env-changed=FORCE_REBUILD");

    let profile = env::var("PROFILE").unwrap_or_default();
    check_profile(&profile)?;

    let git_describe = get_git_describe().unwrap_or_else(|e| {
        println!("cargo::warning=Failed to get git describe");
        eprintln!("Error was {e:?}");
        "FAILED_TO_GATHER_GIT_DESCRIBE".to_string()
    });

    #[cfg(feature = "_transcoders_deps")]
    compile_with_rasn().map_err(|e| {
        eprintln!("{e:?}");
        "rasn-compiler failed"
    })?;

    println!("cargo::rustc-env=CARGO_GIT_DESCRIBE={}", git_describe);

    #[cfg(feature = "built_info")]
    {
        built::write_built_file().expect("Failed to acquire build-time information");
        built_support::configure_built_rerun();
    }

    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        println!("cargo::error={err}");
        std::process::exit(1);
    }
}
