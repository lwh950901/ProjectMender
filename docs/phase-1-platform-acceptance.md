# Phase 1 platform acceptance checklist

This checklist supplements CI. It is not complete until the packaged application has been exercised on both listed platforms. Record the operating-system version, application build identifier, tester, date, and result for each item in the release evidence.

## CI evidence

- Confirm the `Phase 1 platform verification` workflow passes on `macos-latest` and `windows-latest`.
- Retain the workflow URLs or run identifiers for the tested commit.
- Treat a successful CI build as build and automated-test evidence only; it does not replace the real-device checks below.

## macOS Apple Silicon

- Install or launch the packaged application on Apple Silicon running macOS 11 or later.
- Start the application with no selected project and no API key; confirm the usable shell appears without a network or process action.
- Select a fixture project and confirm in-root reads succeed while absolute paths, parent traversal, and external symbolic links are rejected.
- Force private-data initialization to fail (for example, by using a path whose parent is a file in the test build) and confirm the actionable recovery state appears without writing into the selected project.
- Use a missing, locked, or rejected credential-facility case and confirm the user receives the safe `credential storage unavailable` outcome with no API key exposed.

## Windows 10 x64 baseline

- Install or launch the packaged application on a real Windows 10 x64 machine, and record the exact operating-system version.
- Repeat the no-project startup, selected-project access, path-escape rejection, private-data failure, and unavailable credential-facility checks listed for macOS.
- Confirm the Windows Credential Manager entry is created and removed only when the corresponding credential action succeeds, and that normal settings, project files, logs, and visible errors do not contain the original API key.
- Record any platform-specific prompt, credential-facility, packaging, or file-system behavior as a finding; a failed check leaves Phase 1 task 5.4 incomplete.

### Evidence — 2026-07-17

- Tester: Codex-assisted local verification; environment: Windows 10 Pro 22H2, build 19045, x64; WebView2 Runtime 150.0.4078.65.
- Package: `ProjectMender_0.1.0_x64_en-US.msi`, SHA-256 `7F000F1218CC4BEB16FD8964AA22058023243D9071DAA636BDCF20E81E1D8035`.
- Packaging and installation: x64 MSI built with WiX 3.14; a clean elevated install completed with Windows Installer status 0. The installed executable's SHA-256 matched the executable extracted from that MSI: `306D9E1AF22B6974BAE901A9F872D4BA5DA41F739405B4C2D7023E0D627DB6FD`.
- Packaged startup: the installed application was launched in the real desktop session and remained running and responsive with no project selected.
- Boundary scenarios: `cargo test --locked` passed 14 tests on this Windows 10 host, including the idle runtime, in-root read, missing-session/absolute-path/parent-traversal/external-symlink rejection, private-data recovery, credential-unavailable outcome, and API-key serialization-surface checks.
- Credential lifecycle: after enabling the `windows-native` keyring backend, the Windows-specific test created a random Credential Manager entry, confirmed its presence, removed it, and confirmed absence. An independent `cmdkey` create/list/delete check also passed; the test entry was removed.
- Findings: the initial `keyring = "3"` configuration selected its non-native mock backend because neither platform-native feature was enabled. It was corrected to enable `windows-native` and `apple-native`; no fallback file store was introduced.
- Result: Windows 10 x64 baseline passed. The separate Windows 11 follow-up remains tracked by `windows-11-x64-platform-validation`.

## Windows 11 x64 follow-up

- On a real Windows 11 x64 machine, rerun the Windows 10 x64 baseline checklist against the packaged application.
- Retain the operating-system version, application build identifier, tester, date, and results in the `windows-11-x64-platform-validation` change.
- Record Windows 11-specific findings separately; this follow-up is not a substitute for the Windows 10 task 5.4 baseline evidence.
