use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum OperationCategory { NetworkDependencyLookup, ModelContextSend, OpenspecWrite, PatchApply, ValidationCommand }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum LifecycleState { Pending, Approved, Rejected, Cancelled, Expired }
#[derive(Debug)] pub struct AuthorizationRequest { schema_version: u32, _audit_id: String, _category: OperationCategory, _project_session: String, _range_summary: String, state: LifecycleState, _created_at: u64, expiry: u64 }
impl AuthorizationRequest {
    pub fn pending(category: OperationCategory, project_session: &str, range_summary: &str, created_at: u64) -> Self { Self { schema_version: 1, _audit_id: Uuid::new_v4().to_string(), _category: category, _project_session: project_session.into(), _range_summary: range_summary.into(), state: LifecycleState::Pending, _created_at: created_at, expiry: created_at + 60 } }
    pub fn schema_version(&self) -> u32 { self.schema_version }
    pub fn state(&self) -> LifecycleState { self.state }
    pub fn approve(&mut self, at: u64) -> Result<(), &'static str> { if self.state != LifecycleState::Pending { return Err("request is not pending") } if at >= self.expiry { self.state = LifecycleState::Expired } else { self.state = LifecycleState::Approved }; Ok(()) }
    pub fn reject(&mut self, _: u64) -> Result<(), &'static str> { if self.state != LifecycleState::Pending { return Err("request is not pending") } self.state = LifecycleState::Rejected; Ok(()) }
    pub fn cancel(&mut self) -> Result<(), &'static str> { if self.state != LifecycleState::Pending { return Err("request is not pending") } self.state = LifecycleState::Cancelled; Ok(()) }
    pub fn expire_if_due(&mut self, at: u64) { if matches!(self.state, LifecycleState::Pending | LifecycleState::Approved) && at >= self.expiry { self.state = LifecycleState::Expired; } }
    pub fn can_execute_in_phase_one(&self) -> bool { false }
}
