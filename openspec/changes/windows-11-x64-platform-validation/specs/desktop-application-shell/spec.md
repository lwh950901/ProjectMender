## ADDED Requirements

### Requirement: Windows 11 packaged-application release evidence
The project SHALL retain release evidence from exercising the candidate packaged application on a real Windows 11 x64 environment before claiming Windows 11 platform verification.

#### Scenario: Windows 11 acceptance run is recorded

- **WHEN** the packaged candidate application is exercised on a real Windows 11 x64 device
- **THEN** the evidence records the OS edition and version, x64 architecture, application build identifier, tester, date, checklist results, and Windows 11-specific findings

#### Scenario: Windows 11 evidence is unavailable

- **WHEN** no real Windows 11 x64 device is available for the candidate application
- **THEN** the project SHALL leave Windows 11 platform verification outstanding and SHALL NOT represent a different Windows version as Windows 11 evidence
