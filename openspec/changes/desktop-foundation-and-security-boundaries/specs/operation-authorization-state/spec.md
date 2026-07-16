## ADDED Requirements

### Requirement: Scoped authorization lifecycle
The system SHALL model high-risk operation requests with an operation category, project session scope, range summary, lifecycle state, creation time, expiry time, and audit identifier.

#### Scenario: High-risk operation is requested
- **WHEN** a future-capable UI flow creates an authorization request
- **THEN** the system records it as `pending` with a single operation category and explicit scope summary

#### Scenario: User rejects a request
- **WHEN** the user rejects a pending authorization request
- **THEN** the system records the request as `rejected` and it cannot be consumed as approval

### Requirement: Approval is narrow and non-executing in phase 1
The system SHALL support `approved`, `rejected`, `cancelled`, and `expired` lifecycle states, but SHALL NOT use any state to perform network access, process execution, model sending, project writes, or credential disclosure during phase 1.

#### Scenario: Approval reaches its expiry
- **WHEN** an approved authorization request reaches its expiry time before a future consumer uses it
- **THEN** the system marks it `expired` and rejects its use

#### Scenario: Application is in phase 1
- **WHEN** an authorization request is approved during phase 1
- **THEN** no network connection, child process, project write, model request, or credential disclosure occurs
