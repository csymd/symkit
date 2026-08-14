// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

use crate::error::{
    Error,
    Result,
};

/// A checkout is a kit root when it has catalog.yaml and harnesses/.
pub fn is_kit_root(dir: &Path) -> bool {
    dir.join("catalog.yaml").is_file() && dir.join("harnesses").is_dir()
}

pub fn find_kit_root() -> Result<PathBuf> {
    if let Ok(raw) = env::var("SYMKIT_ROOT") {
        let p = PathBuf::from(raw);
        if is_kit_root(&p) {
            return Ok(std::fs::canonicalize(&p).unwrap_or(p));
        }
        return Err(Error::CatalogMissing(p.join("catalog.yaml")));
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.to_path_buf());
        }
    }
    if let Ok(cwd) = env::current_dir() {
        candidates.push(cwd);
    }

    for start in candidates {
        let mut cur = Some(start.as_path());
        while let Some(dir) = cur {
            if is_kit_root(dir) {
                return Ok(std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf()));
            }
            cur = dir.parent();
        }
    }
    Err(Error::KitRootNotFound)
}
