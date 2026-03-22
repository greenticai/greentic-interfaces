#![deny(unsafe_code)]
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
use greentic_interfaces_guest::component_v0_6::{component_i18n, component_qa, node};

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
impl component_qa::Guest for Component {
    fn qa_spec(mode: component_qa::QaMode) -> Vec<u8> {
        let mode = match mode {
            component_qa::QaMode::Default => "default",
            component_qa::QaMode::Setup => "setup",
            component_qa::QaMode::Update => "update",
            component_qa::QaMode::Remove => "remove",
        };
        format!("qa:{mode}").into_bytes()
    }

    fn apply_answers(
        mode: component_qa::QaMode,
        current_config: Vec<u8>,
        answers: Vec<u8>,
    ) -> Vec<u8> {
        let mode = match mode {
            component_qa::QaMode::Default => "default",
            component_qa::QaMode::Setup => "setup",
            component_qa::QaMode::Update => "update",
            component_qa::QaMode::Remove => "remove",
        };

        let mut merged = format!("mode={mode};").into_bytes();
        merged.extend_from_slice(&current_config);
        merged.push(b'|');
        merged.extend_from_slice(&answers);
        merged
    }
}

#[cfg(target_arch = "wasm32")]
impl component_i18n::Guest for Component {
    fn i18n_keys() -> Vec<String> {
        vec!["fixture.title".into(), "fixture.description".into()]
    }
}

#[cfg(target_arch = "wasm32")]
greentic_interfaces_guest::export_component_v060!(
    Component,
    component_qa: Component,
    component_i18n: Component,
);
