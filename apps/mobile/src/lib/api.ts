// API client for the Observer Node
const NODE_URL = 'http://127.0.0.1:7411'

async function fetchAPI(endpoint: string, options?: RequestInit) {
  const res = await fetch(`${NODE_URL}${endpoint}`, {
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    ...options,
  })
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }))
    throw new Error(err.error || `HTTP ${res.status}`)
  }
  return res.json()
}

export const healthCheck = () => fetchAPI('/health')
export const listAgents = () => fetchAPI('/v1/agents')
export const listApprovals = () => fetchAPI('/v1/approvals')
export const approveDecision = (id: string, decision: string, durationSeconds?: number) =>
  fetchAPI(`/v1/approvals/${id}/decision`, { method: 'POST', body: JSON.stringify({ decision, duration_seconds: durationSeconds }) })
export const emergencyStop = () => fetchAPI('/v1/emergency-stop', { method: 'POST', body: '{}' })
export const listResiduals = () => fetchAPI('/v1/residuals')
export const listReceipts = () => fetchAPI('/v1/receipts')
export const stopAgent = (id: string) => fetchAPI(`/v1/agents/${id}/stop`, { method: 'POST' })
export const exportProof = () => fetchAPI('/v1/proofs/export', { method: 'POST', body: '{}' })
