// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("catalog not found: {0}")]
    CatalogMissing(PathBuf),
    #[error("cannot find catalog.yaml (set SYMKIT_ROOT or run from a checkout; cargo-install uses an embedded copy)")]
    KitRootNotFound,
    #[error("unknown harness '{name}' (have: {known})")]
    UnknownHarness { name: String, known: String },
    #[error("harness '{0}' is not installable (status={1})")]
    NotInstallable(String, String),
    #[error("unknown role '{role}' for {harness} (have: {known})")]
    UnknownRole {
        role: String,
        harness: String,
        known: String,
    },
    #[error("--role required for harness '{0}' (no default)")]
    RoleRequired(String),
    #[error("unknown package(s) for {harness}: {names}")]
    UnknownPackages { harness: String, names: String },
    #[error("unknown package '{package}' in harness '{harness}'")]
    UnknownPackage { harness: String, package: String },
    #[error("unknown doc template '{name}' for {harness} (have: {known})")]
    UnknownDocTemplate {
        name: String,
        harness: String,
        known: String,
    },
    #[error("doc template file missing: {0}")]
    DocTemplateMissing(PathBuf),
    #[error("both docs/ and documents/ exist; pass --docs-root docs or --docs-root documents")]
    AmbiguousDocsRoot,
    #[error("invalid --docs-root '{0}' (relative path only; not .symkit/.agents/vendor trees)")]
    InvalidDocsRoot(String),
    #[error("package path missing: {0}")]
    PackagePathMissing(PathBuf),
    #[error("unknown library skill '{name}' (looked in {path})")]
    UnknownSkill { name: String, path: PathBuf },
    #[error("unknown adapter '{0}' (grok|claude|codex|all|none)")]
    UnknownAdapter(String),
    #[error("missing target directory")]
    MissingTarget,
    #[error("--harness is required")]
    MissingHarness,
    #[error("target is not a directory: {0}")]
    TargetNotDir(PathBuf),
    #[error("refusing to install into the symkit repo itself")]
    RefuseSelfInstall,
    #[error("refusing to write without --yes when stdin is not a TTY")]
    NeedYes,
    #[error("usage: symkit show <harness>")]
    ShowUsage,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("catalog.yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("{0}")]
    Msg(String),
}
