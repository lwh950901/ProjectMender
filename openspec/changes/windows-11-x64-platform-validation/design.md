## Evidence model

The evidence belongs to the candidate application build, not merely to CI. Each checklist result records the Windows 11 edition/version, x64 architecture, application build identifier, tester, date, result, and any relevant artifact or screenshot path.

## Acceptance scope

The validation reruns the Windows x64 baseline checklist from `docs/phase-1-platform-acceptance.md`: no-project startup, in-root access, path-escape rejection, private-data recovery, unavailable credential storage, and Credential Manager creation/removal semantics. It also records Windows 11-specific packaging, UAC, file-system, and credential-facility observations.

## Non-goals

This change neither expands the Phase 1 capability surface nor changes the Windows x64 baseline evidence accepted by the desktop-foundation change.
