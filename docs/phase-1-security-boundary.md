# Phase 1 security boundary

## Verified locally on macOS Apple Silicon

- The application starts with an application-private, versioned data root and enters recovery mode when that root cannot be initialized.
- An idle runtime has no project session, network client, process launcher, scanner, model provider, project writer, or patch executor.
- Project reads require an opaque session identifier and a relative path. Each read canonicalizes both root and target and rejects missing sessions, absolute paths, parent traversal, and links outside the root.
- Credentials are stored by the operating-system credential facility through `OsCredentialStore`. Serializable settings contain only provider, model, and a credential reference.
- Authorization requests are metadata only. Every lifecycle state remains non-executing during Phase 1.

## Phase 2 handoff contracts

- Future readers must call `ProjectAccess::read_file` with an existing project session; they must not accept arbitrary absolute paths.
- Future network, process, write, model, scanner, and patch consumers require a separate OpenSpec change and must not treat an approved Phase 1 request as executable permission.
- Future credential consumers must receive a secret only within a narrow OS-credential adapter and must never serialize it.

## Platform finding

The automated suite has run on macOS Apple Silicon. Windows 11 x64 packaging and credential-facility scenarios remain a release-platform verification task and are deliberately not asserted as locally verified.
