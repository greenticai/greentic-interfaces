# Migration Notes: Shared WIT Types Ownership

As of 2026-02-05, shared WIT types are centrally owned by `greentic:interfaces-types@0.1.0` (source of truth: `crates/greentic-interfaces/wit/greentic/interfaces-types@0.1.0/types.wit`).

## What Changed
- Shared identity types (`tenant-ctx`, `env-id`, `tenant-id`, `team-id`, `user-id`) are no longer defined in individual packages. They are imported from `greentic:interfaces-types/types@0.1.0`.
- Canonical host errors are now defined once as `host-error` in `greentic:interfaces-types` and imported where used.
- Canonical interface errors are now defined once as `iface-error` in `greentic:interfaces-types` and imported where used.

## Breaking Notes
- `greentic:worker@1.0.0` now carries its worker-specific tenant/deployment support types directly instead of importing `greentic:types-core@0.4.0`.

## 0.6 QA Mode Rename
- In `greentic:component@0.6.0` and `greentic:pack@0.6.0`, the QA enum value `upgrade` was renamed to `update`.
- This is an ABI-level WIT change and will update interface hashes/snapshots.
- If host/CLI code accepts string mode values, keep backward parsing compatibility by mapping incoming `"upgrade"` to `update`, and emit `update` in output.

## Adding New Shared Types
1. Define the shared record/enum in `crates/greentic-interfaces/wit/greentic/interfaces-types@0.1.0/types.wit`.
2. Import it in all other packages with `use greentic:interfaces-types/types@0.1.0.{...}`.
3. Run `scripts/wit_ownership_lint.sh` (or `ci/local_check.sh`) to verify ownership rules.

## Ownership Lint
The shared-type ownership lint is enforced by `scripts/wit_ownership_lint.sh` and runs in CI via `ci/local_check.sh`.
