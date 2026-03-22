use greentic_interfaces_guest::component_v0_6::{component_i18n, component_qa, node};

struct Impl;

impl node::Guest for Impl {
    fn describe() -> node::ComponentDescriptor {
        node::ComponentDescriptor {
            name: "demo".into(),
            version: "0.1.0".into(),
            summary: None,
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

impl component_qa::Guest for Impl {
    fn qa_spec(_mode: component_qa::QaMode) -> Vec<u8> {
        vec![]
    }

    fn apply_answers(
        _mode: component_qa::QaMode,
        _current_config: Vec<u8>,
        _answers: Vec<u8>,
    ) -> Vec<u8> {
        vec![]
    }
}

impl component_i18n::Guest for Impl {
    fn i18n_keys() -> Vec<String> {
        vec!["demo.title".into()]
    }
}

greentic_interfaces_guest::export_component_v060!(
    Impl,
    component_qa: Impl,
    component_i18n: Impl,
);

fn main() {}
