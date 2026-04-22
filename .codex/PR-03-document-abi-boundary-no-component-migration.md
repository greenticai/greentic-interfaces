# PR-03 — greentic-interfaces — document component ABI vs flow-authoring mappings

## Goal

Make the ABI boundary explicit so downstream teams do **not** think existing components must change to support flow-authoring mapping aliases or terminology.

This PR is intentionally documentation-focused.

---

## Why this repo needs a PR

`greentic-interfaces` is where developers look to understand the component contract.  
If the mapping redesign is introduced elsewhere without clarifying this boundary here, people may assume:

- components now need to expose `in_map` / `out_map` / `err_map`
- invocation ABI is changing
- manifests must change

That is exactly what we want to avoid.

---

## Message this PR should establish

### The authoritative boundary

- Canonical `component@0.6.0` components still implement `describe()` / `invoke()`.
- The host/runtime owns invocation wrapping, `CallSpec` persistence, and flow orchestration.
- Mapping aliases such as `in_map`, `out_map`, and `err_map` are **flow authoring/orchestration concepts**, not new component ABI fields.

### Therefore
Current `component@0.6.0` components do **not** need to update merely because the flow authoring layer adopts new mapping terminology.

---

## Recommended changes

### 1. Root README clarification
Add a short section under `README.md` terminology or the `InvocationEnvelope` / `CallSpec` description:

> Mapping aliases such as `in_map`, `out_map`, and `err_map` are flow-level authoring/runtime concepts used to shape payloads between steps. They are not component ABI fields. Components continue to implement the current `component@0.6.0` WIT exports, while hosts/runtimes own `CallSpec` persistence and invocation wrapping.

### 2. Update an existing ABI doc instead of adding a brand-new one
Prefer one of:

- `docs/DISCOVERY_COMPONENT_INVOKE.md`
- `docs/vision/v0_6.md`

Suggested content:
- what lives in components
- what lives in flows
- what lives in runtimes/hosts
- why additive authoring aliases do not imply ABI changes

### 3. Cross-link from README
Link that doc from `docs/README.md` and from the `README.md` terminology section mentioning `Flow`, `Step`, `CallSpec`, or `InvocationEnvelope`.

---

## Optional wording to include

### Good wording
- “mapping is host-owned”
- “authoring/runtime concern”
- “no component ABI migration required”
- “existing components remain valid”

### Avoid wording that implies
- mandatory manifest updates
- required guest binding changes
- mandatory recompile for all components
- that `in_map` / `out_map` / `err_map` are canonical ABI terms defined by this repo
- that the docs are making claims about removed legacy ABI surfaces

---

## Files to update

- `README.md`
- `docs/README.md`
- one existing ABI doc: `docs/DISCOVERY_COMPONENT_INVOKE.md` or `docs/vision/v0_6.md`

---

## Acceptance criteria

- The docs clearly state that mapping aliases are not a component ABI change.
- A reader can understand that the current `component@0.6.0` ABI remains compatible with this terminology clarification.
- The docs use the repository's existing terminology (`Flow`, `Step`, `CallSpec`, `InvocationEnvelope`) rather than inventing a parallel contract vocabulary.
- The docs do not incorrectly imply the repository still ships removed legacy ABI surfaces.
- No code generation, WIT, or guest-binding changes are introduced.

---

## Non-goals

- Do not modify WIT for this PR.
- Do not add new imports/exports.
- Do not change runtime invoke envelopes.

---

## Suggested PR title

`docs: clarify that flow mapping aliases are host-owned, not component ABI`

---

## Suggested PR body

This PR clarifies the boundary between component ABI and flow orchestration.

It documents that mapping aliases such as `in_map`, `out_map`, and `err_map` are flow-level authoring/runtime concepts owned by hosts and runtimes, not new component ABI fields. The canonical `component@0.6.0` invoke boundary therefore remains unchanged.
