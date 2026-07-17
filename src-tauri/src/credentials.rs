use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CredentialError { #[error("credential storage unavailable")] Unavailable }

pub trait CredentialStore { fn save(&mut self, reference: &str, secret: &str) -> Result<(), CredentialError>; fn is_present(&self, reference: &str) -> Result<bool, CredentialError>; fn remove(&mut self, reference: &str) -> Result<(), CredentialError>; }

#[derive(Default)]
pub struct MemoryCredentialStore(HashMap<String, String>);
impl CredentialStore for MemoryCredentialStore { fn save(&mut self, reference: &str, secret: &str) -> Result<(), CredentialError> { self.0.insert(reference.into(), secret.into()); Ok(()) } fn is_present(&self, reference: &str) -> Result<bool, CredentialError> { Ok(self.0.contains_key(reference)) } fn remove(&mut self, reference: &str) -> Result<(), CredentialError> { self.0.remove(reference); Ok(()) } }

pub struct OsCredentialStore;
impl CredentialStore for OsCredentialStore { fn save(&mut self, reference: &str, secret: &str) -> Result<(), CredentialError> { keyring::Entry::new("com.projectmender", reference).map_err(|_| CredentialError::Unavailable)?.set_password(secret).map_err(|_| CredentialError::Unavailable) } fn is_present(&self, reference: &str) -> Result<bool, CredentialError> { Ok(keyring::Entry::new("com.projectmender", reference).map_err(|_| CredentialError::Unavailable)?.get_password().is_ok()) } fn remove(&mut self, reference: &str) -> Result<(), CredentialError> { keyring::Entry::new("com.projectmender", reference).map_err(|_| CredentialError::Unavailable)?.delete_credential().map_err(|_| CredentialError::Unavailable) } }

#[derive(Debug, Serialize)]
pub struct Settings { provider: String, model: String, credential_reference: String }
impl Settings { pub fn save(store: &mut impl CredentialStore, provider: &str, model: &str, secret: &str) -> Result<Self, CredentialError> { let credential_reference = format!("{}:{}", provider, model); store.save(&credential_reference, secret)?; Ok(Self { provider: provider.into(), model: model.into(), credential_reference }) } pub fn credential_reference(&self) -> &str { &self.credential_reference } }
