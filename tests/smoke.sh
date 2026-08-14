#!/usr/bin/env bash
# Copyright (c) 2026, PalEm Dynamics LLC
# Licensed under the Apache License, Version 2.0.

# Smoke tests for cli/symkit. Run from repo root: ./tests/smoke.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CLI="$ROOT/cli/symkit"
WORKDIR="$(mktemp -d "${TMPDIR:-/tmp}/symkit-smoke.XXXXXX")"
trap 'rm -rf "$WORKDIR"' EXIT

fail() { echo "FAIL: $*" >&2; exit 1; }
pass() { echo "ok  $*"; }

"$CLI" list | grep -q '^HARNESS' || fail "list header"
"$CLI" list | grep -q teaching || fail "list teaching"
"$CLI" show teaching | grep -q '^instructor	' || fail "show teaching roles"
"$CLI" show biosignal | grep -q 'STATUS=later' || fail "biosignal later"

# materials then instructor
T1="$WORKDIR/course"
mkdir -p "$T1"
"$CLI" install "$T1" --harness teaching --role materials --yes
[[ -f "$T1/AGENTS.md" ]] || fail "materials AGENTS.md"
[[ -f "$T1/.agents/rules/data-handling.md" ]] || fail "core data-handling"
[[ -f "$T1/.agents/skills/check-citations/SKILL.md" ]] || fail "core check-citations"
[[ -f "$T1/.agents/skills/release-materials/SKILL.md" ]] || fail "release-materials"
[[ -f "$T1/.agents/skills/write-gherkin/SKILL.md" ]] || fail "write-gherkin on materials"
[[ -d "$T1/.agents/skills/course-prep" ]] && fail "course-prep should be absent on materials"
[[ -d "$T1/.agents/skills/evaluate-content" ]] && fail "evaluate-content should be absent on materials"

"$CLI" install "$T1" --harness teaching --role instructor --yes
[[ -f "$T1/.agents/agents/instructor.md" ]] || fail "instructor agent"
[[ -d "$T1/.agents/skills/course-prep" ]] || fail "course-prep after instructor"
[[ -d "$T1/.agents/skills/accessibility-review" ]] || fail "accessibility-review after instructor"
[[ -d "$T1/.grok/skills/course-prep" ]] || fail "grok adapter course-prep"
[[ -d "$T1/.claude" ]] && fail "claude adapter should be absent by default"
[[ -f "$T1/.symkit/state.yaml" ]] || fail "install state"

# TA prunes instructor-only
"$CLI" install "$T1" --harness teaching --role ta --yes
[[ -d "$T1/.agents/skills/course-prep" ]] && fail "course-prep should be pruned on ta"
[[ -d "$T1/.agents/skills/accessibility-review" ]] && fail "accessibility-review should be pruned on ta"
[[ -f "$T1/.agents/agents/instructor.md" ]] && fail "instructor agent should be pruned on ta"
[[ -f "$T1/.agents/agents/ta.md" ]] || fail "ta agent"
[[ -d "$T1/.agents/skills/evaluate-content" ]] || fail "evaluate-content stays for ta"
[[ -f "$T1/.agents/skills/write-gherkin/SKILL.md" ]] || fail "write-gherkin stays for ta"
[[ -d "$T1/.agents/skills/release-materials" ]] && fail "release-materials should be pruned on ta"

# learner does not drop staff leftovers if we start clean
T2="$WORKDIR/learner"
mkdir -p "$T2"
"$CLI" install "$T2" --harness teaching --role learner --yes --adapters none
[[ -f "$T2/docs/ai-what-to-expect.md" ]] || fail "learner docs"
[[ -f "$T2/.agents/agents/learner.md" ]] || fail "learner agent"
[[ -d "$T2/.agents/skills/evaluate-content" ]] && fail "staff skills must not install on learner"
[[ -d "$T2/.agents/skills/write-gherkin" ]] && fail "write-gherkin must not install on learner"
[[ -f "$T2/.agents/skills/lab-tutor/SKILL.md" ]] || fail "lab-tutor on learner"
[[ -d "$T2/.grok" ]] && fail "--adapters none should skip grok"

# init scaffold, no clobber
T3="$WORKDIR/newcourse"
"$CLI" init "$T3" --harness teaching --role materials --scaffold --yes
[[ -f "$T3/assignments/README.md" ]] || fail "scaffold assignments"
[[ -f "$T3/README.md" ]] || fail "scaffold README"
echo 'keep-me' > "$T3/README.md"
"$CLI" init "$T3" --harness teaching --role materials --scaffold --yes
grep -q keep-me "$T3/README.md" || fail "scaffold must not clobber README without --force"

# research + ai
T4="$WORKDIR/study"
"$CLI" init "$T4" --harness research --role researcher --scaffold --yes
[[ -f "$T4/analysis/README.md" ]] || fail "research scaffold"
[[ -f "$T4/.agents/skills/repro-check/SKILL.md" ]] || fail "repro-check"
[[ -f "$T4/.agents/skills/check-citations/SKILL.md" ]] || fail "core check-citations on research"
[[ -f "$T4/.agents/skills/write-gherkin/SKILL.md" ]] || fail "write-gherkin on research"
[[ -f "$T4/.agents/skills/write-manuscript/SKILL.md" ]] || fail "write-manuscript on research"
[[ -d "$T4/.agents/skills/course-prep" ]] && fail "course-prep must not install on research"

T5="$WORKDIR/eval"
"$CLI" init "$T5" --harness ai --role experimenter --scaffold --yes
[[ -f "$T5/config.yaml" ]] || fail "ai config scaffold"
[[ -f "$T5/.agents/skills/eval-run/SKILL.md" ]] || fail "eval-run"

# refuse later harness
mkdir -p "$WORKDIR/nope"
if "$CLI" install "$WORKDIR/nope" --harness biosignal --yes 2>"$WORKDIR/err"; then
  fail "biosignal should refuse install"
fi
grep -q 'not installable' "$WORKDIR/err" || fail "biosignal error message"

# refuse kit root
if "$CLI" install "$ROOT" --harness teaching --role materials --yes 2>"$WORKDIR/err2"; then
  fail "should refuse kit root"
fi
grep -q 'refusing' "$WORKDIR/err2" || fail "kit-root error message"

# gitignore additive
grep -q '.agents/' "$T1/.gitignore" || fail "gitignore agents"
grep -q '.symkit/' "$T1/.gitignore" || fail "gitignore state"

# adapters all
T6="$WORKDIR/alladapt"
mkdir -p "$T6"
"$CLI" install "$T6" --harness teaching --role materials --adapters all --yes
[[ -f "$T6/CLAUDE.md" ]] || fail "CLAUDE.md pointer"
[[ -d "$T6/.claude/rules" ]] || fail "claude rules"
[[ -d "$T6/.codex/skills" ]] || fail "codex skills"

pass "all smoke checks"
echo "WORKDIR was $WORKDIR (removed)"
