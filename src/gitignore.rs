// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    fs::{
        self,
        OpenOptions,
    },
    io::Write,
    path::Path,
};

use crate::error::Result;

const PATTERNS: &[&str] = &[
    ".agents/", ".grok/", ".claude/", ".codex/", ".symkit/", "*~", ".*.swp", ".*.swo", "*.un~", "\\#*\\#", ".#*",
];
const MARKER_BEGIN: &str = "# BEGIN symkit agent trees (do not commit)";
const MARKER_END: &str = "# END symkit agent trees";

pub fn ensure_agent_gitignore(target: &Path) -> Result<String> {
    let gi = target.join(".gitignore");
    if !gi.exists() {
        fs::write(&gi, "")?;
        println!("  created {}", gi.display());
    }

    let existing = fs::read_to_string(&gi)?;
    let missing: Vec<&str> = PATTERNS.iter().copied().filter(|p| !existing.contains(p)).collect();

    if missing.is_empty() {
        println!("  gitignore: agent trees already covered");
        return Ok("covered".into());
    }

    let mut f = OpenOptions::new().append(true).open(&gi)?;
    writeln!(f)?;
    writeln!(f, "{MARKER_BEGIN}")?;
    writeln!(
        f,
        "# Installed by symkit — local agent config and editor swap/backup files"
    )?;
    for p in &missing {
        writeln!(f, "{p}")?;
    }
    writeln!(f, "{MARKER_END}")?;
    println!("  gitignore: ensured {}", missing.join(" "));
    Ok(missing.join(" "))
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn additive() {
        let dir = tempdir().unwrap();
        ensure_agent_gitignore(dir.path()).unwrap();
        let text = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(text.contains(".agents/"));
        assert!(text.contains(".symkit/"));
        assert!(text.contains(".*.swp"));
        assert!(text.contains(".#*"));
        ensure_agent_gitignore(dir.path()).unwrap();
        let again = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert_eq!(again.matches("# BEGIN symkit agent trees").count(), 1);
    }
}
