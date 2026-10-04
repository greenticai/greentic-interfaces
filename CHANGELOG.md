# Changelog

## Unreleased
- Add `greentic:state@1.1.0` (`state-store`): identical read/write/delete plus `write-if-absent` (returns `true` when the call created the key; expired counts as absent). Adds `state-store-v1-1` features, `StateStoreHostV1_1`, `add_state_store_v1_1_to_linker` and `add_state_store_compat_to_linker` (registers `@1.1.0` and `@1.0.0` from one host impl). **Source break:** `HostFns` gained a `state_store_v1_1` field, so struct-literal users must add it (`None` keeps today's behaviour).
- WIT ABI update: renamed the 0.6 QA enum value `upgrade` to `update` in `greentic:component@0.6.0` and `greentic:pack@0.6.0`; interface hashes/snapshots changed as expected.
- Remove legacy provider-protocol WIT worlds (`greentic:messaging@1.0.0`, `greentic:events@1.0.0` broker/source/sink/bridge, `greentic:secrets-provider@0.1.0` and its generator/audit/policy deps) and drop the corresponding host/guest/wasmtime bindings; provider-core is now the only supported provider protocol.
- Add `greentic:secrets-types@1.0.0` and surface `secret-requirements` in distributor resolve responses plus a typed `get-pack-status-v2` (legacy string `get-pack-status` remains for compatibility).
- Add `greentic:secrets-store@1.0.0` read-only secret lookup world (get -> option<bytes> with structured errors) and generate host/guest bindings.
- Added Wasmtime host-linker helpers for `greentic:secrets-store@1.0.0`.
- Added full v1 host-linker helper façade (`host_helpers::v1::*`) with `add_*_to_linker` and `add_all_v1_to_linker` covering all host-import worlds.
- Add `greentic:distributor-api@1.0.0` WIT world (resolve/get/warm) with host/guest/wasmtime bindings behind the `distributor-api` feature; keep `distribution@1.0.0` published but note it as experimental/future-facing.
- Add `greentic:worker@1.0.0` generic worker ABI (WorkerRequest/Response + exec world), wire host/guest/wasmtime bindings behind `worker-api`, and document the package.
- Introduce the config-aware `greentic:component@0.5.0` ABI with a canonical `@config` record, optional `component-configurable` world exporting `get-config-schema()`, regenerated host/guest/wasmtime bindings, and new smoke tests/docs; keep `component@0.4.0` available for legacy consumers.
- Remove legacy secrets surfaces: drop `greentic:secrets@0.1.0` and `greentic:host-import@0.2.0/0.4.0/0.6.0`, strip `secrets-v1` from `greentic:host@1.0.0`, and keep only `greentic:secrets-store/store@1.0.0` (runner-host remains HTTP/KV only).

## v0.4.37
- Add `greentic:distribution@1.0.0` WIT world for generic desired-state submission/fetch (tenant + IDs + JSON blobs) and wire it through host/wasmtime bindings.
- Remove deprecated `greentic:oauth@0.1.0` and the legacy `events/emitter` world; `oauth-broker@1.0.0` and the broker/source/sink events worlds are the supported surfaces.
- Extend `TenantCtx` with `attributes` and propagate through all WIT copies, mappers, doctests, and round-trip tests.
- Refresh CI/publish guards (wit packaging/validation, local_check) to cover the new world and removed legacy packages.

## v0.4.20
- Add `greentic:component@1.0.0` describe-only world so runners can fetch schema/default metadata over JSON without binding to a specific component ABI.
- Add `greentic:host@1.0.0` with `http-v1`, `secrets-v1`, and `kv-v1` host imports to model runner capabilities that mocks/dev profiles can override.
- Add `greentic:lifecycle@1.0.0` optional exports (`init`, `health`, `shutdown`) that runners can probe and invoke when implemented.
- Add `greentic:events@1.0.0` envelope record for shared telemetry/event payloads.
- Keep the new contracts opt-in via the `describe-v1`, `runner-host-v1`, `component-lifecycle-v1`, and `events-v1` features (use the new `wit-all` helper to flip everything on) and release `greentic-interfaces-wasmtime` 0.2.3 so the Wasmtime helpers pick up the 1.0 packages.

### Migration
- Existing consumers remain unchanged. To call `describe-v1` from Wasmtime, enable the `describe-v1` feature (or `wit-all`) on `greentic-interfaces` and depend on `greentic-interfaces-wasmtime` ≥ 0.2.3 along with the bundled smoke-example pattern.

## v0.4.18
- Add optional `session-id`, `flow-id`, `node-id`, and `provider-id` fields to `TenantCtx` in the 0.4.x WIT contracts and regenerate bindings.
- Extend the Rust mappers/tests to round-trip the new identifiers while preserving backward compatibility.

## v0.4.9
- Stage generated WIT bundles inside `OUT_DIR` and expose `WIT_STAGING_DIR` so builds succeed from Cargo's read-only registry.
- Restore the deprecated `host_import_v0_2` module as a compatibility shim for `greentic-mcp` while it migrates to the Wasmtime crate.
- Reintroduce the `host_import_v0_4` Wasmtime module backed by the staged `greentic:host-import@0.4.0` package.
- Add pure Rust bindings for `component_v0_4`, `pack_export_v0_4`, and `types_core_v0_4` using the archived 0.4 WIT contracts (no Wasmtime dependency).
- Document the new staging behavior and add a crates.io consumer example.

## v0.4.0
- Refresh `greentic:types-core@0.4.0`, `greentic:host-import@0.4.0`, and `greentic:pack-export@0.4.0` to the shared WIT v0.4 contracts (flow/tenant types, secrets + telemetry + HTTP host imports, pack discovery/execution/A2A search).
- Stage WIT dependencies for Wasmtime bindgen (`build.rs` -> `target/wit-bindgen/`) and update `scripts/*wit.sh` to validate/package multi-file packages.
- Wire the new Rust bindings (`types_core_v0_4`, `host_import_v0_4`, `pack_export_v0_4`) and keep `wasix:mcp@0.0.5` byte-for-byte.

## v0.2.2
- Add `wasix:mcp@0.0.5` WIT package and reference bindings.
- Document MCP router module and bump crate to `0.2.2`.

## v0.2.1
- Document new MCP executor and update scripts/CI to validate/package it.
- Bump crate to `0.2.1`.

## v0.2.0
- Add `greentic:types-core@0.2.0` for shared tenant/flow/run structures.
- Introduce `greentic:host-import@0.2.0` for secrets, telemetry, tool, and HTTP host callbacks.
- Add `greentic:pack-export@0.2.0` describing pack execution and discovery APIs.
- Drop the unused `greentic:component@0.3.0` schema and `component_v0_3` bindings.
- Expand the Rust crate with `types_core_v0_2`, `host_import_v0_2`, and `pack_export_v0_2` modules.
- Update validation/publish scripts to cover all new WIT packages.
## 0.4.42
- Added `greentic:repo-ui-actions@1.0.0` WIT package and wired host/guest/wasmtime bindings behind a feature flag.
- Introduced a minimal repo-ui-actions end-to-end test scaffold (currently ignored pending stable component validation).
- Documented repo-ui-actions usage and bumped workspace version.
  - New generation script `scripts/build-repo-ui-actions-dummy.sh` builds a tool-generated component for the wasmtime test; test skips if the asset is missing.
