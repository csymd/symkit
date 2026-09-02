#!/usr/bin/env bash
# Copyright (c) 2026, PalEm Dynamics LLC
# Licensed under the Apache License, Version 2.0.
#
# Stage a release archive for one rustc target triple.
#
# Usage:
#   ./scripts/package-release-binary.sh <version> <target> [out-dir]
#
# Writes <out-dir>/symkit-<version>-<target>.tar.gz containing the binary
# and LICENSE. Prints the archive path on stdout. tar.gz on every target
# (including Windows) so Git-Bash GNU tar does not have to emit zip.

set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
  echo "usage: $0 <version> <target> [out-dir]" >&2
  exit 2
fi

VERSION="$1"
TARGET="$2"
OUT="${3:-dist}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN_NAME="symkit"
EXT=""
if [[ "${TARGET}" == *windows* ]]; then
  EXT=".exe"
fi

SRC="${ROOT}/target/${TARGET}/release/${BIN_NAME}${EXT}"
if [[ ! -f "${SRC}" ]]; then
  echo "error: missing binary ${SRC}" >&2
  exit 1
fi

STAGE="$(mktemp -d "${TMPDIR:-/tmp}/symkit-pkg.XXXXXX")"
cleanup() { rm -rf "${STAGE}"; }
trap cleanup EXIT

cp "${SRC}" "${STAGE}/${BIN_NAME}${EXT}"
if [[ -z "${EXT}" ]] && command -v strip >/dev/null 2>&1; then
  strip "${STAGE}/${BIN_NAME}${EXT}" || true
fi
if [[ -z "${EXT}" ]]; then
  chmod +x "${STAGE}/${BIN_NAME}"
fi
cp "${ROOT}/LICENSE" "${STAGE}/LICENSE"

mkdir -p "${OUT}"
OUT="$(cd "${OUT}" && pwd)"

ARCHIVE="symkit-${VERSION}-${TARGET}.tar.gz"
tar -C "${STAGE}" -czf "${OUT}/${ARCHIVE}" "${BIN_NAME}${EXT}" LICENSE

if [[ ! -s "${OUT}/${ARCHIVE}" ]]; then
  echo "error: empty archive ${OUT}/${ARCHIVE}" >&2
  exit 1
fi

echo "${OUT}/${ARCHIVE}"
