//! `state-store@1.1.0` host wiring: both versions must link from ONE host
//! implementation, and `write-if-absent` must report creation exactly once.

use std::collections::HashMap;

use anyhow::Result;
use greentic_interfaces_wasmtime::host_helpers::v1::state_store::{
    OpAckV1_1, StateKey, StateStoreErrorV1_1, StateStoreHostV1_1, TenantCtxV1_1,
    add_state_store_compat_to_linker, add_state_store_v1_1_to_linker,
};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

fn wt<T>(result: wasmtime::Result<T>) -> Result<T> {
    result.map_err(|err| anyhow::anyhow!("{err:#}"))
}

#[derive(Default)]
struct MemoryState {
    entries: HashMap<String, Vec<u8>>,
}

impl StateStoreHostV1_1 for MemoryState {
    fn read(
        &mut self,
        key: StateKey,
        _ctx: Option<TenantCtxV1_1>,
    ) -> std::result::Result<Vec<u8>, StateStoreErrorV1_1> {
        self.entries
            .get(&key)
            .cloned()
            .ok_or_else(|| StateStoreErrorV1_1 {
                code: "not-found".into(),
                message: key,
            })
    }

    fn write(
        &mut self,
        key: StateKey,
        bytes: Vec<u8>,
        _ctx: Option<TenantCtxV1_1>,
    ) -> std::result::Result<OpAckV1_1, StateStoreErrorV1_1> {
        self.entries.insert(key, bytes);
        Ok(OpAckV1_1::Ok)
    }

    fn delete(
        &mut self,
        key: StateKey,
        _ctx: Option<TenantCtxV1_1>,
    ) -> std::result::Result<OpAckV1_1, StateStoreErrorV1_1> {
        self.entries.remove(&key);
        Ok(OpAckV1_1::Ok)
    }

    fn write_if_absent(
        &mut self,
        key: StateKey,
        bytes: Vec<u8>,
        _ctx: Option<TenantCtxV1_1>,
    ) -> std::result::Result<bool, StateStoreErrorV1_1> {
        if self.entries.contains_key(&key) {
            return Ok(false);
        }
        self.entries.insert(key, bytes);
        Ok(true)
    }
}

fn engine() -> Result<Engine> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    wt(Engine::new(&config))
}

#[test]
fn v1_1_helper_wires_linker() -> Result<()> {
    let engine = engine()?;
    let mut linker: Linker<MemoryState> = Linker::new(&engine);
    wt(add_state_store_v1_1_to_linker(&mut linker, |s| s))?;
    Ok(())
}

#[test]
fn compat_helper_registers_both_versions() -> Result<()> {
    let engine = engine()?;
    let mut linker: Linker<MemoryState> = Linker::new(&engine);
    wt(add_state_store_compat_to_linker(&mut linker, |s| s))?;
    // Registering either instance again must collide, proving both exist.
    assert!(linker.instance("greentic:state/state-store@1.1.0").is_err());
    assert!(linker.instance("greentic:state/state-store@1.0.0").is_err());
    Ok(())
}

/// Declarations (inside an instance type) for the exported `tenant-ctx` and
/// `host-error` types. Component-model import validation requires every type
/// an imported function mentions to be exported by that instance, so nested
/// composite types are exported too.
const TYPES: &str = r#"
    (type $imp0 (record (field "actor-id" string) (field "reason" (option string))))
    (export "impersonation" (type $imp (eq $imp0)))
    (type $attr0 (tuple string string))
    (export "attr" (type $attr (eq $attr0)))
    (type $attrs0 (list $attr))
    (export "attrs" (type $attrs (eq $attrs0)))
    (type $oimp0 (option $imp))
    (export "oimp" (type $oimp (eq $oimp0)))
    (type $ctx0 (record
      (field "env" string) (field "tenant" string) (field "tenant-id" string)
      (field "team" (option string)) (field "team-id" (option string))
      (field "user" (option string)) (field "user-id" (option string))
      (field "trace-id" (option string)) (field "i18n-id" (option string))
      (field "correlation-id" (option string))
      (field "attributes" $attrs)
      (field "session-id" (option string)) (field "flow-id" (option string))
      (field "node-id" (option string)) (field "provider-id" (option string))
      (field "deadline-ms" (option s64)) (field "attempt" u32)
      (field "idempotency-key" (option string))
      (field "impersonation" $oimp)))
    (export "tenant-ctx" (type $ctx (eq $ctx0)))
    (type $octx0 (option $ctx))
    (export "octx" (type $octx (eq $octx0)))
    (type $herr0 (record (field "code" string) (field "message" string)))
    (export "host-error" (type $herr (eq $herr0)))
    (type $rbool0 (result bool (error $herr)))
    (export "rbool" (type $rbool (eq $rbool0)))
    (type $ack0 (enum "ok"))
    (export "op-ack" (type $ack (eq $ack0)))
    (type $rack0 (result $ack (error $herr)))
    (export "rack" (type $rack (eq $rack0)))
"#;

/// A guest importing BOTH versions: `run` calls `@1.1.0#write-if-absent`,
/// `del` calls `@1.0.0#delete`. Both pass key "k" and no tenant context.
fn guest_component() -> String {
    format!(
        r#"(component
  (import "greentic:state/state-store@1.1.0" (instance $s11
{types}
    (export "write-if-absent"
      (func (param "key" string) (param "bytes" (list u8))
            (param "ctx" $octx)
            (result $rbool)))))
  (import "greentic:state/state-store@1.0.0" (instance $s10
{types}
    (export "delete"
      (func (param "key" string) (param "ctx" $octx)
            (result $rack)))))
  (core module $mem
    (memory (export "mem") 1)
    (global $bump (mut i32) (i32.const 8192))
    (func (export "realloc") (param i32 i32 i32 i32) (result i32)
      (local $p i32)
      (local.set $p (global.get $bump))
      (global.set $bump (i32.add (global.get $bump) (local.get 3)))
      (local.get $p)))
  (core instance $mi (instantiate $mem))
  (alias core export $mi "mem" (core memory $m))
  (alias core export $mi "realloc" (core func $r))
  (alias export $s11 "write-if-absent" (func $wia))
  (alias export $s10 "delete" (func $del))
  (core func $wia_l (canon lower (func $wia) (memory $m) (realloc $r)))
  (core func $del_l (canon lower (func $del) (memory $m) (realloc $r)))
  (core module $main
    (import "h" "mem" (memory 1))
    (import "h" "wia" (func $wia (param i32 i32)))
    (import "h" "del" (func $del (param i32 i32)))
    (data (i32.const 1024) "k")
    (data (i32.const 1032) "v")
    (func (export "run") (result i32)
      ;; args tuple at 0: key(ptr,len) bytes(ptr,len) option<ctx>=none
      (i32.store (i32.const 0) (i32.const 1024))
      (i32.store (i32.const 4) (i32.const 1))
      (i32.store (i32.const 8) (i32.const 1032))
      (i32.store (i32.const 12) (i32.const 1))
      (i32.store8 (i32.const 16) (i32.const 0))
      (call $wia (i32.const 0) (i32.const 512))
      ;; (result disc << 8) | bool payload
      (i32.or
        (i32.shl (i32.load8_u (i32.const 512)) (i32.const 8))
        (i32.load8_u (i32.const 516))))
    (func (export "del") (result i32)
      (i32.store (i32.const 0) (i32.const 1024))
      (i32.store (i32.const 4) (i32.const 1))
      (i32.store8 (i32.const 8) (i32.const 0))
      (call $del (i32.const 0) (i32.const 512))
      (i32.load8_u (i32.const 512))))
  (core instance $hi
    (export "mem" (memory $m))
    (export "wia" (func $wia_l))
    (export "del" (func $del_l)))
  (core instance $mainI (instantiate $main (with "h" (instance $hi))))
  (alias core export $mainI "run" (core func $run_c))
  (alias core export $mainI "del" (core func $del_c))
  (func (export "run") (result u32) (canon lift (core func $run_c)))
  (func (export "del") (result u32) (canon lift (core func $del_c))))"#,
        types = TYPES
    )
}

#[test]
fn write_if_absent_returns_true_then_false_across_both_versions() -> Result<()> {
    let engine = engine()?;
    let mut linker: Linker<MemoryState> = Linker::new(&engine);
    wt(add_state_store_compat_to_linker(&mut linker, |s| s))?;

    let wasm = wat::parse_str(guest_component()).map_err(|e| anyhow::anyhow!("{e}"))?;
    let component = wt(Component::new(&engine, wasm))?;
    let mut store = Store::new(&engine, MemoryState::default());
    let instance = wt(linker.instantiate(&mut store, &component))?;
    let run = wt(instance.get_typed_func::<(), (u32,)>(&mut store, "run"))?;
    let del = wt(instance.get_typed_func::<(), (u32,)>(&mut store, "del"))?;

    // disc 0 (ok) in the high byte, bool in the low byte.
    assert_eq!(
        wt(run.call(&mut store, ()))?.0,
        0x0001,
        "first call creates"
    );
    assert_eq!(
        wt(run.call(&mut store, ()))?.0,
        0x0000,
        "second call is a no-op"
    );
    assert_eq!(store.data().entries.get("k"), Some(&b"v".to_vec()));

    // The legacy @1.0.0 instance delegates to the same host.
    assert_eq!(wt(del.call(&mut store, ()))?.0, 0, "1.0.0 delete is ok");
    assert!(store.data().entries.is_empty());
    assert_eq!(
        wt(run.call(&mut store, ()))?.0,
        0x0001,
        "absent again after delete"
    );
    Ok(())
}
