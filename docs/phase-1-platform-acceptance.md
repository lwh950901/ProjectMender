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

## Windows 11 x64

- Install or launch the packaged application on a real Windows 11 x64 machine.
- Repeat the no-project startup, selected-project access, path-escape rejection, private-data failure, and unavailable credential-facility checks listed for macOS.
- Confirm the Windows Credential Manager entry is created and removed only when the corresponding credential action succeeds, and that normal settings, project files, logs, and visible errors do not contain the original API key.
- Record any platform-specific prompt, credential-facility, packaging, or file-system behavior as a finding; a failed check leaves Phase 1 task 5.4 incomplete.
