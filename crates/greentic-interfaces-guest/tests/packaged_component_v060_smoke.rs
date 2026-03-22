#![cfg(feature = "component-v0-6")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn packaged_guest_crate_builds_as_a_standalone_component_consumer() {
    if !cargo_component_available() {
        eprintln!("skipping packaged guest smoke test: cargo-component missing");
        return;
    }
    if !wasm_target_available() {
        eprintln!("skipping packaged guest smoke test: wasm32-wasip2 target missing");
        return;
    }
    if !wasm_tools_available() {
        eprintln!("skipping packaged guest smoke test: wasm-tools missing");
        return;
    }
    if !tar_available() {
        eprintln!("skipping packaged guest smoke test: tar missing");
        return;
    }

    let workspace_root = workspace_root().expect("workspace root");
    let package_dir = workspace_root.join("target/package");
    let temp_root = temp_root("packaged-guest");
    let unpack_root = temp_root.join("pkg");
    let consumer_root = temp_root.join("consumer");
    let consumer_src = consumer_root.join("src");
    let target_dir = temp_root.join("target");

    fs::create_dir_all(&unpack_root).expect("create unpack root");
    fs::create_dir_all(&consumer_src).expect("create consumer src");

    run(Command::new("cargo").current_dir(&workspace_root).args([
        "package",
        "--allow-dirty",
        "--no-verify",
        "-p",
        "greentic-interfaces-guest",
    ]));

    let crate_file = latest_packaged_crate(&package_dir, "greentic-interfaces-guest")
        .expect("packaged guest crate");
    let listing = command_output(Command::new("tar").arg("-tf").arg(&crate_file));
    assert!(
        listing.contains("/wit/"),
        "packaged guest crate is missing bundled wit/**: {}",
        crate_file.display()
    );

    run(Command::new("tar")
        .arg("-xzf")
        .arg(&crate_file)
        .arg("-C")
        .arg(&unpack_root));

    let unpacked_guest = unpack_root.join(format!(
        "greentic-interfaces-guest-{}",
        env!("CARGO_PKG_VERSION")
    ));
    assert!(unpacked_guest.exists(), "unpacked guest crate missing");

    fs::write(
        consumer_root.join("Cargo.toml"),
        format!(
            r#"[package]
name = "packaged-guest-consumer"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[package.metadata.component]
package = "greentic:component"

[package.metadata.component.target]
world = "greentic:component/component@0.6.0"

[dependencies]
greentic-interfaces-guest = {{ path = "{}" , default-features = false, features = ["component-v0-6"] }}
"#,
            unpacked_guest.display()
        ),
    )
    .expect("write consumer manifest");
    fs::write(consumer_src.join("lib.rs"), consumer_source()).expect("write consumer source");

    run(Command::new("cargo")
        .current_dir(&consumer_root)
        .env("CARGO_TARGET_DIR", &target_dir)
        .args([
            "component",
            "build",
            "--target",
            "wasm32-wasip2",
            "--release",
        ]));

    let wasm_path = built_component_path(&target_dir).expect("built wasm artifact");
    let wasm = fs::read(&wasm_path).expect("read wasm artifact");
    assert!(
        contains_bytes(&wasm, b"greentic:component/node@0.6.0"),
        "expected canonical node export identity in {}",
        wasm_path.display()
    );
    assert!(
        contains_bytes(&wasm, b"greentic:component/component-qa@0.6.0"),
        "expected canonical component-qa export identity in {}",
        wasm_path.display()
    );
    assert!(
        contains_bytes(&wasm, b"greentic:component/component-i18n@0.6.0"),
        "expected canonical component-i18n export identity in {}",
        wasm_path.display()
    );
    assert!(
        !contains_bytes(&wasm, b"component-v0-v6-v0"),
        "unexpected internal world alias leaked into {}",
        wasm_path.display()
    );

    let decoded = command_output(
        Command::new("wasm-tools")
            .args(["component", "wit"])
            .arg(&wasm_path),
    );
    assert!(
        decoded.contains("export greentic:component/node@0.6.0;"),
        "expected canonical node export in decoded WIT for {}",
        wasm_path.display()
    );
    assert!(
        decoded.contains("export greentic:component/component-qa@0.6.0;"),
        "expected canonical component-qa export in decoded WIT for {}",
        wasm_path.display()
    );
    assert!(
        decoded.contains("export greentic:component/component-i18n@0.6.0;"),
        "expected canonical component-i18n export in decoded WIT for {}",
        wasm_path.display()
    );
    assert!(
        !decoded.contains("component-v0-v6-v0"),
        "unexpected internal world alias in decoded WIT for {}",
        wasm_path.display()
    );

    let _ = fs::remove_dir_all(&temp_root);
}

fn consumer_source() -> &'static str {
    r#"#![deny(unsafe_code)]
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
use greentic_interfaces_guest::component_v0_6::{component_i18n, component_qa, node};

#[cfg(target_arch = "wasm32")]
struct Component;

#[cfg(target_arch = "wasm32")]
impl node::Guest for Component {
    fn describe() -> node::ComponentDescriptor {
        node::ComponentDescriptor {
            name: "packaged-guest-consumer".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            summary: Some("packaged guest crate smoke fixture".into()),
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
        let mut payload = format!("mode={mode};").into_bytes();
        payload.extend_from_slice(&current_config);
        payload.push(b'|');
        payload.extend_from_slice(&answers);
        payload
    }
}

#[cfg(target_arch = "wasm32")]
impl component_i18n::Guest for Component {
    fn i18n_keys() -> Vec<String> {
        vec!["demo.title".into(), "demo.description".into()]
    }
}

#[cfg(target_arch = "wasm32")]
greentic_interfaces_guest::export_component_v060!(
    Component,
    component_qa: Component,
    component_i18n: Component,
);
"#
}

fn run(command: &mut Command) {
    let status = command.status().expect("run command");
    assert!(status.success(), "command failed with status {status}");
}

fn command_output(command: &mut Command) -> String {
    let output = command.output().expect("run command");
    assert!(
        output.status.success(),
        "command failed with status {}",
        output.status
    );
    String::from_utf8(output.stdout).expect("utf8 stdout")
}

fn latest_packaged_crate(package_dir: &Path, name: &str) -> Option<PathBuf> {
    let prefix = format!("{name}-");
    let suffix = ".crate";
    let mut latest = None;

    for entry in fs::read_dir(package_dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        let file_name = path.file_name()?.to_str()?;
        if !file_name.starts_with(&prefix) || !file_name.ends_with(suffix) {
            continue;
        }

        let modified = entry.metadata().ok()?.modified().ok()?;
        match latest {
            Some((best_modified, _)) if modified <= best_modified => {}
            _ => latest = Some((modified, path)),
        }
    }

    latest.map(|(_, path)| path)
}

fn built_component_path(target_dir: &Path) -> Option<PathBuf> {
    let candidates = [
        target_dir.join("wasm32-wasip1/release/packaged_guest_consumer.wasm"),
        target_dir.join("wasm32-wasip2/release/packaged_guest_consumer.wasm"),
    ];
    candidates.into_iter().find(|path| path.exists())
}

fn cargo_component_available() -> bool {
    Command::new("cargo")
        .args(["component", "--version"])
        .status()
        .is_ok_and(|status| status.success())
}

fn wasm_target_available() -> bool {
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output();
    let Ok(output) = output else {
        return false;
    };
    output.status.success() && String::from_utf8_lossy(&output.stdout).contains("wasm32-wasip2")
}

fn wasm_tools_available() -> bool {
    Command::new("wasm-tools")
        .arg("--version")
        .status()
        .is_ok_and(|status| status.success())
}

fn tar_available() -> bool {
    Command::new("tar")
        .arg("--version")
        .status()
        .is_ok_and(|status| status.success())
}

fn workspace_root() -> Option<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .map(Path::to_path_buf)
}

fn temp_root(label: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!("gi-guest-{label}-{unique}"))
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
