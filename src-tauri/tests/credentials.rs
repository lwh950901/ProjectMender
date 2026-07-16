use project_mender_lib::credentials::{CredentialStore, MemoryCredentialStore, Settings};

#[test]
fn settings_only_serializes_credential_reference_not_api_key() {
    let mut store = MemoryCredentialStore::default();
    let settings = Settings::save(&mut store, "openai", "gpt", "secret-api-key").unwrap();
    let serialized = serde_json::to_string(&settings).unwrap();

    assert!(!serialized.contains("secret-api-key"));
    assert!(store.is_present(settings.credential_reference()).unwrap());
}

#[test]
fn api_keys_are_absent_from_all_normal_serialization_and_error_surfaces() {
    let secret = "secret-api-key";
    let mut store = MemoryCredentialStore::default();
    let settings = Settings::save(&mut store, "openai", "gpt", secret).unwrap();
    let ordinary_surfaces = [
        serde_json::to_string(&settings).unwrap(),
        format!("{:?}", settings.credential_reference()),
        format!("{}", project_mender_lib::credentials::CredentialError::Unavailable),
    ];

    assert!(ordinary_surfaces.iter().all(|surface| !surface.contains(secret)));
}
