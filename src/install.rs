// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    fs,
    io::{
        self,
        IsTerminal,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};

use crate::{
    adapters::write_selected_adapters,
    catalog::{
        Catalog,
        PruneSpec,
        Resolve,
    },
    error::{
        Error,
        Result,
    },
    fsutil::{
        merge_tree,
        merge_tree_safe,
    },
    gitignore::ensure_agent_gitignore,
    kit::is_kit_root,
};

pub struct InstallRequest {
    pub kit_root: PathBuf,
    pub target: PathBuf,
    pub catalog: Catalog,
    pub resolved: Resolve,
    pub adapters: Vec<String>,
    pub scaffold: bool,
    pub force: bool,
    pub prune: bool,
    pub yes: bool,
    pub dry_run: bool,
}

pub fn preview(req: &InstallRequest) {
    println!("Kit:      {}", req.kit_root.display());
    println!("Target:   {}", req.target.display());
    println!("Harness:  {}", req.resolved.harness);
    let role = if req.resolved.role.is_empty() {
        "(explicit packs)"
    } else {
        &req.resolved.role
    };
    println!("Role:     {role}");
    let packs = if req.resolved.packages.is_empty() {
        "(none)".to_string()
    } else {
        req.resolved.packages.join(" ")
    };
    println!("Packs:    {packs}");
    let skills = if req.resolved.skills.is_empty() {
        "(none)".to_string()
    } else {
        req.resolved.skills.join(" ")
    };
    println!("Skills:   {skills}");
    let adapters = if req.adapters.is_empty() {
        "none".to_string()
    } else {
        req.adapters.join(" ")
    };
    println!("Adapters: {adapters}");
    let scaf = if req.scaffold { "1" } else { "0" };
    if req.resolved.workspace.is_empty() {
        println!("Scaffold: {scaf}");
    } else {
        println!("Scaffold: {scaf} ({})", req.resolved.workspace);
    }
    println!(
        "AGENTS.md: append pointer (never replace); {} last pack that ships one wins",
        req.catalog.canonical.agents_overlay()
    );
    if !req.resolved.private.is_empty() {
        println!(
            "Private:  {}  (do not commit to student-facing trees)",
            req.resolved.private.join(" ")
        );
    }
    if !req.resolved.student_safe.is_empty() {
        println!("Safe:     {}", req.resolved.student_safe.join(" "));
    }
}

pub fn confirm(req: &InstallRequest) -> Result<()> {
    if req.dry_run {
        println!();
        println!("Dry run — no files written.");
        std::process::exit(0);
    }
    if req.yes {
        return Ok(());
    }
    if !io::stdin().is_terminal() {
        return Err(Error::NeedYes);
    }
    print!("\nProceed? [y/N] ");
    io::stdout().flush()?;
    let mut ans = String::new();
    io::stdin().read_line(&mut ans)?;
    match ans.trim() {
        "y" | "Y" | "yes" | "YES" => Ok(()),
        _ => {
            println!("Aborted.");
            std::process::exit(1);
        }
    }
}

pub fn run(req: &InstallRequest) -> Result<()> {
    if is_kit_root(&req.target) {
        return Err(Error::RefuseSelfInstall);
    }

    if let Some(existing) = read_existing_harness(&req.target, &req.catalog) {
        if existing != req.resolved.harness {
            eprintln!(
                "warn: target already has harness '{existing}'; installing '{}'.",
                req.resolved.harness
            );
            eprintln!("      v1 assumes one harness per repo. Continue only if you mean to mix.");
        }
    }

    preview(req);
    confirm(req)?;

    println!();
    if req.scaffold {
        scaffold_workspace(&req.kit_root, &req.target, &req.resolved.workspace, req.force)?;
        println!();
    }

    if req.prune {
        let mut prune = req.resolved.prune.clone();
        prune.skills = derived_skill_prune(
            &req.kit_root,
            &req.resolved.library_skills,
            &req.resolved.skills,
            prune.skills,
        )?;
        prune_for_resolve(&req.target, &prune)?;
    }

    if !req.resolved.core_rules.is_empty() {
        let src = req.kit_root.join(&req.resolved.core_rules);
        if src.is_dir() {
            println!("Installing core rules");
            merge_tree(&src, &req.target.join(".agents").join("rules"))?;
            println!("  .agents/rules/ ← {}/", req.resolved.core_rules);
        }
    }

    install_library_skills(req)?;

    let mut wrote_overlay = false;
    for pkg in &req.resolved.packages {
        wrote_overlay |= install_package(&req.kit_root, &req.target, &req.catalog, &req.resolved.harness, pkg)?;
    }
    if wrote_overlay {
        ensure_agents_md_pointer(
            &req.target,
            req.catalog.canonical.agents_md(),
            req.catalog.canonical.agents_overlay(),
        )?;
    }

    println!();
    write_selected_adapters(&req.target, &req.adapters)?;

    println!();
    println!("Ensuring agent trees are gitignored...");
    ensure_agent_gitignore(&req.target)?;

    println!();
    write_install_state(&req.target, &req.catalog, &req.resolved, &req.adapters)?;

    if !req.resolved.private.is_empty() {
        println!();
        println!("Reminder: private packs were installed — keep them out of student-facing commits.");
    }

    println!();
    println!("Done. Review changes in the target (not committed).");
    println!("  cd \"{}\" && git status", req.target.display());
    Ok(())
}

fn scaffold_workspace(kit: &Path, target: &Path, rel: &str, force: bool) -> Result<()> {
    if rel.is_empty() {
        return Ok(());
    }
    let src = kit.join(rel);
    if !src.is_dir() {
        eprintln!("warn: workspace template missing: {rel}");
        return Ok(());
    }
    println!("Scaffolding workspace from {rel} ...");
    let copied = merge_tree_safe(&src, target, force)?;
    for item in copied {
        if let Some(name) = item.strip_prefix("skip ") {
            println!("  skip existing {name}");
        } else {
            println!("  scaffold {item}");
        }
    }
    Ok(())
}

fn install_package(kit: &Path, target: &Path, catalog: &Catalog, harness: &str, name: &str) -> Result<bool> {
    let rel = catalog.package_path(harness, name)?;
    let src = kit.join(&rel);
    if !src.is_dir() {
        return Err(Error::PackagePathMissing(src));
    }
    let safe = catalog.package_safe(harness, name)?;
    println!("Installing package: {harness}/{name}");
    if !safe {
        println!("  note: '{name}' is staff-local — do not commit to student-facing trees");
    }
    let mut wrote_overlay = false;
    let agents_md = src.join("AGENTS.md");
    if agents_md.is_file() {
        let overlay = catalog.canonical.agents_overlay();
        fs::copy(&agents_md, target.join(overlay))?;
        println!("  {overlay} ← {}", agents_md.display());
        wrote_overlay = true;
    }
    merge_tree(
        &src.join(".agents").join("rules"),
        &target.join(".agents").join("rules"),
    )?;
    merge_tree(
        &src.join(".agents").join("skills"),
        &target.join(".agents").join("skills"),
    )?;
    merge_tree(
        &src.join(".agents").join("agents"),
        &target.join(".agents").join("agents"),
    )?;
    let docs = src.join("docs");
    if docs.is_dir() {
        merge_tree(&docs, &target.join("docs"))?;
        println!("  docs/ ← {}/docs/", rel.display());
    }
    Ok(wrote_overlay)
}

const AGENTS_POINTER_BEGIN: &str = "<!-- BEGIN symkit harness (do not edit this block) -->";
const AGENTS_POINTER_END: &str = "<!-- END symkit harness -->";

fn agents_pointer_block(agents_md: &str, overlay: &str) -> String {
    format!(
        "{AGENTS_POINTER_BEGIN}\nRead [`{overlay}`]({overlay}) and follow it as additional always-on project rules from the installed symkit harness. Instructions in this `{agents_md}` take precedence when they conflict.\n{AGENTS_POINTER_END}\n"
    )
}

fn ensure_agents_md_pointer(target: &Path, agents_md: &str, overlay: &str) -> Result<()> {
    let path = target.join(agents_md);
    let block = agents_pointer_block(agents_md, overlay);
    if path.is_file() {
        let existing = fs::read_to_string(&path)?;
        if existing.contains(AGENTS_POINTER_BEGIN) {
            println!("  {agents_md}: harness pointer already present");
            return Ok(());
        }
        let mut out = existing;
        if !out.is_empty() && !out.ends_with('\n') {
            out.push('\n');
        }
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&block);
        fs::write(&path, out)?;
        println!("  {agents_md}: appended harness pointer → {overlay}");
    } else {
        fs::write(&path, format!("# {agents_md}\n\n{block}"))?;
        println!("  {agents_md}: wrote harness pointer → {overlay}");
    }
    Ok(())
}

fn install_library_skills(req: &InstallRequest) -> Result<()> {
    if req.resolved.skills.is_empty() {
        return Ok(());
    }
    println!("Installing skills");
    for name in &req.resolved.skills {
        let rel = req.catalog.skill_path(name);
        let src = req.kit_root.join(&rel);
        if !src.is_dir() {
            return Err(Error::UnknownSkill {
                name: name.clone(),
                path: src,
            });
        }
        merge_tree(&src, &req.target.join(".agents").join("skills").join(name))?;
        println!("  .agents/skills/{name}/ ← {}/", rel.display());
    }
    Ok(())
}

fn derived_skill_prune(kit: &Path, library_rel: &str, keep: &[String], extra: Vec<String>) -> Result<Vec<String>> {
    let mut names = extra;
    if library_rel.is_empty() {
        names.sort();
        names.dedup();
        return Ok(names);
    }
    let dir = kit.join(library_rel);
    if dir.is_dir() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            if !keep.iter().any(|k| k == &name) {
                names.push(name);
            }
        }
    }
    names.sort();
    names.dedup();
    Ok(names)
}

fn prune_for_resolve(target: &Path, prune: &PruneSpec) -> Result<()> {
    if prune.skills.is_empty() && prune.agents.is_empty() && prune.rules.is_empty() {
        return Ok(());
    }
    let mut any = false;
    for name in &prune.skills {
        any |= prune_named(target, "skills", name, any)?;
    }
    for name in &prune.agents {
        any |= prune_named(target, "agents", name, any)?;
    }
    for name in &prune.rules {
        any |= prune_named(target, "rules", name, any)?;
    }
    if any {
        println!();
    }
    Ok(())
}

fn prune_named(target: &Path, kind: &str, name: &str, header_done: bool) -> Result<bool> {
    if name.is_empty() {
        return Ok(false);
    }
    let mut hit = false;
    let vendors: &[&str] = match kind {
        "skills" => &[".agents", ".grok", ".claude", ".codex"],
        _ => &[".agents", ".grok", ".claude"],
    };
    for vendor in vendors {
        let rel = match kind {
            "skills" => format!("{vendor}/skills/{name}"),
            "agents" => format!("{vendor}/agents/{name}.md"),
            "rules" => format!("{vendor}/rules/{name}.md"),
            _ => continue,
        };
        let p = target.join(&rel);
        if p.exists() {
            if !header_done && !hit {
                println!("Pruning leftover role paths...");
            }
            if p.is_dir() {
                fs::remove_dir_all(&p)?;
            } else {
                fs::remove_file(&p)?;
            }
            println!("  prune {rel}");
            hit = true;
        }
    }
    Ok(hit)
}

fn write_install_state(target: &Path, catalog: &Catalog, resolved: &Resolve, adapters: &[String]) -> Result<()> {
    let rel = if catalog.canonical.state.is_empty() {
        ".symkit/state.yaml".to_string()
    } else {
        catalog.canonical.state.clone()
    };
    let path = target.join(&rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = format!(
        "harness: {}\nrole: {}\npackages: [{}]\nskills: [{}]\nadapters: [{}]\n",
        resolved.harness,
        resolved.role,
        resolved.packages.join(" "),
        resolved.skills.join(" "),
        adapters.join(" ")
    );
    fs::write(&path, body)?;
    println!("  state → {rel}");
    Ok(())
}

fn read_existing_harness(target: &Path, catalog: &Catalog) -> Option<String> {
    let rel = if catalog.canonical.state.is_empty() {
        ".symkit/state.yaml".to_string()
    } else {
        catalog.canonical.state.clone()
    };
    let text = fs::read_to_string(target.join(rel)).ok()?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("harness:") {
            let v = rest.trim();
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

pub fn adapt_only(target: &Path, adapters: &[String]) -> Result<()> {
    println!("Target:   {}", target.display());
    write_selected_adapters(target, adapters)?;
    ensure_agent_gitignore(target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn pointer_creates_agents_md() {
        let dir = tempdir().unwrap();
        ensure_agents_md_pointer(dir.path(), "AGENTS.md", "AGENTS-SYMKIT.md").unwrap();
        let text = fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert!(text.contains(AGENTS_POINTER_BEGIN));
        assert!(text.contains("AGENTS-SYMKIT.md"));
        assert!(text.contains("take precedence"));
    }

    #[test]
    fn pointer_appends_without_clobber() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("AGENTS.md");
        fs::write(&path, "repo-specific keep\n").unwrap();
        ensure_agents_md_pointer(dir.path(), "AGENTS.md", "AGENTS-SYMKIT.md").unwrap();
        ensure_agents_md_pointer(dir.path(), "AGENTS.md", "AGENTS-SYMKIT.md").unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.starts_with("repo-specific keep"));
        assert_eq!(text.matches(AGENTS_POINTER_BEGIN).count(), 1);
        assert!(text.contains("AGENTS-SYMKIT.md"));
    }
}
