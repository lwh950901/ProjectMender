## 1. Windows 11 x64 packaged-application validation

- [ ] 1.1 Produce or obtain the Windows MSI package for the candidate commit and record its build identifier.
- [ ] 1.2 Install or launch the package on a real Windows 11 x64 device and record the OS edition, version, architecture, tester, and date.
- [ ] 1.3 Verify no-project startup, selected-project in-root reads, and rejection of absolute paths, parent traversal, and external symbolic links.
- [ ] 1.4 Verify private-data initialization recovery and unavailable credential-storage behavior without exposing an API Key or writing into the selected project.
- [ ] 1.5 Verify Windows Credential Manager creation/removal semantics and absence of original API Keys from settings, project files, logs, and visible errors.
- [ ] 1.6 Retain checklist evidence and Windows 11-specific packaging, UAC, file-system, and credential findings in `docs/phase-1-platform-acceptance.md`.
