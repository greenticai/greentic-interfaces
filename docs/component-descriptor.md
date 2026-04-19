# Component descriptor enhancements (0.6.0)

This repository publishes the Greentic component descriptor schema that hosts/runners use to understand a component’s capabilities, operations, and optional QA/setup contract. Starting with 0.6.0 we treat the descriptor as the canonical metadata layer for:

- **Self-describing ops:** every `op` includes its input/output schema plus CBOR example payloads.
- **Optional QA setup contract:** hosts can surface a wizard that ships with each component.
- **Migration guidance:** 0.5.x uses JSON, 0.6.0 uses CBOR payloads with the new descriptor metadata.

Refer to `crates/greentic-interfaces/tests/fixtures/component-descriptor-example.json` for a template descriptor with two ops, inline schema references, CBOR examples, and a setup contract.

## Descriptor layout

```jsonc
{
  "name": "example-component",
  "version": "0.6.0",
  "ops": [
    {
      "name": "plan",
      "summary": "render a plan",
      "input": {
        "schema": { "inline_cbor": [/* CBOR schema bytes */] },
        "content_type": "application/cbor"
      },
      "output": {
        "schema": { "cbor_schema_id": "greentic:schemas/plan-output" },
        "content_type": "application/cbor"
      },
      "examples": [
        {
          "title": "happy path",
          "input_cbor": [0xa2, 0x61, 0x66, 0x61, 0x6c, 0x73, 0x65],
          "output_cbor": [0xa0]
        }
      ]
    }
  ],
  "setup": {
    "qa_spec": { "inline_cbor": [/* CBOR spec */] },
    "answers_schema": {
      "inline_cbor": [/* schema for QA answers */]
    },
    "examples": [
      {
        "title": "default answers",
        "answers_cbor": [0xa1, 0x61, 0x69, 0x64, 0x61, 0x74, 0x61]
      }
    ],
    "outputs": [
      { "config_only": null }
    ]
  }
}
```

### `ops`

- `name`: canonical op string.
- `summary`: optional human description for documentation/guides.
- `input`/`output`: `io_schema` records that describe the schema/backing payload.
- `examples`: list of sample CBOR `input_cbor`/`output_cbor` payloads plus optional notes.

### `io_schema`

- `schema`: `schema_source` variant. Preferred path is `cbor_schema_id` for stable id lookups; inline CBOR is allowed for small schemas.
- `content_type`: typically `application/cbor`; tooling may expand this when the payload becomes mixed formats.
- `schema_version` or `semantic_version` may be added locally if you need finer-grained contract metadata.

### Schema sources (`schema_source`)

1. `cbor_schema_id(string)`: stable schema identifier understood by hosts/runners.
2. `inline_cbor(list<u8>)`: recommended for small schemas or proofs-of-concept.
3. `ref_pack_path(string)`: path relative to the component artifact root (same directory as `manifest.json`/descriptor). Pack-level references should use a separate variant when needed.
4. `ref_uri(string)`: any external URI.

CBOR is canonical for 0.6.0; `inline_json` is kept only for backwards compatibility or debugging and should be feature-gated if you ever branch to a future major version.

## Optional setup contract

Add a top-level `setup: option<setup_contract>` field in the descriptor when the component exposes a QA wizard. The shape is:

- `qa_spec`: describe the question/answer spec (CBOR or JSON).
- `answers_schema`: optional schema for the QA answers (prefer CBOR schema IDs).
- `examples`: CBOR payloads illustrating plausible answers.
- `outputs`: list of `setup_output` variants:
  - `config_only`: host should only render the component’s config UI.
  - `template_scaffold`: host should instantiate a template (includes `template_ref` and optional `output_layout`).

`ref_pack_path` values point at the component root; they remain stable regardless of how the component is bundled inside a pack. Pack-level references should be expressed via a new variant (e.g., `ref_pack_root_path`) so we do not overload the component descriptor.

## Migration notes

- `component@0.6.0` uses CBOR payloads and expects `descriptor.ops`.
- In `component@0.6.0` QA mode enums, the legacy value `upgrade` was renamed to `update`.
- `setup` is optional and ignored if a host does not understand it; unknown fields are safe to skip. Hosts default `i18n_id` when the component omits it.
