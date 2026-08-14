// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    fs,
    path::Path,
};

use crate::{
    error::{
        Error,
        Result,
    },
    fsutil::{
        dir_nonempty,
        mirror_tree,
    },
};

pub fn parse_adapters_spec(spec: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for tok in spec.split(|c: char| c == ',' || c.is_whitespace()) {
        if tok.is_empty() {
            continue;
        }
        match tok {
            "all" => {
                return Ok(vec!["grok".into(), "claude".into(), "codex".into()]);
            }
            "none" => return Ok(Vec::new()),
            "grok" | "claude" | "codex" => {
                if !out.iter().any(|a| a == tok) {
                    out.push(tok.to_string());
                }
            }
            other => return Err(Error::UnknownAdapter(other.to_string())),
        }
    }
    Ok(out)
}

pub fn add_adapter(list: &mut Vec<String>, name: &str) -> Result<()> {
    match name {
        "all" => {
            *list = vec!["grok".into(), "claude".into(), "codex".into()];
        }
        "none" => list.clear(),
        "grok" | "claude" | "codex" => {
            if !list.iter().any(|a| a == name) {
                list.push(name.to_string());
            }
        }
        other => return Err(Error::UnknownAdapter(other.to_string())),
    }
    Ok(())
}

pub fn resolve_adapters(
    spec: Option<&str>,
    extra: &[String],
    adapters_all: bool,
    default: &[String],
) -> Result<Vec<String>> {
    if adapters_all {
        return Ok(vec!["grok".into(), "claude".into(), "codex".into()]);
    }
    let mut explicit = false;
    let mut out = Vec::new();
    if let Some(s) = spec {
        explicit = true;
        out = parse_adapters_spec(s)?;
    }
    for name in extra {
        if !explicit {
            out.clear();
            explicit = true;
        }
        add_adapter(&mut out, name)?;
    }
    if !explicit {
        out = default.to_vec();
    }
    Ok(out)
}

pub fn write_selected_adapters(target: &Path, adapters: &[String]) -> Result<()> {
    let label = if adapters.is_empty() {
        "none".to_string()
    } else {
        adapters.join(" ")
    };
    println!("Writing adapters ({label})...");
    if adapters.is_empty() {
        println!("  (canonical .agents/ only; no vendor mirrors)");
        return Ok(());
    }
    if adapters.iter().any(|a| a == "grok") {
        mirror_vendor_tree(target, ".grok")?;
    }
    if adapters.iter().any(|a| a == "claude") {
        write_claude_pointer(target)?;
        mirror_vendor_tree(target, ".claude")?;
    }
    if adapters.iter().any(|a| a == "codex") {
        mirror_codex_skills(target)?;
    }
    Ok(())
}

fn write_claude_pointer(target: &Path) -> Result<()> {
    if target.join("AGENTS.md").is_file() {
        fs::write(
            target.join("CLAUDE.md"),
            "@AGENTS.md\n\n<!-- Adapter for Claude Code. Canonical always-on rules live in AGENTS.md (symrig). -->\n",
        )?;
        println!("  adapter CLAUDE.md → @AGENTS.md");
    }
    Ok(())
}

fn mirror_vendor_tree(target: &Path, vendor_root: &str) -> Result<()> {
    for kind in ["rules", "skills", "agents"] {
        let src = target.join(".agents").join(kind);
        if dir_nonempty(&src) {
            let dest = target.join(vendor_root).join(kind);
            mirror_tree(&src, &dest)?;
            println!("  adapter {vendor_root}/{kind}/ ← .agents/{kind}/");
        }
    }
    Ok(())
}

fn mirror_codex_skills(target: &Path) -> Result<()> {
    let src = target.join(".agents").join("skills");
    if dir_nonempty(&src) {
        let dest = target.join(".codex").join("skills");
        mirror_tree(&src, &dest)?;
        println!("  adapter .codex/skills/ ← .agents/skills/");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_all_none() {
        assert_eq!(parse_adapters_spec("all").unwrap(), ["grok", "claude", "codex"]);
        assert!(parse_adapters_spec("none").unwrap().is_empty());
        assert_eq!(parse_adapters_spec("grok,claude").unwrap(), ["grok", "claude"]);
    }
}
