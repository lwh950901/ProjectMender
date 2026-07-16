use std::path::Path;

use crate::private_data::{PrivateDataError, PrivateDataRoot};

#[derive(Debug)]
pub struct PhaseOneRuntime {
    _private_data: PrivateDataRoot,
}

impl PhaseOneRuntime {
    pub fn start_at(private_data_root: &Path) -> Result<Self, PrivateDataError> {
        Ok(Self { _private_data: PrivateDataRoot::initialize_at(private_data_root)? })
    }

    pub fn is_idle(&self) -> bool { true }
    pub fn project_access_active(&self) -> bool { false }
    pub fn network_enabled(&self) -> bool { false }
    pub fn process_execution_enabled(&self) -> bool { false }
}
