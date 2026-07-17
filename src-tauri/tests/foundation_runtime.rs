use project_mender_lib::foundation::PhaseOneRuntime;

#[test]
fn starts_supported_runtime_idle_without_project_network_or_process_activity() {
    let private_root = tempfile::tempdir().unwrap();

    let runtime = PhaseOneRuntime::start_at(private_root.path()).unwrap();

    assert!(runtime.is_idle());
    assert!(!runtime.project_access_active());
    assert!(!runtime.network_enabled());
    assert!(!runtime.process_execution_enabled());
    assert!(!runtime.model_provider_enabled());
    assert!(!runtime.scanner_enabled());
    assert!(!runtime.project_write_enabled());
    assert!(!runtime.patch_application_enabled());
}
