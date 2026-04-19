#![deny(unsafe_code)]
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
mod guest {
    use greentic_interfaces_guest::component_v0_6::node;

    struct Component;

    impl node::Guest for Component {
        fn describe() -> node::ComponentDescriptor {
            node::ComponentDescriptor {
                name: "guest-node-minimal".into(),
                version: "0.1.0".into(),
                summary: Some("Minimal 0.6 component example".into()),
                capabilities: vec![],
                ops: vec![],
                schemas: vec![],
                setup: None,
            }
        }

        fn invoke(
            _op: String,
            _envelope: node::InvocationEnvelope,
        ) -> Result<node::InvocationResult, node::NodeError> {
            Ok(node::InvocationResult {
                ok: true,
                output_cbor: vec![],
                output_metadata_cbor: None,
            })
        }
    }

    greentic_interfaces_guest::export_component_v060!(Component);
}
