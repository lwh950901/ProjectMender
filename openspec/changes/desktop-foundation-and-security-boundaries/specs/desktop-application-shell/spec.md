## ADDED Requirements

### Requirement: Supported desktop application startup
The system SHALL start a ProjectMender desktop application on macOS Apple Silicon and Windows 11 x64, and SHALL present a usable application shell or a user-actionable startup failure.

#### Scenario: Successful startup
- **WHEN** the application starts on a supported platform with its application data directory available
- **THEN** it presents the ProjectMender shell without requiring a project directory, network connection, or API Key

#### Scenario: Application data initialization failure
- **WHEN** the application cannot initialize its private data directory
- **THEN** it presents an actionable failure state and SHALL NOT fall back to storing data in a selected project directory

### Requirement: Safe foundation runtime
The phase-1 application SHALL NOT register or invoke network clients, child-process execution, model providers, project write operations, scanners, or patch application handlers.

#### Scenario: Application shell is idle
- **WHEN** the user opens the application without selecting a project
- **THEN** no project files are read and no network or process activity is initiated
