use crate::state_store_store_v1_0::greentic::state::state_store as bindings;
use crate::state_store_store_v1_1::greentic::state::state_store as bindings_v1_1;

/// Host trait for `greentic:state/store@1.0.0`.
pub use bindings::Host as StateStoreHost;
pub use bindings::{HostError as StateStoreError, OpAck, StateKey, TenantCtx};

/// Host trait for `greentic:state/state-store@1.1.0` (adds `write-if-absent`).
pub use bindings_v1_1::Host as StateStoreHostV1_1;
pub use bindings_v1_1::{
    HostError as StateStoreErrorV1_1, OpAck as OpAckV1_1, TenantCtx as TenantCtxV1_1,
};

/// Register the state-store world on the provided linker.
pub fn add_state_store_to_linker<T>(
    linker: &mut wasmtime::component::Linker<T>,
    get: fn(&mut T) -> &mut dyn StateStoreHost,
) -> wasmtime::Result<()> {
    let mut instance = linker.instance("greentic:state/state-store@1.0.0")?;
    instance.func_wrap(
        "read",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, ctx): (bindings::StateKey, Option<bindings::TenantCtx>)| {
            let host = get(caller.data_mut());
            let result = host.read(key, ctx);
            Ok((result,))
        },
    )?;
    instance.func_wrap(
        "write",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, bytes, ctx): (
            bindings::StateKey,
            wasmtime::component::__internal::Vec<u8>,
            Option<bindings::TenantCtx>,
        )| {
            let host = get(caller.data_mut());
            let result = host.write(key, bytes, ctx);
            Ok((result,))
        },
    )?;
    instance.func_wrap(
        "delete",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, ctx): (bindings::StateKey, Option<bindings::TenantCtx>)| {
            let host = get(caller.data_mut());
            let result = host.delete(key, ctx);
            Ok((result,))
        },
    )?;
    Ok(())
}

/// Register the state-store v1.1.0 world on the provided linker.
pub fn add_state_store_v1_1_to_linker<T>(
    linker: &mut wasmtime::component::Linker<T>,
    get: fn(&mut T) -> &mut dyn StateStoreHostV1_1,
) -> wasmtime::Result<()> {
    let mut instance = linker.instance("greentic:state/state-store@1.1.0")?;
    wire_v1_1(&mut instance, get)
}

/// Registers both `@1.1.0` and the legacy `@1.0.0` state-store instances from
/// a single v1.1 host implementation. The `@1.0.0` instance delegates to the
/// same trait (read/write/delete are identical; `write-if-absent` is v1.1 only).
pub fn add_state_store_compat_to_linker<T>(
    linker: &mut wasmtime::component::Linker<T>,
    get: fn(&mut T) -> &mut dyn StateStoreHostV1_1,
) -> wasmtime::Result<()> {
    let mut inst_v1_1 = linker.instance("greentic:state/state-store@1.1.0")?;
    wire_v1_1(&mut inst_v1_1, get)?;

    let mut inst_v1_0 = linker.instance("greentic:state/state-store@1.0.0")?;
    inst_v1_0.func_wrap(
        "read",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, ctx): (bindings::StateKey, Option<bindings::TenantCtx>)| {
            let host = get(caller.data_mut());
            let result = host
                .read(key_to_v1_1(key), ctx.map(ctx_to_v1_1))
                .map_err(error_to_v1_0);
            Ok((result,))
        },
    )?;
    inst_v1_0.func_wrap(
        "write",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, bytes, ctx): (
            bindings::StateKey,
            wasmtime::component::__internal::Vec<u8>,
            Option<bindings::TenantCtx>,
        )| {
            let host = get(caller.data_mut());
            let result = host
                .write(key_to_v1_1(key), bytes, ctx.map(ctx_to_v1_1))
                .map(|_| bindings::OpAck::Ok)
                .map_err(error_to_v1_0);
            Ok((result,))
        },
    )?;
    inst_v1_0.func_wrap(
        "delete",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, ctx): (bindings::StateKey, Option<bindings::TenantCtx>)| {
            let host = get(caller.data_mut());
            let result = host
                .delete(key_to_v1_1(key), ctx.map(ctx_to_v1_1))
                .map(|_| bindings::OpAck::Ok)
                .map_err(error_to_v1_0);
            Ok((result,))
        },
    )?;
    Ok(())
}

fn wire_v1_1<T>(
    instance: &mut wasmtime::component::LinkerInstance<'_, T>,
    get: fn(&mut T) -> &mut dyn StateStoreHostV1_1,
) -> wasmtime::Result<()> {
    instance.func_wrap(
        "read",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, ctx): (bindings_v1_1::StateKey, Option<bindings_v1_1::TenantCtx>)| {
            let host = get(caller.data_mut());
            let result = host.read(key, ctx);
            Ok((result,))
        },
    )?;
    instance.func_wrap(
        "write",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, bytes, ctx): (
            bindings_v1_1::StateKey,
            wasmtime::component::__internal::Vec<u8>,
            Option<bindings_v1_1::TenantCtx>,
        )| {
            let host = get(caller.data_mut());
            let result = host.write(key, bytes, ctx);
            Ok((result,))
        },
    )?;
    instance.func_wrap(
        "delete",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, ctx): (bindings_v1_1::StateKey, Option<bindings_v1_1::TenantCtx>)| {
            let host = get(caller.data_mut());
            let result = host.delete(key, ctx);
            Ok((result,))
        },
    )?;
    instance.func_wrap(
        "write-if-absent",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key, bytes, ctx): (
            bindings_v1_1::StateKey,
            wasmtime::component::__internal::Vec<u8>,
            Option<bindings_v1_1::TenantCtx>,
        )| {
            let host = get(caller.data_mut());
            let result = host.write_if_absent(key, bytes, ctx);
            Ok((result,))
        },
    )?;
    Ok(())
}

fn error_to_v1_0(err: bindings_v1_1::HostError) -> bindings::HostError {
    bindings::HostError {
        code: err.code,
        message: err.message,
    }
}

fn key_to_v1_1(key: bindings::StateKey) -> bindings_v1_1::StateKey {
    key
}

/// The `@1.0.0` and `@1.1.0` packages generate distinct (but structurally
/// identical) `tenant-ctx` types, so the compat shim converts field by field.
fn ctx_to_v1_1(ctx: bindings::TenantCtx) -> bindings_v1_1::TenantCtx {
    bindings_v1_1::TenantCtx {
        env: ctx.env,
        tenant: ctx.tenant,
        tenant_id: ctx.tenant_id,
        team: ctx.team,
        team_id: ctx.team_id,
        user: ctx.user,
        user_id: ctx.user_id,
        trace_id: ctx.trace_id,
        i18n_id: ctx.i18n_id,
        correlation_id: ctx.correlation_id,
        attributes: ctx.attributes,
        session_id: ctx.session_id,
        flow_id: ctx.flow_id,
        node_id: ctx.node_id,
        provider_id: ctx.provider_id,
        deadline_ms: ctx.deadline_ms,
        attempt: ctx.attempt,
        idempotency_key: ctx.idempotency_key,
        impersonation: ctx.impersonation.map(|i| {
            crate::state_store_store_v1_1::greentic::interfaces_types::types::Impersonation {
                actor_id: i.actor_id,
                reason: i.reason,
            }
        }),
    }
}
