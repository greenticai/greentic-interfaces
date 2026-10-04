# Interfaces Inventory

Status of WIT worlds shipped by `greentic-interfaces`.

## Canonical (New Work)

- `component@0.6.0`, `types-core@0.6.0`, `codec@0.6.0`
- `common-types@0.1.0`, `component-v1@0.1.0`, `pack-export-v1@0.1.0`
- `operator@1.0.0` (`hook-provider`) for operation envelope + hook decision contracts
- Provider protocol: `provider-schema-core@1.0.0` + JSON Schema
- Host capability families: `secrets-store@1.0.0`, `state-store@1.1.0` (and legacy `@1.0.0`), `http-client@1.1.0`, `telemetry-logger@1.0.0`

## Compatibility (Still Shipped)

Legacy/compatibility surfaces are documented in `docs/vision/legacy.md` with replacement targets. New implementations should not start on those contracts.

## Removed Provider Families

- `events@1.0.0` and `events-bridge@1.0.0`
- `secrets-provider@0.1.0` (+ generator/audit/policy add-ons)
- legacy typed provider protocol families replaced by provider-core
