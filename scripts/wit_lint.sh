#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CANONICAL="${ROOT}/wit/greentic"

if [[ ! -d "${CANONICAL}" ]]; then
  any06=()
  while IFS= read -r path; do
    [[ -n "${path}" ]] || continue
    any06+=("${path}")
  done < <(find "${ROOT}/wit" -path '*@0.6.0*' -print 2>/dev/null || true)
  if [[ ${#any06[@]} -gt 0 ]]; then
    echo "[err] @0.6.0 content exists but canonical tree missing (${CANONICAL})"
    exit 1
  fi
  echo "[warn] canonical WIT tree missing (${CANONICAL}); skipping wit lint"
  exit 0
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "[err] naming lint requires ripgrep (rg)"
  exit 1
fi

FAILURES=0

fail() {
  echo "[fail] ${1}"
  FAILURES=$((FAILURES + 1))
}

# No deps snapshots under canonical tree.
while IFS= read -r deps_dir; do
  [[ -n "${deps_dir}" ]] || continue
  fail "Found deps snapshot: ${deps_dir}"
done < <(find "${CANONICAL}" -type d -name deps)

# Record ownership: "record|owner glob" (relative to canonical tree).
OWNERS=(
  "tenant-ctx|*/types-core@0.6.0/*"
  "host-error|*/types-core@0.6.0/*"
  "node-error|*/types-core@0.6.0/*"
  "capability|*/types-core@0.6.0/*"
  "capability-requirement|*/types-core@0.6.0/*"
  "invocation-envelope|*/component@0.6.0/*"
  "invocation-result|*/component@0.6.0/*"
  "component-descriptor|*/component@0.6.0/*"
  "schema-ref|*/component@0.6.0/*"
)

for entry in "${OWNERS[@]}"; do
  rec="${entry%%|*}"
  owner_glob="${entry#*|}"
  owner_path_glob="${CANONICAL}/${owner_glob}"

  defs=()
  while IFS= read -r def; do
    [[ -n "${def}" ]] || continue
    defs+=("${def}")
  done < <(rg -l --glob "*@0.6.0*" "record ${rec}" "${CANONICAL}" || true)
  if [[ ${#defs[@]} -eq 0 ]]; then
    continue
  fi

  owner_count=0
  for def in "${defs[@]}"; do
    if [[ "${def}" != ${owner_path_glob} ]]; then
      fail "record ${rec} must only exist in owner (${owner_glob}); found in ${def}"
    else
      owner_count=$((owner_count + 1))
    fi
  done

  if [[ ${owner_count} -gt 1 ]]; then
    fail "record ${rec} defined multiple times within owner scope (${owner_glob})"
  fi
done

# No stray @0.6.0 packages outside canonical tree under the canonical wit root.
# Crate-local mirrors may exist for packaging and are validated separately.
extra=()
while IFS= read -r file; do
  [[ -n "${file}" ]] || continue
  extra+=("${file}")
done < <(find "${ROOT}/wit" -path '*@0.6.0/package.wit' -print || true)
for file in "${extra[@]}"; do
  [[ -z "${file}" ]] && continue
  if [[ "${file}" != ${CANONICAL}/* ]]; then
    fail "0.6 package found outside canonical tree: ${file}"
  fi
done

if [[ ${FAILURES} -gt 0 ]]; then
  exit 1
fi

echo "[ok] wit lint passed (canonical tree only, no deps, single record owners)."
