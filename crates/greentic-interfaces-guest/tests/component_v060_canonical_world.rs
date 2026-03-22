#![cfg(feature = "component-v0-6")]

use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn component_v060_artifact_uses_canonical_world_identity() {
    if !cargo_component_available() {
        eprintln!("skipping canonical world artifact test: cargo-component missing");
        return;
    }
    if !wasm_target_available() {
        eprintln!("skipping canonical world artifact test: wasm32-wasip2 target missing");
        return;
    }

    let workspace_root = workspace_root().expect("workspace root");
    let status = Command::new("cargo")
        .current_dir(&workspace_root)
        .args([
            "component",
            "build",
            "--package",
            "component-v060-canonical-dummy",
            "--target",
            "wasm32-wasip2",
            "--release",
        ])
        .status()
        .expect("run cargo component build");
    assert!(status.success(), "component fixture build failed");

    let wasm_path = built_fixture_path(&workspace_root).expect("built fixture artifact");
    let wasm = std::fs::read(&wasm_path).expect("read built wasm");

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

    if let Some(decoded) = decode_component_wit(&wasm_path) {
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
    }
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

fn workspace_root() -> Option<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .map(Path::to_path_buf)
}

fn built_fixture_path(workspace_root: &Path) -> Option<PathBuf> {
    let candidates = [
        workspace_root.join("target/wasm32-wasip1/release/component_v060_canonical_dummy.wasm"),
        workspace_root.join("target/wasm32-wasip2/release/component_v060_canonical_dummy.wasm"),
    ];
    candidates.into_iter().find(|path| path.exists())
}

fn decode_component_wit(wasm_path: &Path) -> Option<String> {
    let output = Command::new("wasm-tools")
        .args(["component", "wit"])
        .arg(wasm_path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
