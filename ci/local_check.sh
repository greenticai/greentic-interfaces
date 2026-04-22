#!/usr/bin/env bash
# Usage:
#   LOCAL_CHECK_ONLINE=1 LOCAL_CHECK_STRICT=1 LOCAL_CHECK_VERBOSE=1 ci/local_check.sh
# Defaults: non-strict, quiet.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"

: "${LOCAL_CHECK_ONLINE:=0}"
: "${LOCAL_CHECK_STRICT:=0}"
: "${LOCAL_CHECK_VERBOSE:=0}"
: "${LOCAL_CHECK_EXAMPLES:=0}"
: "${LOCAL_CHECK_WASM_TARGET:=wasm32-wasip2}"
: "${LOCAL_CHECK_ALLOW_DIRTY_PACKAGE:=1}"
WORKSPACE_EXCLUDES=(--exclude component-describe --exclude greentic-interfaces-crates-io-consumer)

if [[ "${LOCAL_CHECK_VERBOSE}" == "1" ]]; then
    set -x
fi

step() {
    echo ""
    echo "▶ $*"
}

need() {
    command -v "$1" >/dev/null 2>&1
}

require_tool() {
    local tool="$1"
    if [[ "${tool}" == "wasm-tools" && "${LOCAL_CHECK_ONLINE}" == "1" ]]; then
        if ! need "${tool}" && need cargo; then
            echo "[info] installing '${tool}' (LOCAL_CHECK_ONLINE=1)"
            if ! cargo_cmd install wasm-tools --locked >/dev/null 2>&1; then
                echo "[warn] failed to install '${tool}' automatically"
            fi
        fi
    fi
    if need "${tool}"; then
        return 0
    fi
    echo "[warn] missing tool '${tool}'"
    if [[ "${LOCAL_CHECK_STRICT}" == "1" ]]; then
        echo "[err] STRICT mode enabled; aborting due to missing '${tool}'"
        exit 1
    fi
    return 1
}

CARGO_BIN=()
declare -a CARGO_BIN
if [[ -n "${LOCAL_CHECK_CARGO_BIN:-}" ]]; then
    # shellcheck disable=SC2206
    CARGO_BIN=(${LOCAL_CHECK_CARGO_BIN})
elif need rustup; then
    TARGET_TOOLCHAIN="1.95.0"
    if ! rustup toolchain list 2>/dev/null | grep -q "${TARGET_TOOLCHAIN}"; then
        if [[ "${LOCAL_CHECK_ONLINE}" == "1" ]]; then
            echo "[info] installing rustup toolchain ${TARGET_TOOLCHAIN}"
            rustup toolchain install "${TARGET_TOOLCHAIN}" >/dev/null 2>&1 || true
        else
            echo "[warn] rustup toolchain ${TARGET_TOOLCHAIN} missing; set LOCAL_CHECK_ONLINE=1 to auto-install"
        fi
    fi
    if ! rustup toolchain list 2>/dev/null | grep -q "${TARGET_TOOLCHAIN}"; then
        echo "[err] rustup toolchain ${TARGET_TOOLCHAIN} not available; install it or set LOCAL_CHECK_CARGO_BIN"
        exit 1
    fi
    CARGO_BIN=("rustup" "run" "${TARGET_TOOLCHAIN}" "cargo")
fi
if [[ ${#CARGO_BIN[@]} -eq 0 ]]; then
    CARGO_BIN=("cargo")
fi
CARGO_DISPLAY="${CARGO_BIN[*]}"

cargo_cmd() {
    "${CARGO_BIN[@]}" "$@"
}

require_cargo() {
    local first="${CARGO_BIN[0]}"
    if [[ "${first}" == "rustup" ]]; then
        require_tool rustup || return 1
    fi
    require_tool cargo
}

if [[ "${CARGO_BIN[0]}" == "rustup" && "${CARGO_BIN[1]:-}" == "run" ]]; then
    CARGO_TOOLCHAIN="${CARGO_BIN[2]}"
else
    CARGO_TOOLCHAIN=""
fi

ensure_rust_target() {
    local target="$1"
    if ! need rustup; then
        echo "rustup missing; cannot verify ${target} target"
        return 1
    fi
    local list_args=()
    if [[ -n "${CARGO_TOOLCHAIN}" ]]; then
        list_args+=(--toolchain "${CARGO_TOOLCHAIN}")
    fi
    if rustup target list "${list_args[@]}" --installed | grep -q "^${target}\$"; then
        return 0
    fi
    echo "Target '${target}' is not installed. Install it with:"
    if [[ -n "${CARGO_TOOLCHAIN}" ]]; then
        echo "  rustup target add ${target} --toolchain ${CARGO_TOOLCHAIN}"
    else
        echo "  rustup target add ${target}"
    fi
    return 1
}

run_step() {
    local desc="$1"
    shift
    step "${desc}"
    if "$@"; then
        echo "[ok] ${desc}"
    else
        echo "[fail] ${desc}"
        FAILURES=1
    fi
}

skip_step() {
    local desc="$1"
    shift || true
    echo "[skip] ${desc}: $*"
}

run_or_skip() {
    local desc="$1"
    shift
    set +e
    "$@"
    local status=$?
    set -e
    if [[ ${status} -eq 0 ]]; then
        echo "[ok] ${desc}"
    else
        echo "[skip] ${desc}"
    fi
}

FAILURES=0

step "Tool versions"
run_or_skip "configured cargo (${CARGO_DISPLAY}) --version" cargo_cmd --version
run_or_skip "rustc --version" bash -c 'command -v rustc >/dev/null 2>&1 && rustc --version'
run_or_skip "wasm-tools --version" bash -c 'command -v wasm-tools >/dev/null 2>&1 && wasm-tools --version'

do_fmt() {
    cargo_cmd fmt --all -- --check
}
do_clippy() {
    cargo_cmd clippy --workspace --all-targets --all-features "${WORKSPACE_EXCLUDES[@]}" -- -D warnings
}
do_build() {
    cargo_cmd build --workspace --all-features "${WORKSPACE_EXCLUDES[@]}"
}
do_test() {
    cargo_cmd test --workspace --all-features "${WORKSPACE_EXCLUDES[@]}"
}

do_external_consumer_check() {
    EXTERNAL_CONSUMER_ALLOW_DIRTY="${LOCAL_CHECK_ALLOW_DIRTY_PACKAGE}" \
        bash ci/steps/external_consumer_check.sh
}

do_naming_lint() {
    bash scripts/naming_lint.sh
}
do_wit_lint() {
    bash scripts/wit_lint.sh
}

build_component_example() {
    CARGO_TARGET_DIR="$ROOT/target" cargo_cmd build \
        --manifest-path examples/component-describe/Cargo.toml \
        --target "${LOCAL_CHECK_WASM_TARGET}"
}

build_guest_crate() {
    CARGO_TARGET_DIR="$ROOT/target" cargo_cmd build \
        --manifest-path crates/greentic-interfaces-guest/Cargo.toml \
        --target "${LOCAL_CHECK_WASM_TARGET}"
}

run_runner_example() {
    local wasm_path="$ROOT/target/${LOCAL_CHECK_WASM_TARGET}/debug/component_describe.wasm"
    CARGO_TARGET_DIR="$ROOT/target" \
        COMPONENT_DESCRIBE_WASM="$wasm_path" \
        cargo_cmd run \
        --manifest-path examples/runner-host-smoke/Cargo.toml
}

do_wit_validate() {
    local dir="$1"
    if ! need bash || ! need wasm-tools; then
        echo "missing bash or wasm-tools"
        return 1
    fi
    WKG_FULL_STAGE=1 bash scripts/validate-wit.sh "${dir}"
}

do_wit_diff() {
    local latest_tag
    latest_tag="$(git tag --list 'v*' --sort=-version:refname | head -n1 | tr -d '\n')"
    local baseline_tag="${BASELINE_TAG:-${latest_tag}}"
    if [[ -z "${baseline_tag}" ]]; then
        echo "No baseline release tag found; skipping WIT diff."
        return 0
    fi
    local baseline_version="${baseline_tag#v}"
    local current_version
    current_version="$(sed -n 's/^version = "\(.*\)"/\1/p' crates/greentic-interfaces/Cargo.toml | head -n1)"
    if [[ -z "${current_version}" ]]; then
        current_version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n1)"
    fi
    if [[ -z "${current_version}" ]]; then
        echo "Unable to determine current version; skipping WIT diff"
        return 0
    fi
    if [[ "${current_version}" != "${baseline_version}" ]]; then
        echo "Detected version bump ${baseline_version} -> ${current_version}; skipping WIT diff guard."
        return 0
    fi
    local tmpdir
    tmpdir="$(mktemp -d)"
    local current_dir="${tmpdir}/current"
    local prev_dir="${tmpdir}/prev"
    if ! wasm-tools component wit wit --out-dir "${current_dir}" >/dev/null; then
        echo "Failed to materialize current WIT; skipping diff"
        rm -rf "${tmpdir}"
        return 0
    fi
    if ! git rev-parse -q --verify "${baseline_tag}" >/dev/null; then
        echo "Unable to resolve ${baseline_tag}; skipping diff"
        rm -rf "${tmpdir}"
        return 0
    fi
    git archive "${baseline_tag}" wit | tar -x -C "${tmpdir}"
    if ! wasm-tools component wit "${tmpdir}/wit" --out-dir "${prev_dir}" >/dev/null; then
        echo "Failed to materialize baseline WIT; skipping diff"
        rm -rf "${tmpdir}"
        return 0
    fi
    if ! diff -ru "${prev_dir}" "${current_dir}" >/dev/null; then
        echo "WIT contracts diverged relative to ${baseline_tag}. Run CI for full context."
        rm -rf "${tmpdir}"
        return 1
    fi
    rm -rf "${tmpdir}"
    return 0
}

do_wit_ownership_lint() {
    bash scripts/wit_ownership_lint.sh
}

do_no_duplicate_wit_check() {
    bash ci/check_no_duplicate_canonical_wit.sh .
}

if require_cargo; then run_step "cargo fmt" do_fmt; else skip_step "cargo fmt" "cargo missing"; fi
run_step "naming lint" do_naming_lint
if require_tool rg; then run_step "wit lint" do_wit_lint; else skip_step "wit lint" "rg missing"; fi
if require_cargo; then run_step "cargo clippy" do_clippy; else skip_step "cargo clippy" "cargo missing"; fi
if require_tool wasm-tools; then run_step "Validate ABI WIT" do_wit_validate wit; else skip_step "Validate ABI WIT" "wasm-tools missing"; fi
if require_tool wasm-tools; then run_step "Validate Wasmtime WIT" do_wit_validate crates/greentic-interfaces-wasmtime/wit; else skip_step "Validate Wasmtime WIT" "wasm-tools missing"; fi
if require_tool wasm-tools && require_tool git; then run_step "WIT diff guard" do_wit_diff; else skip_step "WIT diff guard" "missing wasm-tools or git"; fi
if require_cargo; then run_step "cargo build" do_build; else skip_step "cargo build" "cargo missing"; fi
if require_cargo; then run_step "cargo test" do_test; else skip_step "cargo test" "cargo missing"; fi
if require_cargo; then run_step "Packaged external consumer check" do_external_consumer_check; else skip_step "Packaged external consumer check" "cargo missing"; fi

if [[ "${LOCAL_CHECK_EXAMPLES}" == "1" ]]; then
    if require_cargo && ensure_rust_target "${LOCAL_CHECK_WASM_TARGET}"; then
        run_step "Build greentic-interfaces-guest (${LOCAL_CHECK_WASM_TARGET})" build_guest_crate
    else
        skip_step "Build greentic-interfaces-guest (${LOCAL_CHECK_WASM_TARGET})" "missing cargo or target"
    fi
else
    skip_step "Build greentic-interfaces-guest (${LOCAL_CHECK_WASM_TARGET})" "set LOCAL_CHECK_EXAMPLES=1 to enable"
fi

if [[ "${LOCAL_CHECK_EXAMPLES}" == "1" ]]; then
    if require_cargo && ensure_rust_target "${LOCAL_CHECK_WASM_TARGET}"; then
        run_step "Build describe-v1 example" build_component_example
    else
        skip_step "Build describe-v1 example" "missing cargo or target"
    fi
else
    skip_step "Build describe-v1 example" "set LOCAL_CHECK_EXAMPLES=1 to enable"
fi

if [[ "${LOCAL_CHECK_EXAMPLES}" == "1" ]]; then
    if require_cargo && ensure_rust_target "${LOCAL_CHECK_WASM_TARGET}"; then
        run_step "Run runner-host smoke" run_runner_example
    else
        skip_step "Run runner-host smoke" "missing cargo or target"
    fi
else
    skip_step "Run runner-host smoke" "set LOCAL_CHECK_EXAMPLES=1 to enable"
fi

if [[ "${FAILURES}" -ne 0 ]]; then
    echo "\nSome checks failed."
    exit 1
fi

echo "\nAll local checks passed."
