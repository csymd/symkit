// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::{
    collections::BTreeMap,
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use serde::Deserialize;

use crate::error::{
    Error,
    Result,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Catalog {
    #[serde(default)]
    pub version: u32,
    #[serde(default)]
    pub kit: KitMeta,
    #[serde(default)]
    pub adapters: AdaptersConfig,
    #[serde(default)]
    pub canonical: Canonical,
    #[serde(default)]
    pub library: Library,
    #[serde(default)]
    pub core: CorePaths,
    #[serde(default)]
    pub harnesses: BTreeMap<String, Harness>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct KitMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AdaptersConfig {
    #[serde(default)]
    pub default: Vec<String>,
    #[serde(default)]
    pub all: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Canonical {
    #[serde(default = "default_state")]
    pub state: String,
}

fn default_state() -> String {
    ".symrig/state.yaml".into()
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Library {
    #[serde(default)]
    pub skills: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CorePaths {
    #[serde(default)]
    pub rules: String,
    #[serde(default)]
    pub always_skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RoleSpec {
    Packages(Vec<String>),
    Assign(RoleAssign),
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RoleAssign {
    #[serde(default)]
    pub packages: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
}

impl RoleSpec {
    pub fn packages(&self) -> &[String] {
        match self {
            RoleSpec::Packages(v) => v,
            RoleSpec::Assign(a) => &a.packages,
        }
    }

    pub fn skills(&self) -> &[String] {
        match self {
            RoleSpec::Packages(_) => &[],
            RoleSpec::Assign(a) => &a.skills,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Harness {
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub default_role: String,
    #[serde(default)]
    pub workspace: String,
    #[serde(default)]
    pub packages: BTreeMap<String, PackageMeta>,
    #[serde(default)]
    pub roles: BTreeMap<String, RoleSpec>,
    #[serde(default)]
    pub prune: BTreeMap<String, PruneSpec>,
}

fn default_status() -> String {
    "active".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackageMeta {
    #[serde(default)]
    pub description: String,
    pub path: String,
    #[serde(default)]
    pub student_safe: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PruneSpec {
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub agents: Vec<String>,
    #[serde(default)]
    pub rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Resolve {
    pub harness: String,
    pub role: String,
    pub packages: Vec<String>,
    pub skills: Vec<String>,
    pub private: Vec<String>,
    pub student_safe: Vec<String>,
    pub workspace: String,
    pub prune: PruneSpec,
    pub core_rules: String,
    pub library_skills: String,
}

impl Catalog {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(Error::CatalogMissing(path.to_path_buf()));
        }
        let text = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&text)?)
    }

    pub fn harness(&self, name: &str) -> Result<&Harness> {
        self.harnesses.get(name).ok_or_else(|| Error::UnknownHarness {
            name: name.to_string(),
            known: join_keys(&self.harnesses),
        })
    }

    pub fn resolve(&self, harness: &str, role: Option<&str>, extra_packs: &[String]) -> Result<Resolve> {
        let h = self.harness(harness)?;
        if h.status != "active" {
            return Err(Error::NotInstallable(harness.to_string(), h.status.clone()));
        }

        let (used_role, selected, role_skills) = if extra_packs.is_empty() {
            let used = role
                .filter(|r| !r.is_empty())
                .map(str::to_string)
                .filter(|r| !r.is_empty())
                .or_else(|| {
                    if h.default_role.is_empty() {
                        None
                    } else {
                        Some(h.default_role.clone())
                    }
                })
                .ok_or_else(|| Error::RoleRequired(harness.to_string()))?;
            let spec = h.roles.get(&used).ok_or_else(|| Error::UnknownRole {
                role: used.clone(),
                harness: harness.to_string(),
                known: join_keys(&h.roles),
            })?;
            (used, spec.packages().to_vec(), spec.skills().to_vec())
        } else {
            (role.unwrap_or("").to_string(), extra_packs.to_vec(), Vec::new())
        };

        let unknown: Vec<_> = selected
            .iter()
            .filter(|p| !h.packages.contains_key(p.as_str()))
            .cloned()
            .collect();
        if !unknown.is_empty() {
            return Err(Error::UnknownPackages {
                harness: harness.to_string(),
                names: unknown.join(", "),
            });
        }

        let private: Vec<String> = selected
            .iter()
            .filter(|p| !h.packages[*p].student_safe)
            .cloned()
            .collect();
        let student_safe: Vec<String> = selected
            .iter()
            .filter(|p| h.packages[*p].student_safe)
            .cloned()
            .collect();
        let prune = h.prune.get(&used_role).cloned().unwrap_or_default();
        let skills = merge_skill_names(&self.core.always_skills, &role_skills);

        Ok(Resolve {
            harness: harness.to_string(),
            role: used_role,
            packages: selected,
            skills,
            private,
            student_safe,
            workspace: h.workspace.clone(),
            prune,
            core_rules: self.core.rules.clone(),
            library_skills: self.library.skills.clone(),
        })
    }

    pub fn package_path(&self, harness: &str, pkg: &str) -> Result<PathBuf> {
        let h = self.harness(harness)?;
        let meta = h.packages.get(pkg).ok_or_else(|| Error::UnknownPackage {
            harness: harness.to_string(),
            package: pkg.to_string(),
        })?;
        Ok(PathBuf::from(&meta.path))
    }

    pub fn package_safe(&self, harness: &str, pkg: &str) -> Result<bool> {
        let h = self.harness(harness)?;
        let meta = h.packages.get(pkg).ok_or_else(|| Error::UnknownPackage {
            harness: harness.to_string(),
            package: pkg.to_string(),
        })?;
        Ok(meta.student_safe)
    }

    pub fn format_list(&self) -> String {
        let mut out = String::new();
        let name = if self.kit.name.is_empty() {
            "symrig"
        } else {
            &self.kit.name
        };
        out.push_str(&format!("KIT\t{name}\n"));
        if !self.kit.description.is_empty() {
            out.push_str(&format!("DESC\t{}\n", self.kit.description));
        }
        out.push_str(&format!("ADAPTERS_DEFAULT\t{}\n", self.adapters.default.join(" ")));
        out.push_str("HARNESS\tSTATUS\tDEFAULT_ROLE\tDESCRIPTION\n");
        for (name, h) in &self.harnesses {
            let role = if h.default_role.is_empty() {
                "-"
            } else {
                &h.default_role
            };
            out.push_str(&format!("{name}\t{}\t{role}\t{}\n", h.status, h.description));
        }
        out
    }

    pub fn format_show(&self, name: &str) -> Result<String> {
        let h = self.harness(name)?;
        let mut out = String::new();
        out.push_str(&format!("HARNESS={name}\n"));
        out.push_str(&format!("STATUS={}\n", h.status));
        out.push_str(&format!("DESCRIPTION={}\n", h.description));
        out.push_str(&format!("DEFAULT_ROLE={}\n", h.default_role));
        out.push_str(&format!("WORKSPACE={}\n", h.workspace));
        out.push_str("PACKAGE\tSAFE\tPATH\tDESCRIPTION\n");
        for (pkg, meta) in &h.packages {
            let safe = if meta.student_safe { "yes" } else { "no" };
            out.push_str(&format!("{pkg}\t{safe}\t{}\t{}\n", meta.path, meta.description));
        }
        out.push_str("ROLE\tPACKAGES\tSKILLS\n");
        for (role, spec) in &h.roles {
            out.push_str(&format!(
                "{role}\t{}\t{}\n",
                spec.packages().join(" "),
                spec.skills().join(" ")
            ));
        }
        Ok(out)
    }

    pub fn skill_path(&self, name: &str) -> PathBuf {
        PathBuf::from(&self.library.skills).join(name)
    }
}

fn merge_skill_names(always: &[String], role: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for name in always.iter().chain(role.iter()) {
        if !name.is_empty() && !out.iter().any(|s| s == name) {
            out.push(name.clone());
        }
    }
    out
}

fn join_keys<V>(map: &BTreeMap<String, V>) -> String {
    if map.is_empty() {
        "(none)".into()
    } else {
        map.keys().cloned().collect::<Vec<_>>().join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_repo() -> Catalog {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("catalog.yaml");
        Catalog::load(path).expect("repo catalog")
    }

    #[test]
    fn resolve_instructor() {
        let cat = load_repo();
        let r = cat.resolve("teaching", Some("instructor"), &[]).unwrap();
        assert_eq!(r.packages, ["shared", "staff", "instructor"]);
        assert_eq!(r.private, ["staff", "instructor"]);
        assert_eq!(r.student_safe, ["shared"]);
        assert_eq!(r.prune.agents, ["ta"]);
        assert!(r.skills.contains(&"check-citations".into()));
        assert!(r.skills.contains(&"write-gherkin".into()));
        assert!(r.skills.contains(&"course-prep".into()));
        assert_eq!(r.library_skills, "core/library/skills");
    }

    #[test]
    fn resolve_researcher_skills() {
        let cat = load_repo();
        let r = cat.resolve("research", Some("researcher"), &[]).unwrap();
        assert_eq!(r.packages, ["shared"]);
        assert!(r.skills.contains(&"write-gherkin".into()));
        assert!(r.skills.contains(&"write-manuscript".into()));
        assert!(!r.skills.contains(&"course-prep".into()));
    }

    #[test]
    fn resolve_learner_omits_staff_skills() {
        let cat = load_repo();
        let r = cat.resolve("teaching", Some("learner"), &[]).unwrap();
        assert!(r.skills.contains(&"lab-tutor".into()));
        assert!(!r.skills.contains(&"write-gherkin".into()));
        assert!(!r.skills.contains(&"evaluate-content".into()));
    }

    #[test]
    fn resolve_default_role() {
        let cat = load_repo();
        let r = cat.resolve("teaching", None, &[]).unwrap();
        assert_eq!(r.role, "instructor");
    }

    #[test]
    fn resolve_later_refused() {
        let cat = load_repo();
        let err = cat.resolve("biosignal", None, &[]).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("not installable"), "{msg}");
    }

    #[test]
    fn resolve_unknown_role() {
        let cat = load_repo();
        let err = cat.resolve("teaching", Some("dean"), &[]).unwrap_err();
        assert!(err.to_string().contains("unknown role"));
    }

    #[test]
    fn resolve_explicit_packs() {
        let cat = load_repo();
        let r = cat
            .resolve("teaching", None, &["shared".into(), "learner".into()])
            .unwrap();
        assert_eq!(r.packages, ["shared", "learner"]);
        assert!(r.role.is_empty());
        assert_eq!(r.skills, ["check-citations"]);
    }

    #[test]
    fn list_has_header() {
        let cat = load_repo();
        let s = cat.format_list();
        assert!(s.lines().any(|l| l.starts_with("HARNESS")));
        assert!(s.contains("teaching"));
    }

    #[test]
    fn show_teaching_roles() {
        let cat = load_repo();
        let s = cat.format_show("teaching").unwrap();
        assert!(s.lines().any(|l| l.starts_with("instructor\t")));
    }
}
