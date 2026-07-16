type Props = { state: string; scope: string; onApprove: () => void; onReject: () => void; onCancel: () => void };
export function AuthorizationPanel({ state, scope, onApprove, onReject, onCancel }: Props) {
  return <section aria-label="Authorization request"><p>{state}</p><p>{scope}</p><button onClick={onApprove}>Approve</button><button onClick={onReject}>Reject</button><button onClick={onCancel}>Cancel</button></section>;
}
