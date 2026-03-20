# Component Invoke & Messaging Ops Contract

## Invocation contract (component@0.5.0)

Note: `component@0.6.0` replaces the legacy JSON `node.invoke` flow with the canonical CBOR-first `greentic:component/component@0.6.0` world. The 0.6.0 flow also uses descriptor/schema/qa/i18n exports instead of the legacy JSON manifest.

Components implement `node.invoke(ctx, op, input)` as defined in `greentic:component@0.5.0`. `ctx` is an `exec-ctx` with a `tenant` identity, `flow-id`, and optional `node-id` providing the host with tenant scoping, correlation, and tracing metadata. `op` is a host-defined string that selects which capability the component should execute, and `input` is a JSON payload encoded as UTF-8. Every call returns an `invoke-result` (either `ok(json)` or `err(node-error)`) so the host can surface component responses or failures in a uniform way.

### Execution and tenant context

- `tenant-ctx` carries tenant identifier, optional team/user names, trace and correlation IDs, deadline (Unix milliseconds), attempt count, and optional idempotency key; hosts populate whatever subset is relevant and components may log or forward them for observability and deduplication.
- `exec-ctx.flow-id` identifies the flow that triggered the call, and `exec-ctx.node-id` is provided when the host invokes a specific flow node.
- The `json` type in this interface is just a string alias; components should deserialise it with their preferred JSON library and emit responses as JSON strings as well.

## Operation dispatch

Op string dispatch is entirely up to the host-component contract. Components SHOULD treat `op` values as versioned, semantic keywords so the host can add new operations without changing the WIT surface. The host must also document the semantics of every supported op and the expected JSON schema for requests and responses.

## Node error & retry semantics

`node-error` is the shared error record returned by `invoke` failures. Its fields are:

- `code`: a structured identifier for the class of failure.
- `message`: a human-readable description.
- `retryable`: when `true`, the host/orchestrator is invited to re-run the operation (often with exponential or bounded backoff). When `false`, the failure is terminal and the host should not retry.
- `backoff-ms` (optional): the minimum number of milliseconds the host should wait before issuing another attempt; the host may treat the absence of `backoff-ms` as a suggestion that retry is safe immediately but should still cap repeated retries.
- `details` (optional): opaque JSON that components can use to give hosts richer telemetry or guidance for debugging.

Hosts must interpret `retryable` together with `backoff-ms` so they do not overwhelm downstream services—retryable errors should honor the grace period in `backoff-ms`, while non-retryable errors should surface to operators immediately.

## Recommended op names

To keep the op vocabulary consistent across components and simplify wiring, prefer the following canonical names whenever the semantics align:

1. `ingest_http` – pull raw data from an HTTP source, validate it, and normalize headers/payload for downstream processing.
2. `render_plan` – take flow state and render a plan or UI model (cards, steps, etc.) that downstream hosts can present to users.
3. `encode` – transform in-memory or domain models into wire-encoded payloads (JSON, CBOR, etc.) without sending them yet.
4. `send_payload` – deliver an already-encoded payload to an external system (HTTP, message queue, etc.) and report its status.

These names are guidelines, not enforcement; hosts may extend the list as needed, but new names should be documented alongside the corresponding JSON schema so components and hosts stay in sync.

## Migration note: CBOR payloads (component@0.6.0)

`component@0.6.0` introduces the CBOR invoke boundary: `invoke(ctx, op, input)` now receives/returns `list<u8>` payloads, while the rest of the context (session, tenant, etc.) remains structured. Hosts that support both versions should use the descriptor metadata (see `docs/component-descriptor.md`) to discover whether the component exposes 0.5 or 0.6 wiring.

- Hosts should keep `tenant-ctx` and `exec-ctx` in sync with the new `i18n_id` field so every phase of the invocation pipeline can format telemetry or localized output consistently.
- Schema IDs + inline CBOR examples live in the descriptor ops metadata; tooling that validates `input/output` payloads can use those bytes to verify encoder/decoder compatibility before invoking the component.
- The new descriptor `setup` contract (also described in `docs/component-descriptor.md`) allows components to ship QA specs and wizard answers, with `ref_pack_path` values relative to the component root.
- In 0.6 QA mode enums, `upgrade` was renamed to `update`; hosts parsing string mode values should accept legacy `"upgrade"` as an alias and normalize to `update`.

Document both your current JSON schema and the new CBOR schema to make the migration from `component@0.5.0` to `component@0.6.0` smooth for hosts and guests.
