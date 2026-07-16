import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import App from "./App";

describe("ProjectMender application shell", () => {
  it("starts in an idle state without requiring a project or credentials", () => {
    render(<App />);

    expect(screen.getByRole("heading", { name: "ProjectMender" })).toBeInTheDocument();
    expect(screen.getByText("Select a project to begin.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Select project" })).toBeEnabled();
  });

  it("shows an actionable recovery view when private-data initialization fails", () => {
    render(<App startupState={{ kind: "initialization-error", category: "private-data-unavailable" }} />);

    expect(screen.getByRole("alert")).toHaveTextContent("ProjectMender could not initialize its private data.");
    expect(screen.getByRole("button", { name: "Retry initialization" })).toBeEnabled();
  });
});
