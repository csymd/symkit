// Copyright (c) 2026, cSYMd
// Licensed under Apache 2.0

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
    println!("AGENTS.md: last pack that ships one wins");
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
        prune_for_resolve(&req.target, &req.resolved.prune)?;
    }

    if !req.resolved.core_rules.is_empty() {
        let src = req.kit_root.join(&req.resolved.core_rules);
        if src.is_dir() {
            println!("Installing core rules");
            merge_tree(&src, &req.target.join(".agents").join("rules"))?;
            println!("  .agents/rules/ ← {}/", req.resolved.core_rules);
        }
    }

    for pkg in &req.resolved.packages {
        install_package(&req.kit_root, &req.target, &req.catalog, &req.resolved.harness, pkg)?;
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

fn install_package(kit: &Path, target: &Path, catalog: &Catalog, harness: &str, name: &str) -> Result<()> {
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
    let agents_md = src.join("AGENTS.md");
    if agents_md.is_file() {
        fs::copy(&agents_md, target.join("AGENTS.md"))?;
        println!("  AGENTS.md ← {}", agents_md.display());
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
    Ok(())
}

fn prune_for_resolve(target: &Path, prune: &PruneSpec) -> Result<()> {
    if prune.skills.is_empty() && prune.agents.is_empty() && prune.rules.is_empty() {
        return Ok(());
    }
    println!("Pruning leftover role paths...");
    for name in &prune.skills {
        prune_named(target, "skills", name)?;
    }
    for name in &prune.agents {
        prune_named(target, "agents", name)?;
    }
    for name in &prune.rules {
        prune_named(target, "rules", name)?;
    }
    println!();
    Ok(())
}

fn prune_named(target: &Path, kind: &str, name: &str) -> Result<()> {
    if name.is_empty() {
        return Ok(());
    }
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
            if p.is_dir() {
                fs::remove_dir_all(&p)?;
            } else {
                fs::remove_file(&p)?;
            }
            println!("  prune {rel}");
        }
    }
    Ok(())
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
        "harness: {}\nrole: {}\npackages: [{}]\nadapters: [{}]\n",
        resolved.harness,
        resolved.role,
        resolved.packages.join(" "),
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
