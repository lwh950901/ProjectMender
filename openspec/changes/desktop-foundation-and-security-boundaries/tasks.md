## 1. Desktop application foundation

- [x] 1.1 Initialize the Tauri 2, React/TypeScript, and Rust application structure with macOS Apple Silicon and Windows x64 build targets.
- [x] 1.2 Implement the minimal application shell, startup state, and actionable initialization-error view without requiring a project, API Key, or network access.
- [x] 1.3 Create an application-private data-root abstraction with schema version initialization and a recovery-mode error path that never falls back to a project directory.
- [x] 1.4 Add tests for supported startup, private-data initialization failure, and an idle shell with no project access, network client, or process activity.

## 2. Project directory capability boundary

- [x] 2.1 Implement native local-directory selection and creation of an opaque project session identifier backed by a canonical project root in Rust.
- [x] 2.2 Implement the project-scoped read boundary that accepts a session identifier plus relative path and rejects missing sessions and absolute paths.
- [x] 2.3 Add canonical-path, parent-traversal, symbolic-link escape, and root revalidation checks before every permitted project read.
- [x] 2.4 Record safe project-session and path-policy audit metadata without persisting source content or absolute paths in ordinary UI state.
- [x] 2.5 Add fixture tests for in-root access, missing-session rejection, parent traversal, absolute paths, and symlinks resolving outside the selected root.

## 3. Credential storage boundary

- [x] 3.1 Define a credential-storage interface that separates provider/model configuration from original API Key values.
- [x] 3.2 Implement macOS Keychain and Windows Credential Manager adapters behind that interface for save, read-presence, and remove operations.
- [x] 3.3 Implement settings state that stores only provider, model name, and credential reference metadata; never return or serialize an original key after save.
- [x] 3.4 Add safe credential-unavailable errors for locked, missing, or rejected credential-facility operations.
- [ ] 3.5 Add adapter fakes and tests proving API Keys are absent from project files, normal configuration, databases, logs, audit records, errors, reports, and serialized UI state.

## 4. Authorization state and local audit

- [x] 4.1 Define versioned authorization and audit data models for operation category, project-session scope, range summary, lifecycle state, timestamps, expiry, and audit identifier.
- [x] 4.2 Implement pending, approved, rejected, cancelled, and expired transitions with one operation category and explicit scope per request.
- [x] 4.3 Expose read-only authorization state to the UI and provide explicit approve, reject, cancel, and expiry actions without registering any operation consumer.
- [x] 4.4 Add tests that an authorization state cannot initiate network access, child processes, model requests, project writes, scanner execution, patch application, or credential disclosure during phase 1.

## 5. Security hardening and phase verification

- [x] 5.1 Configure the Tauri command and capability surface so only phase-1 approved commands are reachable from the UI.
- [ ] 5.2 Verify phase-1 build and runtime configuration contains no network client, child-process launcher, model provider, scanner, patch writer, or project write handler.
- [ ] 5.3 Run the requirement scenarios on macOS Apple Silicon and Windows 11 x64, including unavailable data directory and credential facility cases.
- [x] 5.4 Document the verified phase-1 boundary, remaining platform-specific findings, and handoff contracts required by phase 2.
