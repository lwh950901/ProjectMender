## ADDED Requirements

### Requirement: Operating-system credential storage
The system SHALL save, retrieve, and remove model API Keys through the operating system credential facility behind a credential-storage interface.

#### Scenario: User saves an API Key
- **WHEN** the user saves a provider and model API Key in settings
- **THEN** the system stores the original key only in the operating system credential facility and returns no original key value to ordinary application storage

#### Scenario: User removes an API Key
- **WHEN** the user removes a configured API Key
- **THEN** the system removes the corresponding credential entry and marks the provider configuration as having no usable key

### Requirement: No plaintext credential persistence
The system SHALL NOT persist an API Key in project files, normal configuration files, databases, logs, audit records, error messages, reports, or serialized UI state.

#### Scenario: Credential storage is unavailable
- **WHEN** the operating system credential facility is locked, unavailable, or rejects a request
- **THEN** the system reports a credential-storage error without displaying or persisting the API Key
