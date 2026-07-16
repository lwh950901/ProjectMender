import { fireEvent, render, screen } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { AuthorizationPanel } from "./AuthorizationPanel";

it("shows read-only authorization metadata and explicit non-executing actions", () => {
  const approve = vi.fn();
  render(<AuthorizationPanel state="pending" scope="selected range" onApprove={approve} onReject={vi.fn()} onCancel={vi.fn()} />);
  expect(screen.getByText("pending")).toBeInTheDocument();
  fireEvent.click(screen.getByRole("button", { name: "Approve" }));
  expect(approve).toHaveBeenCalledOnce();
});
