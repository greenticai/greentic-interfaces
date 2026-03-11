# InterfacesValPR-01 — Add `greentic:pack-validate@0.1.0` WIT (Validator-as-WASM contract)

**Repo:** `greenticai/greentic-interfaces`

## Goal
Introduce a stable WIT interface that allows pack validators to be shipped as WASM components and executed by `greentic-pack doctor` (and other tooling) without hardcoding domains.

## Non-goals
- No implementation in this PR (no Rust host, no sample validator).
- No domain logic (messaging/events/secrets) here.

## Deliverables

### A) New WIT package
Add a new WIT package and world:

- Package: `greentic:pack-validate@0.1.0`
- World: `pack-validator`

Suggested file layout:
- `wit/greentic/pack-validate@0.1.0/pack-validate.wit` (or repo convention)
- Ensure it is included in any workspace indexing/build scripts the repo uses.

### B) Interface definition (minimal, pure)
Define:
- `Diagnostic` record (maps 1:1 to `greentic-types` diagnostics)
- `PackInputs` record
- `validator` interface
- `pack-validator` world exporting `validator`

Recommended WIT (adapt to repository conventions for naming/imports):

```wit
package greentic:pack-validate@0.1.0;

record Diagnostic {
  severity: string,        // "info" | "warn" | "error"
  code: string,
  message: string,
  path: option<string>,
  hint: option<string>,
}

record PackInputs {
  manifest_cbor: list<u8>,     // raw manifest.cbor
  sbom_json: string,           // sbom.json content as string
  file_index: list<string>,    // list of paths present in the pack zip
}

interface validator {
  applies: func(inputs: PackInputs) -> bool;
  validate: func(inputs: PackInputs) -> list<Diagnostic>;
}

world pack-validator {
  export validator;
}
```

### C) Documentation
Add `docs/pack-validate.md`:
- Purpose: validators as WASM packs/components
- Security expectations: pure, no network/fs by default
- Compatibility guarantees for `@0.1.0` (no breaking field renames)

## Tests / checks
- Ensure `wit` build tooling (if any) passes.
- Ensure any schema generation / wit lint passes.

## Acceptance criteria
- `greentic:pack-validate@0.1.0` is present in the repo and can be consumed by downstream repos.
- World compiles/validates in repo CI.

