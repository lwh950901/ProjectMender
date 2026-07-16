import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { AuthorizationPanel } from "./AuthorizationPanel";

export type StartupState =
  | { kind: "ready" }
  | { kind: "initialization-error"; category: "private-data-unavailable" };

type AppProps = {
  startupState?: StartupState;
};

export default function App({ startupState = { kind: "ready" } }: AppProps) {
  const selectProject = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") {
      await invoke("create_project_session", { directory: selected });
    }
  };
  if (startupState.kind === "initialization-error") {
    return (
      <main>
        <h1>ProjectMender</h1>
        <section role="alert">
          <p>ProjectMender could not initialize its private data.</p>
          <p>Your selected projects have not been changed.</p>
          <button type="button">Retry initialization</button>
        </section>
      </main>
    );
  }

  return (
    <main>
      <h1>ProjectMender</h1>
      <p>Select a project to begin.</p>
      <button type="button" onClick={() => void selectProject()}>Select project</button>
      <AuthorizationPanel state="No authorization request" scope="No project scope" onApprove={() => {}} onReject={() => {}} onCancel={() => {}} />
    </main>
  );
}
