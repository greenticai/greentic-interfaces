use std::fs;
use std::path::{Path, PathBuf};

pub fn canonical_wit_root() -> PathBuf {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));
    canonical_wit_root_from(&manifest_dir)
}

pub(crate) fn canonical_wit_root_from(manifest_dir: &Path) -> PathBuf {
    // The published crate carries its own canonical WIT bundle.
    let bundled = manifest_dir.join("wit");
    if has_wit_files(&bundled) {
        return bundled
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    // Workspace checkout from `crates/<this-crate>`.
    let workspace_root_wit = manifest_dir.join("../../wit");
    if has_wit_files(&workspace_root_wit) {
        return workspace_root_wit
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    // Workspace checkout from a sibling crate during local development.
    let workspace_sibling = manifest_dir.join("../greentic-interfaces/wit");
    if has_wit_files(&workspace_sibling) {
        return workspace_sibling
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    // `cargo package` verification from `target/package/<crate-version>`.
    let package_verify_workspace = manifest_dir.join("../../../crates/greentic-interfaces/wit");
    if has_wit_files(&package_verify_workspace) {
        return package_verify_workspace
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    panic!("Failed to locate canonical WIT root");
}

fn has_wit_files(root: &Path) -> bool {
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("wit") {
                return true;
            }
        }
    }
    false
}
