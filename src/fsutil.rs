// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    fs,
    path::Path,
};

use walkdir::WalkDir;

use crate::error::Result;

/// Copy every file under `src` into `dest`, overwriting existing files.
pub fn merge_tree(src: &Path, dest: &Path) -> Result<Vec<String>> {
    copy_tree(src, dest, true)
}

/// Copy files under `src` into `dest`. Skip existing files unless `force`.
pub fn merge_tree_safe(src: &Path, dest: &Path, force: bool) -> Result<Vec<String>> {
    copy_tree(src, dest, force)
}

fn copy_tree(src: &Path, dest: &Path, overwrite: bool) -> Result<Vec<String>> {
    let mut copied = Vec::new();
    if !src.is_dir() {
        return Ok(copied);
    }
    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = match entry.path().strip_prefix(src) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let dest_path = dest.join(rel);
        if dest_path.exists() && !overwrite {
            copied.push(format!("skip {}", rel.display()));
            continue;
        }
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(entry.path(), &dest_path)?;
        copied.push(rel.display().to_string());
    }
    Ok(copied)
}

pub fn dir_nonempty(path: &Path) -> bool {
    path.is_dir()
        && WalkDir::new(path)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .any(|e| e.file_type().is_file())
}

/// Replace `dest` with a copy of `src` (rsync --delete equivalent for trees).
pub fn mirror_tree(src: &Path, dest: &Path) -> Result<()> {
    if dest.exists() {
        fs::remove_dir_all(dest)?;
    }
    merge_tree(src, dest)?;
    Ok(())
}
