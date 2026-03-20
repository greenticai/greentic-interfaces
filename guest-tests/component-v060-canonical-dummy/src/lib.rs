#![deny(unsafe_code)]
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
use greentic_interfaces_guest::component_v0_6::node;

#[cfg(target_arch = "wasm32")]
struct Component;

#[cfg(target_arch = "wasm32")]
impl node::Guest for Component {
    fn describe() -> node::ComponentDescriptor {
        node::ComponentDescriptor {
            name: "component-v060-canonical-dummy".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            summary: Some("artifact metadata regression fixture".into()),
            capabilities: vec![],
            ops: vec![],
            schemas: vec![],
            setup: None,
        }
    }

    fn invoke(
        _operation: String,
        _envelope: node::InvocationEnvelope,
    ) -> Result<node::InvocationResult, node::NodeError> {
        Ok(node::InvocationResult {
            ok: true,
            output_cbor: vec![],
            output_metadata_cbor: None,
        })
    }
}

#[cfg(target_arch = "wasm32")]
greentic_interfaces_guest::export_component_v060!(Component);
