use std::fs;

use project_mender_lib::project_access::{ProjectAccess, ProjectAccessError};

#[test]
fn reads_a_file_inside_a_selected_project_using_only_an_opaque_session() {
    let project = tempfile::tempdir().unwrap();
    fs::write(project.path().join("inside.txt"), "allowed").unwrap();
    let mut access = ProjectAccess::default();
    let session = access.select_directory(project.path()).unwrap();

    assert_eq!(access.read_file(&session, "inside.txt").unwrap(), b"allowed");
}

#[test]
fn rejects_missing_sessions_absolute_paths_parent_traversal_and_outside_symlinks() {
    let project = tempfile::tempdir().unwrap();
    let outside = tempfile::NamedTempFile::new().unwrap();
    #[cfg(unix)] std::os::unix::fs::symlink(outside.path(), project.path().join("escape")).unwrap();
    let mut access = ProjectAccess::default();
    let session = access.select_directory(project.path()).unwrap();

    assert_eq!(access.read_file("missing", "file").unwrap_err(), ProjectAccessError::MissingSession);
    assert_eq!(access.read_file(&session, "/etc/hosts").unwrap_err(), ProjectAccessError::InvalidPath);
    assert_eq!(access.read_file(&session, "../outside").unwrap_err(), ProjectAccessError::InvalidPath);
    #[cfg(unix)] assert_eq!(access.read_file(&session, "escape").unwrap_err(), ProjectAccessError::OutsideRoot);
}
