#![allow(dead_code)]

#[path = "../build_support/wit_paths.rs"]
mod wit_paths;

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn prefers_bundled_wit_root_over_workspace_paths() {
    let root = temp_root("bundled-preferred");
    let manifest_dir = root.join("crate");
    let bundled = manifest_dir.join("wit");
    let workspace = root.join("wit");

    fs::create_dir_all(bundled.join("greentic/component@0.6.0")).expect("create bundled wit");
    fs::create_dir_all(workspace.join("greentic/component@0.6.0")).expect("create workspace wit");
    fs::write(
        bundled.join("greentic/component@0.6.0/package.wit"),
        "package greentic:component@0.6.0;\n",
    )
    .expect("write bundled wit");
    fs::write(
        workspace.join("greentic/component@0.6.0/package.wit"),
        "package greentic:component@0.6.0;\n",
    )
    .expect("write workspace wit");

    let selected = wit_paths::canonical_wit_root_from(&manifest_dir);
    let expected = fs::canonicalize(PathBuf::from(&bundled)).expect("canonical bundled path");
    assert_eq!(selected, expected);

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn falls_back_to_workspace_wit_when_bundle_is_absent() {
    let root = temp_root("workspace-fallback");
    let manifest_dir = root.join("crates/greentic-interfaces-guest");
    let workspace = root.join("wit");

    fs::create_dir_all(workspace.join("greentic/component@0.6.0")).expect("create workspace wit");
    fs::create_dir_all(&manifest_dir).expect("create manifest dir");
    fs::write(
        workspace.join("greentic/component@0.6.0/package.wit"),
        "package greentic:component@0.6.0;\n",
    )
    .expect("write workspace wit");

    let selected = wit_paths::canonical_wit_root_from(&manifest_dir);
    let expected = fs::canonicalize(PathBuf::from(&workspace)).expect("canonical workspace path");
    assert_eq!(selected, expected);

    let _ = fs::remove_dir_all(&root);
}

fn temp_root(label: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!("gi-guest-wit-paths-{label}-{unique}"))
}
