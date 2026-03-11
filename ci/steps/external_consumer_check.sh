#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CRATE_NAME="greentic-interfaces"

cd "${ROOT}"

echo "[external-consumer] packaging ${CRATE_NAME}"
cargo package --allow-dirty -p "${CRATE_NAME}"

echo "[external-consumer] listing packaged files for ${CRATE_NAME}"
cargo package --allow-dirty --list -p "${CRATE_NAME}"

crate_tar="$(ls -1t "target/package/${CRATE_NAME}-"*.crate | head -n1)"
if [[ -z "${crate_tar}" ]]; then
  echo "ERROR: no packaged crate found for ${CRATE_NAME}" >&2
  exit 1
fi

crate_file="$(basename "${crate_tar}")"
crate_version="${crate_file#${CRATE_NAME}-}"
crate_version="${crate_version%.crate}"

tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

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
cargo build --manifest-path "${unpacked_path}/Cargo.toml"

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
cargo check --manifest-path "${consumer_root}/Cargo.toml"

echo "[external-consumer] ok"
