# GI-PR-02 — Add provisioning WIT contract skeleton (future reuse)

REPO: greenticai/greentic-interfaces

GOAL
Add a minimal, stable WIT contract for provisioning/setup wizards so provider extension packs can expose standardized setup flows.
(This enables `greentic-provision` to be generic across messaging/events/secrets.)

DELIVERABLES
1) New WIT package `greentic:provision@0.1.0`
2) Records:
   - ProvisionInputs (tenant ctx, provider id/install id, public_base_url?, mode, answers, existing)
   - ProvisionPlan / Patch outputs (config patch, secrets patch, webhook ops, subscription ops)
   - Diagnostic (reuse pack-validate diagnostic shape or import it)
3) World `provision-runner` (or equivalent) describing how host invokes provisioning steps.

NOTE
Keep this PR minimal: define types and placeholders, no full implementation.

ACCEPTANCE
- WIT compiles/validates; downstream can generate bindings.

