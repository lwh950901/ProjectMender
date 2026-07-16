use project_mender_lib::authorization::{AuthorizationRequest, LifecycleState, OperationCategory};

#[test]
fn authorization_is_scoped_versioned_and_never_executes_phase_one_operations() {
    let mut request = AuthorizationRequest::pending(
        OperationCategory::ModelContextSend,
        "session-opaque",
        "selected file range",
        10,
    );
    assert_eq!(request.state(), LifecycleState::Pending);
    assert_eq!(request.schema_version(), 1);
    assert!(!request.can_execute_in_phase_one());

    request.approve(11).unwrap();
    assert_eq!(request.state(), LifecycleState::Approved);
    assert!(!request.can_execute_in_phase_one());
    request.expire_if_due(71);
    assert_eq!(request.state(), LifecycleState::Expired);
}

#[test]
fn rejected_cancelled_and_expired_requests_cannot_be_approved_again() {
    let mut request = AuthorizationRequest::pending(OperationCategory::PatchApply, "session", "scope", 10);
    request.reject(11).unwrap();
    assert!(request.approve(12).is_err());
}
