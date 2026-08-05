// API client for the Observer Node
const BASE_URL = process.env.NEXT_PUBLIC_OBSERVER_URL || 'http://127.0.0.1:7411'

export async function fetchAPI(endpoint: string, options?: RequestInit) {
  const res = await fetch(`${BASE_URL}${endpoint}`, {
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    ...options,
  })
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }))
    throw new Error(err.error || `HTTP ${res.status}`)
  }
  return res.json()
}

// Agents
export const listAgents = () => fetchAPI('/v1/agents')
export const getAgent = (id: string) => fetchAPI(`/v1/agents/${id}`)
export const registerAgent = (data: any) => fetchAPI('/v1/agents', { method: 'POST', body: JSON.stringify(data) })
export const stopAgent = (id: string) => fetchAPI(`/v1/agents/${id}/stop`, { method: 'POST' })
export const revokeAgent = (id: string) => fetchAPI(`/v1/agents/${id}/revoke`, { method: 'POST' })

// Approvals
export const listApprovals = () => fetchAPI('/v1/approvals')
export const approveDecision = (id: string, decision: string, durationSeconds?: number) =>
  fetchAPI(`/v1/approvals/${id}/decision`, { method: 'POST', body: JSON.stringify({ decision, duration_seconds: durationSeconds }) })

// Residuals
export const listResiduals = () => fetchAPI('/v1/residuals')

// Receipts
export const listReceipts = () => fetchAPI('/v1/receipts')

// Consequences
export const listConsequences = () => fetchAPI('/v1/consequences')

// Proof
export const exportProof = () => fetchAPI('/v1/proofs/export', { method: 'POST', body: '{}' })

// Emergency
export const emergencyStop = () => fetchAPI('/v1/emergency-stop', { method: 'POST', body: '{}' })

// Health
export const healthCheck = () => fetchAPI('/health')
