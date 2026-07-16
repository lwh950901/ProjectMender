use project_mender_lib::private_data::PrivateDataRoot;

#[test]
fn initializes_versioned_application_private_data_at_the_given_root() {
    let temporary_directory = tempfile::tempdir().unwrap();
    let root = temporary_directory.path().join("projectmender-private");

    let initialized = PrivateDataRoot::initialize_at(&root).unwrap();

    assert_eq!(initialized.schema_version(), 1);
    assert_eq!(initialized.path(), root);
    assert!(initialized.path().join("schema.json").is_file());
}

#[test]
fn reports_recovery_mode_when_private_data_cannot_be_initialized() {
    let temporary_file = tempfile::NamedTempFile::new().unwrap();
    let impossible_root = temporary_file.path().join("child");

    let error = PrivateDataRoot::initialize_at(&impossible_root).unwrap_err();

    assert!(error.is_recovery_mode());
    assert_eq!(error.category(), "private-data-unavailable");
}
