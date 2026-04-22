# Component Invoke & Messaging Ops Contract

## Invocation contract (component@0.6.0)

`greentic:component/component@0.6.0` is the canonical component invoke boundary in this repository.

Components implement `describe()` and `invoke(op, envelope)`. `describe()` returns runtime-facing metadata about supported ops, schemas, capabilities, and optional setup. `invoke()` consumes the host-built `InvocationEnvelope` and returns a structured `InvocationResult`.

### Boundary ownership

- Components own operation behavior and descriptor metadata.
- Hosts/runtimes own `InvocationEnvelope` construction, `CallSpec` persistence, and flow orchestration.
- Flow-level mapping aliases such as `in_map`, `out_map`, and `err_map` are not component ABI fields.

### Execution and tenant context

- `InvocationEnvelope` carries tenant/session/flow/step metadata together with the call-spec bytes.
- Components should treat the envelope as host-supplied execution context rather than an authoring surface.
- `TenantCtx.i18n_id` stays part of the canonical invocation metadata so telemetry and localized output remain aligned across the call path.

## Operation dispatch

Op string dispatch remains a host-component contract. Components SHOULD treat `op` values as semantic operation identifiers, and hosts SHOULD document the corresponding payload/schema expectations using descriptor metadata.

## Error and retry semantics

`NodeError` is the shared invoke-failure record. Its fields are:

- `code`: structured error identifier.
- `message`: human-readable explanation.
- `retryable`: whether the orchestrator may retry.
- `backoff-ms`: optional retry delay hint.
- `details`: optional structured debugging payload.

Hosts should respect `retryable` and `backoff-ms` together so retries stay bounded and intentional.

## Descriptor-driven validation

- Schema IDs and inline CBOR examples live in descriptor `ops` metadata.
- Tooling can validate request/response payload compatibility before invocation using those schemas/examples.
- Optional QA/setup contracts are documented separately in `docs/component-descriptor.md`.

## Naming and mapping clarification

Flow terms such as `Flow`, `Step`, `CallSpec`, `InvocationEnvelope`, and mapping aliases belong to host/runtime orchestration. They do not change the guest component ABI unless the WIT itself changes.
