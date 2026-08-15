// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    env,
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use include_dir::{
    Dir,
    include_dir,
};

use crate::error::{
    Error,
    Result,
};

static EMBEDDED_CATALOG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/catalog.yaml"));
static EMBEDDED_CORE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/core");
static EMBEDDED_HARNESSES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/harnesses");

/// A checkout is a kit root when it has catalog.yaml and harnesses/.
pub fn is_kit_root(dir: &Path) -> bool {
    dir.join("catalog.yaml").is_file() && dir.join("harnesses").is_dir()
}

/// Prefer SYMKIT_ROOT, then a checkout next to the exe or cwd, then the
/// versioned cache filled from the binary (cargo install).
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

    ensure_embedded_kit()
}

/// Data dir /symkit/<crate version>/ with catalog.yaml, core/, harnesses/.
pub fn embedded_kit_dir() -> PathBuf {
    data_dir().join(env!("CARGO_PKG_VERSION"))
}

fn data_dir() -> PathBuf {
    if let Some(p) = env::var_os("SYMKIT_DATA") {
        return PathBuf::from(p);
    }
    if let Some(p) = env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(p).join("symkit");
    }
    #[cfg(windows)]
    {
        if let Some(p) = env::var_os("LOCALAPPDATA") {
            return PathBuf::from(p).join("symkit");
        }
    }
    if let Some(p) = env::var_os("HOME") {
        return PathBuf::from(p).join(".local/share/symkit");
    }
    env::temp_dir().join("symkit")
}

fn ensure_embedded_kit() -> Result<PathBuf> {
    let dest = embedded_kit_dir();
    if is_kit_root(&dest) {
        return Ok(dest);
    }
    extract_embedded_kit(&dest)?;
    if is_kit_root(&dest) {
        Ok(dest)
    } else {
        Err(Error::KitRootNotFound)
    }
}

fn extract_embedded_kit(dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;
    EMBEDDED_CORE.extract(dest.join("core"))?;
    EMBEDDED_HARNESSES.extract(dest.join("harnesses"))?;
    fs::write(dest.join("catalog.yaml"), EMBEDDED_CATALOG)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_extract_is_kit_root() {
        let dir = tempfile::tempdir().unwrap();
        extract_embedded_kit(dir.path()).unwrap();
        assert!(is_kit_root(dir.path()));
        assert!(dir.path().join("core/rules").is_dir());
        assert!(dir.path().join("harnesses/teaching").is_dir());
        let cat = fs::read_to_string(dir.path().join("catalog.yaml")).unwrap();
        assert!(cat.contains("name: symkit"));
    }
}
