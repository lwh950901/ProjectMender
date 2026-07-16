use std::{collections::HashMap, fs, path::{Component, Path, PathBuf}};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectAccessError { MissingSession, InvalidPath, OutsideRoot, ReadFailed }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeAudit { pub event: &'static str, pub outcome: &'static str }

#[derive(Default)]
pub struct ProjectAccess { sessions: HashMap<String, PathBuf>, audits: Vec<SafeAudit> }

impl ProjectAccess {
    pub fn select_directory(&mut self, directory: &Path) -> Result<String, ProjectAccessError> {
        let root = directory.canonicalize().map_err(|_| ProjectAccessError::ReadFailed)?;
        if !root.is_dir() { return Err(ProjectAccessError::ReadFailed); }
        let session = Uuid::new_v4().to_string();
        self.sessions.insert(session.clone(), root);
        self.audits.push(SafeAudit { event: "project-session", outcome: "created" });
        Ok(session)
    }

    pub fn read_file(&mut self, session: &str, relative_path: &str) -> Result<Vec<u8>, ProjectAccessError> {
        let root = self.sessions.get(session).ok_or(ProjectAccessError::MissingSession)?;
        let requested = Path::new(relative_path);
        if requested.is_absolute() || requested.components().any(|part| matches!(part, Component::ParentDir | Component::RootDir | Component::Prefix(_))) {
            return Err(ProjectAccessError::InvalidPath);
        }
        let root = root.canonicalize().map_err(|_| ProjectAccessError::ReadFailed)?;
        let target = root.join(requested).canonicalize().map_err(|_| ProjectAccessError::ReadFailed)?;
        if !target.starts_with(&root) { self.audits.push(SafeAudit { event: "path-policy", outcome: "outside-root" }); return Err(ProjectAccessError::OutsideRoot); }
        fs::read(target).map_err(|_| ProjectAccessError::ReadFailed)
    }

    pub fn audits(&self) -> &[SafeAudit] { &self.audits }
}
