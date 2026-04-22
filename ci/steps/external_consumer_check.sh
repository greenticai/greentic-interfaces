#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CRATE_NAME="greentic-interfaces"
LOCAL_CHECK_ONLINE="${LOCAL_CHECK_ONLINE:-0}"

cd "${ROOT}"

declare -a CARGO_ARGS=()
if [[ "${LOCAL_CHECK_ONLINE}" != "1" ]]; then
  CARGO_ARGS+=(--offline)
fi

package_target_dir="$(mktemp -d)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${package_target_dir}" "${tmpdir}"' EXIT

echo "[external-consumer] packaging ${CRATE_NAME}"
CARGO_TARGET_DIR="${package_target_dir}" cargo package "${CARGO_ARGS[@]}" --allow-dirty -p "${CRATE_NAME}"

echo "[external-consumer] listing packaged files for ${CRATE_NAME}"
CARGO_TARGET_DIR="${package_target_dir}" cargo package "${CARGO_ARGS[@]}" --allow-dirty --list -p "${CRATE_NAME}"

crate_tar="$(ls -1t "${package_target_dir}/package/${CRATE_NAME}-"*.crate | head -n1)"
if [[ -z "${crate_tar}" ]]; then
  echo "ERROR: no packaged crate found for ${CRATE_NAME}" >&2
  exit 1
fi

crate_file="$(basename "${crate_tar}")"
crate_version="${crate_file#${CRATE_NAME}-}"
crate_version="${crate_version%.crate}"

pkg_root="${tmpdir}/pkg"
consumer_root="${tmpdir}/consumer"
mkdir -p "${pkg_root}" "${consumer_root}/src"

echo "[external-consumer] unpacking ${crate_file}"
tar -xzf "${crate_tar}" -C "${pkg_root}"

unpacked_path="${pkg_root}/${CRATE_NAME}-${crate_version}"
if [[ ! -d "${unpacked_path}" ]]; then
  echo "ERROR: unpacked crate path not found: ${unpacked_path}" >&2
  exit 1
fi

echo "[external-consumer] cargo build from unpacked crate"
cargo build "${CARGO_ARGS[@]}" --manifest-path "${unpacked_path}/Cargo.toml"

cat > "${consumer_root}/Cargo.toml" <<EOF
[package]
name = "external-consumer-check"
version = "0.1.0"
edition = "2021"

[dependencies]
${CRATE_NAME} = { path = "${unpacked_path}" }
EOF

cat > "${consumer_root}/src/main.rs" <<'EOF'
fn main() {
    let _ = greentic_interfaces::canonical::types::ErrorCode::Internal;
}
EOF

echo "[external-consumer] cargo check"
cargo check "${CARGO_ARGS[@]}" --manifest-path "${consumer_root}/Cargo.toml"

echo "[external-consumer] ok"
