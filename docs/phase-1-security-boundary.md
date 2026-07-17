# Phase 1 security boundary

## Verified locally on macOS Apple Silicon

- The application starts with an application-private, versioned data root and enters recovery mode when that root cannot be initialized.
- An idle runtime has no project session, network client, process launcher, scanner, model provider, project writer, or patch executor.
- Project reads require an opaque session identifier and a relative path. Each read canonicalizes both root and target and rejects missing sessions, absolute paths, parent traversal, and links outside the root.
- Credentials are stored by the operating-system credential facility through `OsCredentialStore`. Serializable settings contain only provider, model, and a credential reference.
- Authorization requests are metadata only. Every lifecycle state remains non-executing during Phase 1.
- Frontend tests, all Rust tests, and a no-bundle Tauri desktop build have passed locally; the resulting binary is a macOS arm64 executable.
- Direct manifest, command-handler, capability, and runtime-surface tests verify that Phase 1 has no registered network, process, model, scanner, project-write, or patch capability.
- Credential tests cover JSON and debug serialization, normal error surfaces, and project-directory non-persistence using a memory credential-store fake.

## Cross-platform verification status

- `.github/workflows/phase-one-platform.yml` runs frontend tests, Rust tests, and no-bundle Tauri builds on macOS and Windows GitHub-hosted runners.
- `docs/phase-1-platform-acceptance.md` defines the required packaged-application acceptance evidence for macOS Apple Silicon and Windows 11 x64.
- A real Windows 11 x64 packaged-application run remains outstanding and must be recorded before Phase 1 task 5.4 can be completed.

## Phase 2 handoff contracts

- Future readers must call `ProjectAccess::read_file` with an existing project session; they must not accept arbitrary absolute paths.
- Future network, process, write, model, scanner, and patch consumers require a separate OpenSpec change and must not treat an approved Phase 1 request as executable permission.
- Future credential consumers must receive a secret only within a narrow OS-credential adapter and must never serialize it.

## Platform finding

The automated suite and a no-bundle desktop build have run on macOS Apple Silicon. Windows CI coverage is configured, but real Windows 11 x64 packaging and credential-facility scenarios remain a release-platform verification task and are deliberately not asserted as locally verified.
