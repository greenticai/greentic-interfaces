# Greentic Interfaces Workspace

This repository contains the shared ABI contracts and Wasmtime runtime helpers used across the Greentic platform.

## Terminology

- **Component**: the WASM module that implements `describe()`/`invoke()` and is referenced by descriptors.
- **Flow**: the graph definition whose runtime steps are wired together and whose metadata is stored as a `CallSpec`.
- **Step**: a flow node instance. Each step has a `step_id` and resolves to one component plus an `op`.
- **CallSpec**: the canonical `{op, payload_cbor, metadata_cbor}` that flows persist. This is everything stored in packs; the host never persists the invocation envelope.
- **InvocationEnvelope**: the host-owned wrapper of `TenantCtx`, `flow_id`, `step_id`, `component_id`, and `attempt` plus the call spec bytes. Components only see the call spec.
- **TenantCtx / i18n_id**: the tenant context carries tenant/team/user/env IDs and the mandatory `i18n_id` that threads localization and telemetry through every call.

Refer to `contracts/0.6.0/RENAMES.md` for the full naming dictionary and `contracts/0.6.0/CANONICAL_MAP.md` for the decisions driving each term.
`ci/local_check.sh` now runs `scripts/naming_lint.sh` to enforce this vocabulary in the canonical contract tree before the rest of the checks execute.

## 0.6 Core Packages

- `greentic:types-core@0.6.0` exposes the canonical identifier aliases plus the `TenantCtx` record (tenant/team/user/env IDs, required `i18n_id`, correlation/trace IDs, deadline, attempt, and idempotency key) so every host can build an invocation envelope with consistent metadata.
- `greentic:codec@0.6.0` is a lightweight helper world for encoding/decoding CBOR payloads; tooling that emits descriptor examples or call specs can import it to keep hashes stable.
- `greentic:component@0.6.0` is the CBOR-first component world: `describe()` returns a `component-descriptor` with ops, schema refs (hash + uri), optional setup, and capabilities, while `invoke()` consumes the host-built `InvocationEnvelope` and returns a structured `InvocationResult`.

[![CI](https://github.com/greenticai/greentic-interfaces/actions/workflows/ci.yml/badge.svg)](https://github.com/greenticai/greentic-interfaces/actions/workflows/ci.yml)
[![WIT Docs](https://img.shields.io/badge/docs-WIT%20packages-4c9)](https://greenticai.github.io/greentic-interfaces/)
[![MSRV](https://img.shields.io/badge/MSRV-1.91%2B-blue)](#minimum-supported-rust-version)

> Canonical runtime target: `greentic:component@0.6.0` + `greentic:types-core@0.6.0` + `greentic:codec@0.6.0`.
> Older surfaces are compatibility-only and are documented in `docs/vision/legacy.md`.

## Documentation Map

- Docs index: `docs/README.md`
- Vision docs: `docs/vision/README.md`
- Canonical v0.6 direction: `docs/vision/v0_6.md`
- Legacy compatibility matrix and replacements: `docs/vision/legacy.md`

- Host/guest reexports: `greentic-interfaces-host` and `greentic-interfaces-guest` now surface the v1 worlds plus mapper helpers: component outcomes (`ComponentOutcome`, `ComponentOutcomeStatus`) and pack/flow descriptors (`PackDescriptor`, `FlowDescriptor`) for the new pack-export-v1 ABI.

Legacy JSON component config guidance (`component@0.5.0`) is tracked in `docs/vision/legacy.md`.

### Feature flags for component@0.6.0 + v1 worlds

- `component-v0-6`: enables `greentic:component@0.6.0` (world `component-v0-v6-v0`) guest bindings.

- `common-types-v0-1`: enables `greentic:common-types@0.1.0`.
- `component-v1`: enables `greentic:component-v1@0.1.0` (component-host world) and reexports the component outcome mappers.
- `pack-export-v1`: enables `greentic:pack-export-v1@0.1.0` (pack-host world) and reexports the pack/flow descriptor mappers.
- Default and `wit-all` now include these flags; guest builds also stage the v1 WIT packages when the features are on.

- [`crates/greentic-interfaces`](crates/greentic-interfaces) exposes the WebAssembly Interface Types (WIT) packages, generated Rust bindings, and thin mappers that bridge the generated types to the richer structures in [`greentic-types`](https://github.com/greenticai/greentic-types). It is intentionally ABI-only and has no Wasmtime dependency.
- [`crates/greentic-interfaces-host`](crates/greentic-interfaces-host) curates the host-facing bindings: Wasmtime-ready WIT worlds plus the shared mappers.
- [`crates/greentic-interfaces-guest`](crates/greentic-interfaces-guest) curates the guest-facing bindings for components built against `wasm32-wasip2`, including distributor API import bindings plus `DistributorApiImports` (`distributor-api-imports`) and `DistributorApiImportsV1_1` (`distributor-api-v1-1-imports`) for resolve/get/get-v2/warm and ref-based resolution + digest fetches.
- [`crates/greentic-interfaces-wasmtime`](crates/greentic-interfaces-wasmtime) hosts the Wasmtime integration layer. It wires the Greentic host imports into a Wasmtime linker, instantiates components, and forwards calls through the ABI bindings.

> Node configuration schemas always live alongside their components. This repository only ships shared WIT contracts plus the corresponding bindings/mappers.

## Canonical WIT Policy

All shared `greentic:*` WIT packages live exclusively under:

    wit/

No other crate may define or copy these packages.
Binding generation must reference the canonical path via `build.rs`.

CI enforces this.

Provider protocols are now unified under `greentic:provider-schema-core@1.0.0`. Legacy messaging/events/secrets provider WIT worlds have been removed; migrate provider components to provider-core JSON schemas instead of typed provider-specific worlds.

These crates are published from this workspace. Downstream components that only need the ABI can depend solely on `greentic-interfaces`. Runtimes that execute packs should add `greentic-interfaces-wasmtime` and choose whether to stay on the stable Wasmtime feature path or opt into the nightly configuration. Hosts that just want re-exported bindings can depend on `greentic-interfaces-host`, while guest components can pull `greentic-interfaces-guest` for `wasm32-wasip2` builds.

```rust
// Host side: wire imports into a Wasmtime linker.
use greentic_interfaces_host::host_import::v0_6::add_to_linker;

// Guest side: call host capabilities from inside a component.
use greentic_interfaces_guest::component::node::Guest;
use greentic_interfaces_guest::secrets_store::secrets_store;
```

## Which crate should I use?

- Hosts (runner, deployer, gateways): `greentic-interfaces-host`
- Wasm components (`wasm32-wasip2`): `greentic-interfaces-guest`
- Wasmtime glue / linker helpers: `greentic-interfaces-wasmtime`
- ABI/WIT tooling and validation: `greentic-interfaces`

For debugging `wkg` resolution with fully staged dependencies, use
`scripts/wkg-build-staged.sh` (defaults to attestation@1.0.0).

### Host examples

```rust
use greentic_interfaces_host::http::http_client;
use greentic_interfaces_host::secrets::store_v1::secrets_store;
use greentic_interfaces_host::telemetry::log;
```

### Guest examples

#### Component@0.6.0 (CBOR)

```rust
use greentic_interfaces_guest::component_v0_6::{
    component_descriptor, component_i18n, component_qa, component_runtime, component_schema,
};
```

```rust
use greentic_interfaces_guest::component::node::Guest;
use greentic_interfaces_guest::secrets_store::secrets_store;
use greentic_interfaces_guest::http_client::http_client;
use greentic_interfaces_guest::telemetry_logger::logger_api;

// Distributor imports (host calls) — enable the feature in Cargo.toml:
// greentic-interfaces-guest = { version = "0.4", features = ["distributor-api-imports"] }
use greentic_interfaces_guest::distributor_api::DistributorApiImports;
use greentic_interfaces_guest::distributor_api::ResolveComponentRequest;

let api = DistributorApiImports::new();
let _resp = api.resolve_component(&ResolveComponentRequest {
    tenant_id: "tenant".into(),
    environment_id: "env".into(),
    pack_id: "pack".into(),
    component_id: "comp".into(),
    version: "1.0.0".into(),
    extra: "{}".into(),
});

// Ref-based distributor imports (host calls) — enable the feature in Cargo.toml:
// greentic-interfaces-guest = { version = "0.4", features = ["distributor-api-v1-1-imports"] }
use greentic_interfaces_guest::distributor_api_v1_1::DistributorApiImportsV1_1;

let ref_api = DistributorApiImportsV1_1::new();
let resolved = ref_api.resolve_ref("oci://registry.example/greentic/component@sha256:...");
let _artifact = ref_api.get_by_digest(&resolved.digest);
```

#### Legacy guest example (component@0.5.0 node)

Use the `component_entrypoint!` macro so the crate generates the WASM export glue (marker section, unsafe `#[export_name]` funcs) for you. Only the payload description and invoke handler are required; streaming defaults to `[Progress(0), Data(result), Done]` and lifecycle hooks default to `Ok`.

```rust
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use greentic_interfaces_guest::component::node::{InvokeResult, NodeError};
use greentic_interfaces_guest::component_entrypoint;

fn describe_payload() -> String {
    r#"{"name":"demo","version":"1.0.0"}"#.to_string()
}

fn handle_message(op: String, input: String) -> InvokeResult {
    match op.as_str() {
        "fail" => InvokeResult::Err(NodeError {
            code: "demo".into(),
            message: format!("bad input: {input}"),
            retryable: false,
            backoff_ms: None,
            details: None,
        }),
        _ => InvokeResult::Ok(format!("ok:{input}")),
    }
}

component_entrypoint!({
    manifest: describe_payload,
    invoke: handle_message,
    // Optional:
    // invoke_stream: true, // default; set false to disable
    // on_start: my_start_fn,
    // on_stop: my_stop_fn,
});
```

## Migration guide: move to host/guest crates

1. Replace direct `greentic-interfaces` imports in hosts with `greentic-interfaces-host` and switch to the curated modules (`secrets`, `state`, `http`, `telemetry`, `oauth`).
2. Replace direct bindgen usage in wasm components with `greentic-interfaces-guest`; import from the module for the capability you need (`secrets_store`, `state_store`, `http_client`, `oauth`).
3. Update your target/toolchain: guests should build with `--target wasm32-wasip2`; hosts stay native.
4. For Wasmtime wiring, depend on `greentic-interfaces-wasmtime` alongside the host crate if you need linker helpers.
5. Drop local WIT regeneration: the host/guest crates ship the generated bindings; WIT remains the source of truth here.

For local development you can override the crates.io dependency on `greentic-types` by copying `.cargo/local-patch.example.toml` to `.cargo/config.toml` and pointing it at a sibling checkout of `greentic-types`.

## Feature flags

| Feature | World(s) enabled | Published package | Notes |
| --- | --- | --- | --- |
| `secrets-store-v1` | `greentic:secrets-store/store@1.0.0` (`store`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/secrets-store@1.0.0/package.wit) | Read-only secret lookup (`get`) returning bytes with structured errors. |
| `state-store-v1` | `greentic:state/store@1.0.0` (`store`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/state-store@1.0.0/package.wit) | Tenant-scoped blob store aligned with `HostCapabilities.state`. |
| `http-client-v1` | `greentic:http/client@1.0.0` (`client`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/http-client@1.0.0/package.wit) | Preview 2 HTTP client matching `HostCapabilities.http`. |
| `http-client-v1-1` | `greentic:http/client@1.1.0` (`client`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/http-client@1.1.0/package.wit) | Adds optional `request-options` + tenant context; hosts should also expose `@1.0.0` for legacy bundles. |
| `telemetry-logger-v1` | `greentic:telemetry/logger@1.0.0` (`logger`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/telemetry-logger@1.0.0/package.wit) | Tenant-aware telemetry logger aligned with `HostCapabilities.telemetry`. |
| `worker-api` | `greentic:worker/worker@1.0.0` (`worker`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/worker@1.0.0/package.wit) | Generic worker request/response envelope; see `docs/worker.md` for details. |
| `gui-fragment` | `greentic:gui/gui-fragment@1.0.0` (`gui-fragment`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/gui@1.0.0/package.wit) | Server-rendered HTML fragments for Greentic-GUI; hosts call `render-fragment(fragment-id, ctx)` and inject the returned HTML. |
| `oauth-broker-v1` | `greentic:oauth-broker@1.0.0` (`broker`, `broker-client`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/oauth-broker@1.0.0/package.wit) | Generic OAuth broker: hosts implement the broker world; guest components import via the new `broker-client` world to build consent URLs, exchange codes, and fetch tokens. |
| `component-v0-5` | `greentic:component/component@0.5.0` (`component`, `component-configurable`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/component@0.5.0/package.wit) | Config-aware component ABI with a canonical `@config` record; optional `get-config-schema()` export for JSON Schema overrides; `component@0.4.0` remains available for legacy consumers. |
| `describe-v1` | `greentic:component@1.0.0` (`describe-v1`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/component@1.0.0/package.wit) | Describe-only schema export for packs without the full component ABI. |
| `runner-host-v1` | `greentic:host@1.0.0` (`http-v1`, `kv-v1`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/host@1.0.0/package.wit) | Legacy runner host bundle (now secrets-free; kept only for HTTP/KV). |
| `operator-hooks-v1` | `greentic:operator@1.0.0` (`hook-provider`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/operator@1.0.0/package.wit) | Operation envelope + hook decision contracts for pre/post operator interceptors. |
| `component-lifecycle-v1` | `greentic:lifecycle@1.0.0` (`lifecycle-v1`) | [`package.wit`](https://greenticai.github.io/greentic-interfaces/lifecycle@1.0.0/package.wit) | Optional lifecycle hooks (`init`, `health`, `shutdown`). |
| `source-v1` | `greentic:source/source-sync@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/source@1.0.0/package.wit) | Tenant-scoped source provider interface (list repos/branches, commit metadata, webhooks). |
| `build-v1` | `greentic:build/builder@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/build@1.0.0/package.wit) | Tenant-scoped build execution (build plan/status/log refs). |
| `scan-v1` | `greentic:scan/scanner@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/scan@1.0.0/package.wit) | Tenant-scoped scan execution (scan kind/result/SBOM refs). |
| `signing-v1` | `greentic:signing/signer@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/signing@1.0.0/package.wit) | Tenant-scoped signing/verification using signing key refs. |
| `attestation-v1` | `greentic:attestation/attester@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/attestation@1.0.0/package.wit) | Tenant-scoped attestation generation (predicate/statement refs). |
| `policy-v1` | `greentic:policy/policy-evaluator@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/policy@1.0.0/package.wit) | Tenant-scoped policy evaluation (allow/deny with reasons). |
| `metadata-v1` | `greentic:metadata/metadata-store@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/metadata@1.0.0/package.wit) | Tenant-scoped metadata upsert/query for components/versions. |
| `distributor-api` | `greentic:distributor-api/distributor-api@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/distributor@1.0.0/package.wit) | Active distributor API for runner/deployer flows: resolve-component (includes secret requirements), legacy `get-pack-status` string, structured `get-pack-status-v2` (status + secret requirements), and warm-pack; guests can also enable `distributor-api-imports` for import bindings plus a `DistributorApiImports` helper. |
| `distributor-api-v1-1` | `greentic:distributor-api/distributor-api@1.1.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/distributor@1.1.0/package.wit) | Adds ref-based resolution (`resolve-ref`) and digest fetching (`get-by-digest`) for OCI component references (tag or digest); keep `@1.0.0` for pack-id + component-id flows. |
| `distribution-v1` | `greentic:distribution/distribution@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/distribution@1.0.0/package.wit) | Experimental desired state submission/retrieval (tenant + IDs + JSON blobs), not used by current flows. |
| `oci-v1` | `greentic:oci/oci-distribution@1.0.0` | [`package.wit`](https://greenticai.github.io/greentic-interfaces/oci@1.0.0/package.wit) | Tenant-scoped OCI distribution helpers (push/get pull reference). |
| `wit-all` | Aggregates every feature above plus the legacy defaults (`component-v0-4`, `types-core-*`, etc.) | – | Handy opt-in when you just want “everything on”. |

Additional shared package: `provider:common@0.0.2` (under `wit/provider-common/world.wit`) carries messaging provider metadata, capability flags, limits, render tiers, warnings, and encoded payload helpers for provider components. Enable the `provider-common` feature to generate bindings; the package remains additive and shared across messaging providers.

### Distributor component references (v1.1)

Packs and runners that need to resolve remote components should use the ref-based distributor surface in `greentic:distributor-api@1.1.0`:

- pass a ComponentRef string (`oci://registry/repo:tag` or `oci://registry/repo@sha256:<digest>`) into `resolve-ref`.
- read the returned digest + metadata and persist the digest alongside the pack manifest.
- fetch the actual artifact with `get-by-digest` (returns bytes or a filesystem path).

Older flows that only have `pack-id` + `component-id` + `version` should keep using `resolve-component` from `@1.0.0`.

### Host crate feature gates

`greentic-interfaces-host` exposes optional features for host bindings:

- `worker-v1`: enables `greentic_interfaces_host::worker::*` for `greentic:worker@1.0.0`.
- `oauth-broker-v1`: enables `greentic_interfaces_host::oauth_broker::*` for `greentic:oauth-broker@1.0.0`; pair this with `greentic-oauth-sdk` when calling the broker from services.

Example:

```toml
greentic-interfaces-host = { version = "0.4", features = ["worker-v1", "oauth-broker-v1"] }
```

Host quickstart (workers + broker):

```rust
use greentic_interfaces_host::{mappers, worker};
use greentic_types::TenantCtx;
use serde_json::json;

// Convert the shared TenantCtx into the WIT shape expected by worker bindings.
let wit_ctx = mappers::tenant_ctx_to_wit(TenantCtx::default())?;

// Build a worker request using the host bindings.
let request = worker::exports::greentic::worker::worker_api::WorkerRequest {
    version: "1.0".into(),
    tenant: wit_ctx,
    worker_id: "example-worker".into(),
    correlation_id: None,
    session_id: None,
    thread_id: None,
    payload_json: "{}".into(),
    timestamp_utc: "2025-01-01T00:00:00Z".into(),
};

// Reverse mapping when you need to turn a WIT tenant back into greentic_types::TenantCtx:
let tenant_ctx = mappers::tenant_ctx_from_wit(request.tenant.clone())?;

// OAuth broker bindings live under:
// greentic_interfaces_host::oauth_broker::exports::greentic::oauth_broker::broker_v1

// Host-friendly worker request/response with serde_json payloads:
use greentic_interfaces_host::worker::{HostWorkerRequest, HostWorkerResponse};

let host_req = HostWorkerRequest {
    version: "1.0".into(),
    tenant: TenantCtx::default(),
    worker_id: "my-worker".into(),
    payload: json!({"input": "value"}),
    timestamp_utc: "2025-01-01T00:00:00Z".into(),
    correlation_id: None,
    session_id: None,
    thread_id: None,
};

// Convert to WIT and invoke via generated bindings (e.g., Wasmtime host):
let wit_req = greentic_interfaces_host::worker::exports::greentic::worker::worker_api::WorkerRequest::try_from(host_req)?;

// Convert responses back to host types:
let host_resp: HostWorkerResponse = wit_resp.try_into()?;
```

### MCP router WIT

All MCP protocol WIT packages live here; routers should not redefine them elsewhere. New work should target `wasix:mcp@25.06.18`; older snapshots remain only for compatibility.

| WIT package | MCP spec revision | Link |
|-------------|-------------------|------|
| `wasix:mcp@24.11.05` | 2024-11-05 (+ Greentic config/secret/output descriptors) | https://modelcontextprotocol.io/specification/2024-11-05 |
| `wasix:mcp@25.03.26` | 2025-03-26 (annotations, audio content, completions, progress; metadata carries config/secrets/output hints) | https://modelcontextprotocol.io/specification/2025-03-26 |
| `wasix:mcp@25.06.18` | 2025-06-18 (structured output, resource/resource-link, elicitation, titles/_meta, tightened auth/resource metadata) | https://modelcontextprotocol.io/specification/2025-06-18 |

## Deployment plan world

Deployment packs can import `greentic:deploy-plan@1.0.0` to read the current `DeploymentPlan` and emit status updates. The world exposes two funcs:

- `get-deployment-plan()` – returns the JSON-encoded `DeploymentPlan` built by the host/deployer for this execution.
- `emit-status(message)` – reports a free-form status line that hosts may log or display in a UI.

Hosts wire this world alongside the existing runner-host imports, so deployment flows still run as ordinary events flows with an additional channel for structured deployment context.

## Minimum Supported Rust Version

The workspace targets **Rust 1.91** or newer (required by the 2024 edition). CI pins the same stable toolchain for formatting/clippy, so make sure your local toolchain matches 1.91+ when hacking.

## Examples

Two smoke-level examples live under `examples/`:

- `component-describe`: a `no_std` component that implements `describe-v1::describe-json`.
- `runner-host-smoke`: a host-side binary that links the runner-host imports, instantiates the `component-describe` Wasm artifact, and executes `describe-json`.
  The runner repository also ships a secrets-oriented guest fixture (`component-secrets`) that exercises the `secrets-store` imports end-to-end.

### Running the examples locally

```bash
# Install the WASI Preview 2 target once (matches CI)
rustup target add wasm32-wasip2 --toolchain 1.91.0

# Compile the component to Wasm (targets wasm32-wasip2)
CARGO_TARGET_DIR=target cargo build --manifest-path examples/component-describe/Cargo.toml --target wasm32-wasip2

# Run the host smoke test (reuses the artifact above)
COMPONENT_DESCRIBE_WASM=target/wasm32-wasip2/debug/component_describe.wasm \
  CARGO_TARGET_DIR=target cargo run --manifest-path examples/runner-host-smoke/Cargo.toml
```

## What is `greentic-interfaces-wasmtime`?

The runtime crate provides Wasmtime glue for the Greentic WIT packages: an engine builder, feature-gated `add_*_to_linker` helpers, and mapper utilities that bridge between the ABI structs re-exported from `greentic-interfaces` and the richer models in `greentic-types`. It does **not** regenerate WIT on its own – everything flows through the ABI crate – so downstream consumers (runner, MCP, packs) can instantiate components and call exports without duplicating linker boilerplate.

## Provenance

Every tagged release publishes a tarball, checksum, raw `package.wit` files, and a signed provenance note that enumerates per-package hashes. Grab the [latest release notes](https://github.com/greenticai/greentic-interfaces/releases/latest/download/RELEASE_NOTES.md) to verify what you downloaded.

## Local Checks

Run the CI-equivalent checks locally with:

```bash
ci/local_check.sh
```

Toggles:
- `LOCAL_CHECK_ONLINE=1` – enable networked steps (none today, reserved for future use).
- `LOCAL_CHECK_STRICT=1` – fail immediately if required tools are missing.
- `LOCAL_CHECK_VERBOSE=1` – print every command before executing it.
- `LOCAL_CHECK_EXAMPLES=1` – build/run the example crates (requires the `wasm32-wasip2` target).
- The example steps expect `rustup target add wasm32-wasip2 --toolchain 1.91.0` to have been run first.

A `pre-push` hook is installed automatically (if absent) to run the script before pushing; remove `.git/hooks/pre-push` if you prefer to opt out.

## Fetching WIT packages from OCI

The published WIT bundles live in GitHub Container Registry under `ghcr.io/greenticai/wit`. The registry metadata served from `https://greentic.ai/.well-known/wasm-pkg/registry.json` advertises this prefix, so any `wkg` client can resolve the `greentic:*` namespace automatically.

```bash
# 1. Install the wasm packaging CLI
cargo install wkg

# 2. Point your config at the Greentic registry (writes ~/.config/wasm-pkg/config.toml)
wkg config --default-registry greentic.ai

# 3. Fetch the desired package (auto-discovers ghcr.io/greenticai/wit/<namespace>/<pkg>)
wkg get greentic:component@1.0.0 --output ./component.wasm
# or grab the raw WIT:
wkg get greentic:component@1.0.0 --output ./component.wit --format wit
```

If you prefer to edit the config file manually, add this stanza:

```toml
[namespace_registries]
greentic = "greentic.ai"
```

With that mapping in place the CLI will transparently pull from GHCR using the namespace prefix advertised by the registry metadata (`greenticai/wit/`).

Legacy secrets provider surfaces are documented for migration notes in `docs/secrets-provider.md`; provider-core JSON schemas have replaced the typed provider WIT worlds.

## Using `secrets-store-v1` from guests

The `secrets-store-v1` feature gates the `greentic:secrets-store/store@1.0.0` package. Secret requirement metadata now lives in `greentic:secrets-types@1.0.0` (key/scope/format/schema/examples); distributor responses surface `secret-requirements` using that shared type, and no secret values are returned.

Components that need to work with secrets should:

All secret requirement modeling is handled in `greentic-types`; `greentic-interfaces` only defines the WIT surface.

1. Enable `secrets-store-v1` (or `wit-all`) on the dependency.
2. Import the interface in their WIT (`use greentic:secrets-store/store@1.0.0`) or via `wit-bindgen`.
3. Call the synchronous host function surfaced by the runner:

```wit
interface secrets-store {
  /// Secret lookup failures.
  enum secrets-error { not-found, denied, invalid-key, internal }

  /// Fetch a secret by key; returns `none` when the key is missing.
  get: func(key: string) -> result<option<list<u8>>, secrets-error>;
}
```

- `get` returns `Some(bytes)` when the secret exists, `None` when absent, and a structured `secrets-error` when the host rejects or cannot service the lookup.

