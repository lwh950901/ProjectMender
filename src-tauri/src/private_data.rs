use std::{fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

const CURRENT_SCHEMA_VERSION: u32 = 1;
const SCHEMA_FILE: &str = "schema.json";

#[derive(Debug)]
pub struct PrivateDataRoot {
    path: PathBuf,
    schema_version: u32,
}

#[derive(Debug, thiserror::Error)]
#[error("application private data is unavailable; recovery mode is required")]
pub struct PrivateDataError;

#[derive(Serialize, Deserialize)]
struct SchemaRecord {
    schema_version: u32,
}

impl PrivateDataRoot {
    pub fn initialize_at(path: &Path) -> Result<Self, PrivateDataError> {
        fs::create_dir_all(path).map_err(|_| PrivateDataError)?;
        let schema = SchemaRecord { schema_version: CURRENT_SCHEMA_VERSION };
        let encoded = serde_json::to_vec(&schema).map_err(|_| PrivateDataError)?;
        fs::write(path.join(SCHEMA_FILE), encoded).map_err(|_| PrivateDataError)?;

        Ok(Self { path: path.to_path_buf(), schema_version: CURRENT_SCHEMA_VERSION })
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl PrivateDataError {
    pub fn is_recovery_mode(&self) -> bool {
        true
    }

    pub fn category(&self) -> &str {
        "private-data-unavailable"
    }
}
