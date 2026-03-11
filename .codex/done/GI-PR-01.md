# GI-PR-01 — Add `greentic:pack-validate@0.1.0` WIT (Validator-as-WASM contract)

REPO: greenticai/greentic-interfaces

GOAL
Introduce a stable WIT interface so validators can be shipped as WASM components/packs and executed by `greentic-pack doctor` without hardcoding domains.

DELIVERABLES
1) New WIT package `greentic:pack-validate@0.1.0`
2) World `pack-validator` exporting interface `validator`
3) Records:
   - Diagnostic { severity, code, message, path?, hint? }
   - PackInputs { manifest_cbor: bytes, sbom_json: string, file_index: list<string> }
4) Methods:
   - applies(PackInputs) -> bool
   - validate(PackInputs) -> list<Diagnostic>
5) Docs: `docs/pack-validate.md` explaining purpose, stability and sandbox expectations.

ACCEPTANCE
- WIT compiles/validates in CI and is consumable by downstream crates.

