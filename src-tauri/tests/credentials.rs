use project_mender_lib::credentials::{CredentialStore, MemoryCredentialStore, OsCredentialStore, Settings};

struct UnavailableCredentialStore;

impl CredentialStore for UnavailableCredentialStore {
    fn save(&mut self, _: &str, _: &str) -> Result<(), project_mender_lib::credentials::CredentialError> {
        Err(project_mender_lib::credentials::CredentialError::Unavailable)
    }

    fn is_present(&self, _: &str) -> Result<bool, project_mender_lib::credentials::CredentialError> {
        Err(project_mender_lib::credentials::CredentialError::Unavailable)
    }

    fn remove(&mut self, _: &str) -> Result<(), project_mender_lib::credentials::CredentialError> {
        Err(project_mender_lib::credentials::CredentialError::Unavailable)
    }
}

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

#[test]
fn api_keys_are_absent_from_debug_serialization_and_project_files() {
    let secret = "credential-that-must-never-leak";
    let project = tempfile::tempdir().unwrap();
    let mut store = MemoryCredentialStore::default();
    let settings = Settings::save(&mut store, "openai", "gpt", secret).unwrap();

    let ordinary_surfaces = [
        serde_json::to_string(&settings).unwrap(),
        format!("{settings:?}"),
        format!("{}", project.path().display()),
        format!("{:?}", project.path().read_dir().unwrap().count()),
    ];

    assert!(ordinary_surfaces.iter().all(|surface| !surface.contains(secret)));
    assert_eq!(project.path().read_dir().unwrap().count(), 0);
}

#[test]
fn unavailable_credential_storage_returns_a_safe_error_without_the_api_key() {
    let secret = "credential-that-must-never-appear-in-an-error";
    let result = Settings::save(&mut UnavailableCredentialStore, "openai", "gpt", secret);

    let error = result.unwrap_err();
    assert_eq!(error.to_string(), "credential storage unavailable");
    assert!(!error.to_string().contains(secret));
}

#[cfg(target_os = "windows")]
#[test]
fn windows_credential_manager_creates_and_removes_a_test_credential() {
    let reference = format!("phase-1-platform-test-{}", uuid::Uuid::new_v4());
    let entry = keyring::Entry::new("com.projectmender", &reference).unwrap();
    let mut store = OsCredentialStore;

    store.save(&reference, "test-secret-not-for-serialization").unwrap();
    let password = entry.get_password();
    store.remove(&reference).unwrap();
    match password {
        Ok(password) => assert_eq!(password, "test-secret-not-for-serialization"),
        Err(error) => panic!("credential could not be read after save: {error:?}"),
    }
    assert!(!store.is_present(&reference).unwrap());
}
