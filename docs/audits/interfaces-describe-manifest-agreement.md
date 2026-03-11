# Audit: `describe()` and Manifest-Derived Metadata Agreement

## Scope

This audit checks whether the existing `greentic:component@0.6.0` `describe()` contract is already sufficient for the planned model where:

- authoring manifests stay external during development,
- canonical manifest data may also be embedded in the Wasm artifact as deterministic CBOR,
- some manifest-derived data may be projected through `describe()`,
- passive embedded metadata and active runtime self-description must agree on overlapping meaning.

This audit is limited to `greentic-interfaces`. It does not assume any future `greentic-component` or `greentic-types` implementation beyond what the current interface and repository docs/examples imply.

## Sources Inspected

- `wit/greentic/component@0.6.0/package.wit`
- `wit/greentic/types-core@0.6.0/package.wit`
- `crates/greentic-interfaces/wit/greentic/component@0.6.0/package.wit`
- `crates/greentic-interfaces-guest/src/lib.rs`
- `crates/greentic-interfaces-guest/tests/trybuild/component_v060_export.rs`
- `crates/greentic-interfaces/src/abi/mod.rs`
- `README.md`
- `docs/component-descriptor.md`
- `docs/DISCOVERY_COMPONENT_INVOKE.md`
- `docs/interfaces_inventory.md`
- `docs/vision/v0_6.md`
- `contracts/0.6.0/CANONICAL_MAP.md`
- `CHANGELOG.md`
- `crates/greentic-interfaces/tests/fixtures/component-descriptor-example.json`
- `crates/greentic-interfaces/tests/component_descriptor_roundtrip.rs`

## Executive Answer

The current `greentic:component@0.6.0` interface is sufficient for the planned MVP model. `describe()` already works as a projection of runtime-relevant component metadata, and embedded manifest CBOR can coexist with it without a WIT change.

The main risk is not missing WIT structure. The main risk is semantic drift in surrounding docs/examples that sometimes describe a broader “descriptor/schema/runtime/qa/i18n” surface than the actual 0.6.0 WIT exports, or imply descriptor fields that the WIT does not define.

Current recommendation: do not change WIT now. Tighten conventions and docs instead.

## What `describe()` Actually Promises Today

`greentic:component@0.6.0` exports one `node` interface with:

- `describe: func() -> component-descriptor`
- `invoke: func(op: string, envelope: invocation-envelope) -> result<invocation-result, node-error>`

The returned `component-descriptor` is:

- `name: string`
- `version: string`
- `summary: option<string>`
- `capabilities: list<capability-id>`
- `ops: list<op>`
- `schemas: list<schema-ref>`
- `setup: option<setup-contract>`

Nested records returned by `describe()`:

- `op`
  - `name`
  - `summary`
  - `input: io-schema`
  - `output: io-schema`
  - `examples: list<example>`
- `io-schema`
  - `schema: schema-source`
  - `content-type: string`
  - `schema-version: option<string>`
- `schema-source`
  - `cbor-schema-id(string)`
  - `inline-cbor(list<u8>)`
  - `ref-pack-path(string)`
  - `ref-uri(string)`
- `example`
  - `title`
  - `input-cbor`
  - `output-cbor`
- `schema-ref`
  - `id`
  - `content-type`
  - `blake3-hash`
  - `version`
  - `bytes`
  - `uri`
- `setup-contract`
  - `qa-spec`
  - `answers-schema`
  - `examples: list<setup-example>`
  - `outputs: list<setup-output>`
- `setup-example`
  - `title`
  - `answers-cbor`
- `setup-output`
  - `config-only`
  - `template-scaffold(setup-template-scaffold)`
- `setup-template-scaffold`
  - `template-ref`
  - `output-layout`

### Semantic classification

- Runtime contract metadata:
  - `capabilities`
  - `ops`
  - `ops[].input`
  - `ops[].output`
  - `setup`
- Manifest-derived metadata:
  - `name`
  - `version`
  - `summary`
  - `capabilities`
  - `ops`
  - `schemas`
  - `setup`
- Packaging/artifact metadata:
  - `schemas[].blake3-hash`
  - `schemas[].bytes`
  - `schemas[].uri`
  - `schema-source.ref-pack-path`
  - `schema-source.ref-uri`
- Execution/lifecycle metadata:
  - none in `describe()` itself
  - execution data lives in `invocation-envelope` and `node-error`, not in `component-descriptor`

### Important boundary

The WIT does not promise that `describe()` is a full manifest. It promises a component descriptor shaped around:

- callable operations,
- schema references,
- capability declarations,
- optional setup contract,
- minimal identity/presentation fields.

That is a runtime-facing projection, not a full authoring or packaging manifest.

## Field Mapping Table

| `describe()` field | Current meaning | Manifest-backed today? | Can carry planned data? | WIT change needed? |
| --- | --- | --- | --- | --- |
| `name` | Human/component identity label | Yes, by convention/examples | Yes for canonical component name | No |
| `version` | Component contract/artifact version string | Yes, by convention/examples | Yes for manifest version projection | No |
| `summary` | Optional presentation text | Likely, by convention only | Yes for short manifest summary; not for long docs | No |
| `capabilities` | Runtime-declared capability requirements/affordances using `capability-id` strings | Yes, in repo docs/conventions | Yes, but only if capability vocabulary is standardized | No, convention tightening only |
| `ops` | Active callable surface exposed by the component | Yes, in docs/fixture/example code | Yes | No |
| `ops[].name` | Host-callable operation identifier | Yes, by authoring convention | Yes | No |
| `ops[].summary` | Optional short op description | Likely, by convention only | Yes | No |
| `ops[].input` / `ops[].output` | Runtime I/O contract projection | Yes | Yes | No |
| `ops[].examples` | Illustrative sample payloads for op usage | Yes in fixture/docs | Yes, if examples are intended to be shipped | No |
| `schemas` | Reusable named schema catalog associated with the component | Yes in docs/fixture | Yes | No |
| `setup` | Optional setup/QA contract exposed by the component | Yes in docs/fixture | Yes | No |

## Which Fields Are Manifest-Backed in Practice Today?

Within this repository, `describe()` is not generated from a manifest. The guest-facing API requires the component author to return a `node::ComponentDescriptor` directly, and the trybuild example constructs the record manually.

Evidence:

- `crates/greentic-interfaces-guest/src/lib.rs` shows the exported trait requiring an explicit `fn describe() -> node::ComponentDescriptor`.
- `crates/greentic-interfaces-guest/tests/trybuild/component_v060_export.rs` manually populates `name`, `version`, `capabilities`, `ops`, `schemas`, and `setup`.
- No code in this repository derives `ComponentDescriptor` from an authoring manifest or embedded descriptor blob.

So the current mapping is:

- present in interface shape: yes
- already auto-generated from manifests: no
- already assumed by docs/examples to come from authoring metadata: yes

That means manifest-backing exists today mostly as a documented convention, not an enforced interface mechanism.

## Agreement With the Planned Embedded-Manifest Model

### Clean interpretation

The current interface supports the following interpretation cleanly:

- authoring manifest: full source-of-truth document used during build/authoring
- embedded manifest CBOR: passive artifact-local copy of canonical manifest data for inspection, provenance, and reproducibility
- `describe()`: active runtime self-description surface, returning only the subset of metadata relevant to invocation/setup/schema/capability discovery

This is a coherent model and does not require a WIT change.

### Overlap rules

For overlapping data, `describe()` and embedded manifest CBOR should agree semantically on:

- component name/version
- declared capabilities
- operation names
- operation schema references
- optional setup contract

They do not need to be identical in scope. `describe()` may intentionally omit authoring-only or packaging-only fields that still exist in the embedded manifest.

## Candidate Metadata Fit: A / B / C

### A. Already representable

- Component identity: `name`, `version`
- Short human summary: `summary`
- Runtime capability declarations: `capabilities`
- Operation inventory and per-op summaries: `ops`
- Input/output schema references and inline schema bytes: `io-schema`, `schema-source`, `schema-ref`
- Setup/QA wizard contract and scaffolding hints: `setup`

### B. Representable with stricter conventions only

- Capability semantics
  - The field exists, but `capability-id` is just `string`. The interface does not define canonical ids or namespace rules.
- Version semantics
  - `version` exists, but the interface does not declare whether this is package version, artifact version, semver-only, or some other release identifier.
- Path/URI reference interpretation
  - `ref-pack-path` and `ref-uri` exist, but the exact root and packaging invariants are documented only by prose.
- Relationship between `schemas` and `ops[].input/output`
  - The interface allows both, but does not define whether every op schema must also appear in top-level `schemas`.
- Equality expectations between embedded CBOR and `describe()`
  - The interface does not specify whether hosts should require exact equality, subset equality, or semantic equality.

### C. Not representable in the current `describe()` shape

- Full authoring manifest content as such
- Build inputs, source locations, repository metadata, authorship, and provenance
- Artifact digests for the Wasm itself
- Embedded custom-section inventory or raw embedded CBOR payloads
- Distribution metadata such as OCI references or pack resolution data
- Arbitrary packaging layout or build-time-only annotations
- Host policy defaults not meant to be component runtime self-description

If any of those become required host-consumed runtime semantics, that would justify a future interface revision. Nothing in this repository shows such a requirement for the current MVP.

## Semantic Mismatches and Hidden Drift

The WIT itself is serviceable. The drift is in surrounding docs/examples.

### Confirmed doc/example mismatches

- Some docs describe `component@0.6.0` as if it exported separate descriptor/schema/runtime/qa/i18n interfaces or a `component-runtime.run` world. The actual 0.6.0 WIT exports a single `node` interface plus imported `control`.
- `docs/component-descriptor.md` and `crates/greentic-interfaces/tests/fixtures/component-descriptor-example.json` include or imply fields not present in WIT semantics:
  - fixture uses `notes` inside `examples`, but `example` only defines `title`, `input-cbor`, and `output-cbor`
  - fixture uses `null` for `output_cbor`, but WIT requires `list<u8>`
  - prose mentions `semantic_version`, unknown-field skipping, and defaulted `i18n_id`, none of which are part of `component-descriptor`
- `contracts/0.6.0/CANONICAL_MAP.md` and other prose sometimes speak of descriptor data as if it were the canonical metadata layer in general, which risks blurring “runtime projection” with “full manifest.”

These are documentation problems, not proof that the WIT is insufficient.

## Does Embedded Manifest CBOR Create Interface Ambiguity?

Not inherently.

The ambiguity appears only if documentation treats both surfaces as interchangeable. A clean rule is:

- embedded manifest CBOR = passive artifact metadata for offline inspection and build provenance
- `describe()` = active runtime self-description for hosts/runners

With that rule:

- embedded CBOR is the richer artifact-local source
- `describe()` is the narrower executable contract surface
- overlapping fields should agree semantically
- non-overlapping manifest fields should remain outside `describe()`

This interpretation is consistent with the current WIT. It is not yet stated consistently in all surrounding prose.

## Should `greentic-interfaces` Change Now?

No current MVP blocker was found that requires a WIT change.

Reasons:

- The current record already carries the runtime-relevant manifest projection the MVP appears to need.
- The missing pieces are mostly full-manifest concerns, which should remain outside `describe()`.
- The current repo does not show a host requirement for additional runtime fields that cannot be expressed in the existing descriptor.
- The strongest problems found are convention/documentation drift, not shape insufficiency.

## Agreement Statement

Yes, the current interface agrees with the planned embedded-manifest model for MVP, provided `describe()` is treated as a runtime-facing projection rather than a full manifest replacement.

Yes, `describe()` can include more manifest-derived information without changing WIT, but only where that information naturally fits existing fields such as:

- `name`
- `version`
- `summary`
- `capabilities`
- `ops`
- `schemas`
- `setup`

The following should remain outside `describe()` and live in the full authoring manifest and/or embedded manifest CBOR:

- full build/package metadata
- provenance and distribution metadata
- arbitrary authoring annotations
- raw embedded descriptor payloads
- artifact-local details with no runtime contract meaning

## Follow-Up PR Suggestions

- Tighten docs to explicitly define `describe()` as a runtime/contract projection, not a full manifest.
- Correct stale references to nonexistent 0.6.0 sub-exports or worlds.
- Bring `docs/component-descriptor.md` and the JSON fixture back into exact agreement with the WIT shape.
- Document semantic-equality expectations for overlapping fields between embedded manifest CBOR and `describe()`.
- Define capability-id and version-string conventions in prose if hosts will rely on them for policy or compatibility checks.

## Final Recommendation

Do not revise `greentic:component@0.6.0` in this PR.

The current interface is adequate for MVP alignment with external authoring manifests plus embedded canonical CBOR, as long as:

- embedded manifest data is treated as passive artifact metadata,
- `describe()` remains a narrower runtime self-description surface,
- overlapping fields are kept semantically consistent by convention,
- documentation is tightened to remove the current semantic drift.
