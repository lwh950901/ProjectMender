use std::{fs, path::PathBuf};

fn project_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(path).unwrap()
}

#[test]
fn phase_one_manifest_excludes_direct_high_risk_dependencies() {
    let manifest = project_file("Cargo.toml");
    let prohibited_dependencies = [
        "reqwest =",
        "ureq =",
        "tauri-plugin-http",
        "tauri-plugin-shell",
        "tauri-plugin-fs",
    ];

    assert!(prohibited_dependencies
        .iter()
        .all(|dependency| !manifest.contains(dependency)));
}

#[test]
fn phase_one_command_and_capability_surface_only_exposes_project_selection() {
    let runtime = project_file("src/lib.rs");
    let capabilities = project_file("capabilities/default.json");

    assert!(runtime.contains("generate_handler![create_project_session]"));
    for prohibited_handler in ["scan", "write", "patch", "model", "process", "command", "network"] {
        assert!(!runtime.contains(&format!("fn {prohibited_handler}")));
    }

    for prohibited_permission in ["fs:", "shell:", "http:", "process:"] {
        assert!(!capabilities.contains(prohibited_permission));
    }
}
