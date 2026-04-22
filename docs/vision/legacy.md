# Historical Compatibility Surfaces

This page tracks older ABI surfaces that existed in this repository and the modern surfaces that replaced them.

## Policy

- New work should target the canonical v0.6/v1 stack in `docs/vision/v0_6.md`.
- Component and pack-export legacy surfaces have been removed from this repository.
- Some non-canonical compatibility packages may still exist while dependents migrate.

## Legacy Surface Matrix

| Legacy surface | Status | Replacement target |
| --- | --- | --- |
| `greentic:component@0.4.0` | removed | `greentic:component@0.6.0` |
| `greentic:component@0.5.0` | removed | `greentic:component@0.6.0` |
| `greentic:pack-export@0.2.0` | removed | `greentic:pack-export-v1@0.1.0` |
| `greentic:pack-export@0.4.0` | removed | `greentic:pack-export-v1@0.1.0` |
| `greentic:types-core@0.2.0` | removed | `greentic:types-core@0.6.0` |
| `greentic:types-core@0.4.0` | removed | `greentic:types-core@0.6.0` |
| `greentic:host@1.0.0` (`runner-host`) | compatibility only | dedicated host imports (`http-client@1.1.0`, `state-store@1.0.0`, `secrets-store@1.0.0`, `telemetry-logger@1.0.0`) |
| `greentic:distributor-api@1.0.0` | compatibility only | `greentic:distributor-api@1.1.0` |
| `wasix:mcp@24.11.5` | compatibility only | `wasix:mcp@25.6.18` |
| `wasix:mcp@25.3.26` | compatibility only | `wasix:mcp@25.6.18` |

## Removed Legacy Families

- `greentic:events@1.0.0` and `greentic:events-bridge@1.0.0`
- `greentic:secrets-provider@0.1.0` (+ add-ons)
- legacy typed provider protocol families replaced by `provider-schema-core@1.0.0`
