## ADDED Requirements

### Requirement: Explicit project directory authorization
The system SHALL obtain project access only after the user selects a local directory through the application flow, and SHALL represent that authorization with a new opaque project session identifier.

#### Scenario: User selects a project directory
- **WHEN** the user confirms a local directory selection
- **THEN** the system creates a project session identifier associated with the canonical project root and does not expose that root as a reusable UI authorization token

#### Scenario: No project selected
- **WHEN** no active project session exists
- **THEN** project-scoped operations are rejected without attempting file-system access

### Requirement: Project path confinement
The system SHALL accept project-relative paths only when their resolved targets remain inside the active canonical project root, and SHALL reject absolute paths, parent traversal, and links that resolve outside that root.

#### Scenario: Relative file request remains inside root
- **WHEN** a project-scoped read request names a valid relative path whose resolved target is inside the active root
- **THEN** the system permits only the authorized read operation

#### Scenario: Path attempts to escape root
- **WHEN** a request includes an absolute path, parent traversal, or a symbolic link resolving outside the active root
- **THEN** the system rejects the request and records only safe error metadata

### Requirement: Application data separation
The system SHALL store application metadata outside every selected project directory and SHALL NOT create project files during phase 1.

#### Scenario: Application records local metadata
- **WHEN** the application creates session or audit metadata
- **THEN** it writes only to its private data directory and leaves the selected project unchanged
