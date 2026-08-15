// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    io::{
        self,
        IsTerminal,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
    process::ExitCode,
};

use clap::{
    Args,
    Parser,
    Subcommand,
};
use symkit::{
    adapters::resolve_adapters,
    catalog::Catalog,
    error::{
        Error,
        Result,
    },
    install::{
        self,
        InstallRequest,
    },
    kit::{
        find_kit_root,
        is_kit_root,
    },
};

#[derive(Parser)]
#[command(
    name = "symkit",
    about = "Install agent instructions, skills, and optional workspace files into another repo. Does not commit.",
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Harnesses, roles, pack summaries
    List,
    /// Role matrix and on-disk paths
    Show { harness: String },
    /// Create or activate a workspace
    Init(WorkArgs),
    /// Install packs into an existing repo
    Install(WorkArgs),
    /// Rewrite vendor adapters only
    Adapt {
        target: PathBuf,
        #[arg(long)]
        adapters: Option<String>,
        #[arg(long = "adapter", action = clap::ArgAction::Append)]
        adapter: Vec<String>,
        #[arg(long = "adapters-all")]
        adapters_all: bool,
    },
}

#[derive(Args, Default)]
struct WorkArgs {
    target: Option<PathBuf>,
    #[arg(long)]
    harness: Option<String>,
    #[arg(long)]
    role: Option<String>,
    #[arg(long = "pack", action = clap::ArgAction::Append)]
    pack: Vec<String>,
    #[arg(long = "also", action = clap::ArgAction::Append)]
    also: Vec<String>,
    #[arg(long)]
    adapters: Option<String>,
    #[arg(long = "adapter", action = clap::ArgAction::Append)]
    adapter: Vec<String>,
    #[arg(long = "adapters-all")]
    adapters_all: bool,
    #[arg(long)]
    scaffold: bool,
    #[arg(long = "no-prune")]
    no_prune: bool,
    #[arg(long = "yes", short = 'y')]
    yes: bool,
    #[arg(long)]
    force: bool,
    #[arg(long)]
    dry_run: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let kit_root = find_kit_root()?;
    let catalog = Catalog::load(kit_root.join("catalog.yaml"))?;

    match cli.command {
        None => {
            // clap already printed help if -h; bare invoke → help via clap after we re-parse
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            cmd.print_help()?;
            println!();
            Ok(())
        }
        Some(Command::List) => {
            print!("{}", catalog.format_list());
            Ok(())
        }
        Some(Command::Show { harness }) => {
            if harness.is_empty() {
                return Err(Error::ShowUsage);
            }
            print!("{}", catalog.format_show(&harness)?);
            Ok(())
        }
        Some(Command::Init(args)) => cmd_init(&kit_root, &catalog, args),
        Some(Command::Install(args)) => cmd_install(&kit_root, &catalog, args),
        Some(Command::Adapt {
            target,
            adapters,
            adapter,
            adapters_all,
        }) => {
            let target = std::fs::canonicalize(&target).map_err(|_| Error::TargetNotDir(target.clone()))?;
            let adapters = resolve_adapters(adapters.as_deref(), &adapter, adapters_all, &catalog.adapters.default)?;
            install::adapt_only(&target, &adapters)
        }
    }
}

fn cmd_install(kit_root: &Path, catalog: &Catalog, args: WorkArgs) -> Result<()> {
    let target_arg = args.target.clone().ok_or(Error::MissingTarget)?;
    let harness = args.harness.clone().ok_or(Error::MissingHarness)?;
    if !target_arg.is_dir() {
        return Err(Error::TargetNotDir(target_arg));
    }
    let target = std::fs::canonicalize(&target_arg)?;
    run_work(kit_root, catalog, args, harness, target, false)
}

fn cmd_init(kit_root: &Path, catalog: &Catalog, mut args: WorkArgs) -> Result<()> {
    if args.target.is_none() {
        if !io::stdin().is_terminal() {
            return Err(Error::MissingTarget);
        }
        print!("Target directory: ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let t = line.trim();
        if t.is_empty() {
            return Err(Error::MissingTarget);
        }
        args.target = Some(PathBuf::from(t));
    }

    if args.harness.is_none() && io::stdin().is_terminal() {
        println!("Active harnesses:");
        for (name, h) in &catalog.harnesses {
            if h.status == "active" {
                println!("  {name}");
            }
        }
        print!("Harness: ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        args.harness = Some(line.trim().to_string());
    }
    let harness = args
        .harness
        .clone()
        .filter(|s| !s.is_empty())
        .ok_or(Error::MissingHarness)?;

    if args.role.is_none() && args.pack.is_empty() && io::stdin().is_terminal() {
        println!("Roles for {harness}:");
        if let Ok(h) = catalog.harness(&harness) {
            for role in h.roles.keys() {
                println!("  {role}");
            }
        }
        print!("Role [default]: ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let r = line.trim();
        if !r.is_empty() {
            args.role = Some(r.to_string());
        }
    }

    let target_arg = args.target.clone().ok_or(Error::MissingTarget)?;
    if !args.scaffold && io::stdin().is_terminal() && !target_arg.exists() {
        print!("Create workspace scaffold? [y/N] ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        match line.trim() {
            "y" | "Y" | "yes" | "YES" => args.scaffold = true,
            _ => {}
        }
    }

    std::fs::create_dir_all(&target_arg)?;
    let target = std::fs::canonicalize(&target_arg)?;
    run_work(kit_root, catalog, args, harness, target, true)
}

fn run_work(
    kit_root: &Path,
    catalog: &Catalog,
    args: WorkArgs,
    harness: String,
    target: PathBuf,
    _is_init: bool,
) -> Result<()> {
    if is_kit_root(&target) {
        return Err(Error::RefuseSelfInstall);
    }

    let adapters = resolve_adapters(
        args.adapters.as_deref(),
        &args.adapter,
        args.adapters_all,
        &catalog.adapters.default,
    )?;

    let mut resolved = catalog.resolve(&harness, args.role.as_deref(), &args.pack)?;
    for extra in args.also {
        if !resolved.packages.iter().any(|p| p == &extra) {
            // validate
            catalog.package_path(&harness, &extra)?;
            if !catalog.package_safe(&harness, &extra)? {
                resolved.private.push(extra.clone());
            } else {
                resolved.student_safe.push(extra.clone());
            }
            resolved.packages.push(extra);
        }
    }

    let req = InstallRequest {
        kit_root: kit_root.to_path_buf(),
        target,
        catalog: catalog.clone(),
        resolved,
        adapters,
        scaffold: args.scaffold,
        force: args.force,
        prune: !args.no_prune,
        yes: args.yes,
        dry_run: args.dry_run,
    };
    install::run(&req)
}
