'use client'

import { useState, useEffect } from 'react'
import { Shield, Activity, Users, AlertTriangle, FileCheck, Zap, RefreshCw, AlertOctagon } from 'lucide-react'
import * as api from '@/lib/api'

export default function Dashboard() {
  const [data, setData] = useState<any>({ agents: [], approvals: [], residuals: [], receipts: [], consequences: [] })
  const [health, setHealth] = useState<string>('checking...')
  const [error, setError] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState('overview')

  const refresh = async () => {
    try {
      const h = await api.healthCheck()
      setHealth(h.status)
      const [agents, approvals, residuals, receipts, consequences] = await Promise.all([
        api.listAgents().catch(() => ({ agents: [] })),
        api.listApprovals().catch(() => ({ approvals: [] })),
        api.listResiduals().catch(() => ({ residuals: [] })),
        api.listReceipts().catch(() => ({ receipts: [] })),
        api.listConsequences().catch(() => ({ consequences: [] })),
      ])
      setData({ agents: agents.agents || [], approvals: approvals.approvals || [], residuals: residuals.residuals || [], receipts: receipts.receipts || [], consequences: consequences.consequences || [] })
      setError(null)
    } catch (e: any) {
      setError(e.message)
      setHealth('disconnected')
    }
  }

  useEffect(() => { refresh(); const i = setInterval(refresh, 5000); return () => clearInterval(i) }, [])

  const tabs = [
    { id: 'overview', label: 'Overview', icon: Activity },
    { id: 'agents', label: 'Agents', icon: Users },
    { id: 'approvals', label: 'Approvals', icon: FileCheck },
    { id: 'residuals', label: 'Residuals', icon: AlertTriangle },
    { id: 'receipts', label: 'Receipts', icon: Shield },
  ]

  return (
    <div style={{ minHeight: '100vh', background: '#0a0a0f', color: '#e0e0e0', fontFamily: 'system-ui, sans-serif' }}>
      {/* Header */}
      <header style={{ borderBottom: '1px solid #1a1a2e', padding: '16px 24px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div>
          <h1 style={{ fontSize: '20px', fontWeight: 700, color: '#fff', margin: 0 }}>Inner I Control Center</h1>
          <span style={{ fontSize: '12px', color: health === 'ok' ? '#4ade80' : '#f87171' }}>
            Observer Node: {health}
          </span>
        </div>
        <div style={{ display: 'flex', gap: 8 }}>
          {error && <span style={{ fontSize: '12px', color: '#f87171', alignSelf: 'center' }}>{error}</span>}
          <button onClick={refresh} style={{ background: '#1a1a2e', border: '1px solid #2a2a4e', color: '#e0e0e0', borderRadius: 6, padding: '8px 12px', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: 6 }}>
            <RefreshCw size={14} /> Refresh
          </button>
          <button onClick={async () => { if (confirm('STOP ALL AI ACTIVITY?')) { await api.emergencyStop(); refresh() } }} style={{ background: '#7f1d1d', border: '1px solid #991b1b', color: '#fca5a5', borderRadius: 6, padding: '8px 16px', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: 6, fontWeight: 600 }}>
            <AlertOctagon size={16} /> STOP ALL
          </button>
        </div>
      </header>

      {/* Tabs */}
      <nav style={{ display: 'flex', gap: 0, borderBottom: '1px solid #1a1a2e', padding: '0 24px' }}>
        {tabs.map(tab => (
          <button key={tab.id} onClick={() => setActiveTab(tab.id)}
            style={{
              padding: '12px 20px', border: 'none', background: 'none', cursor: 'pointer',
              color: activeTab === tab.id ? '#fff' : '#666',
              borderBottom: activeTab === tab.id ? '2px solid #3b82f6' : '2px solid transparent',
              fontWeight: activeTab === tab.id ? 600 : 400,
              display: 'flex', alignItems: 'center', gap: 6, fontSize: '14px',
            }}>
            <tab.icon size={16} />
            {tab.label}
          </button>
        ))}
      </nav>

      {/* Content */}
      <main style={{ padding: 24 }}>
        {activeTab === 'overview' && <OverviewTab data={data} />}
        {activeTab === 'agents' && <AgentsTab agents={data.agents} />}
        {activeTab === 'approvals' && <ApprovalsTab approvals={data.approvals} />}
        {activeTab === 'residuals' && <ResidualsTab residuals={data.residuals} />}
        {activeTab === 'receipts' && <ReceiptsTab receipts={data.receipts} />}
      </main>
    </div>
  )
}

function OverviewTab({ data }: any) {
  const cards = [
    { label: 'Active Agents', value: data.agents.length, icon: Users, color: '#3b82f6' },
    { label: 'Pending Approvals', value: data.approvals.filter((a: any) => a.status === 'pending').length, icon: FileCheck, color: '#f59e0b' },
    { label: 'Residuals', value: data.residuals.length, icon: AlertTriangle, color: '#ef4444' },
    { label: 'Receipts', value: data.receipts.length, icon: Shield, color: '#22c55e' },
  ]
  return (
    <div>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: 16, marginBottom: 24 }}>
        {cards.map(card => (
          <div key={card.label} style={{ background: '#12121a', border: `1px solid ${card.color}22`, borderRadius: 8, padding: 20 }}>
            <card.icon size={24} color={card.color} />
            <div style={{ fontSize: 32, fontWeight: 700, color: '#fff', marginTop: 8 }}>{card.value}</div>
            <div style={{ fontSize: 13, color: '#888', marginTop: 4 }}>{card.label}</div>
          </div>
        ))}
      </div>

      <h2 style={{ fontSize: 16, fontWeight: 600, color: '#fff', marginBottom: 12 }}>Recent Agents</h2>
      {data.agents.length === 0 ? <Empty>No agents registered</Empty> : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
          {data.agents.slice(0, 5).map((a: any) => (
            <div key={a.agent_id} style={{ background: '#12121a', border: '1px solid #1a1a2e', borderRadius: 8, padding: '12px 16px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <div style={{ fontWeight: 600, color: '#fff' }}>{a.display_name}</div>
                <div style={{ fontSize: 12, color: '#666' }}>{a.declared_purpose || a.provider}</div>
              </div>
              <StatusBadge status={a.status} />
            </div>
          ))}
        </div>
      )}

      <h2 style={{ fontSize: 16, fontWeight: 600, color: '#fff', marginTop: 24, marginBottom: 12 }}>Recent Residuals</h2>
      {data.residuals.length === 0 ? <Empty>No residuals detected</Empty> : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
          {data.residuals.slice(0, 5).map((r: any) => (
            <div key={r.residual_id} style={{ background: '#12121a', border: '1px solid #ef444422', borderRadius: 8, padding: '12px 16px' }}>
              <div style={{ fontWeight: 600, color: '#fca5a5', fontSize: 14 }}>{r.plain_language_summary}</div>
              <div style={{ fontSize: 12, color: '#666', marginTop: 4 }}>
                Severity: <span style={{ color: r.severity === 'Critical' ? '#ef4444' : '#f59e0b' }}>{r.severity}</span> | Response: {r.response}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

function AgentsTab({ agents }: any) {
  return agents.length === 0 ? <Empty>No agents</Empty> : (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
      {agents.map((a: any) => (
        <div key={a.agent_id} style={{ background: '#12121a', border: '1px solid #1a1a2e', borderRadius: 8, padding: 16 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'start' }}>
            <div>
              <div style={{ fontWeight: 600, color: '#fff', fontSize: 16 }}>{a.display_name}</div>
              <div style={{ fontSize: 13, color: '#888', marginTop: 2 }}>ID: {a.agent_id}</div>
              <div style={{ fontSize: 13, color: '#666', marginTop: 4 }}>Provider: {a.provider} | Purpose: {a.declared_purpose || 'none'}</div>
              {a.active_grants?.length > 0 && (
                <div style={{ marginTop: 8, display: 'flex', flexWrap: 'wrap', gap: 4 }}>
                  {a.active_grants.map((g: any, i: number) => (
                    <span key={i} style={{ background: '#1a2a1a', color: '#4ade80', padding: '2px 8px', borderRadius: 4, fontSize: 12 }}>
                      {g.action} → {g.resource}
                    </span>
                  ))}
                </div>
              )}
            </div>
            <StatusBadge status={a.status} />
          </div>
        </div>
      ))}
    </div>
  )
}

function ApprovalsTab({ approvals }: any) {
  return approvals.length === 0 ? <Empty>No pending approvals</Empty> : (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
      {approvals.map((a: any) => (
        <div key={a.approval_id} style={{ background: '#12121a', border: '1px solid #f59e0b22', borderRadius: 8, padding: 16 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div>
              <div style={{ fontWeight: 600, color: '#fff' }}>{a.agent_display_name}</div>
              <div style={{ fontSize: 13, color: '#f59e0b', marginTop: 2 }}>{a.action_description}</div>
              <div style={{ fontSize: 12, color: '#666', marginTop: 4 }}>Risk: {a.risk_level} | Status: {a.status}</div>
            </div>
            <StatusBadge status={a.status} />
          </div>
          {a.status === 'pending' && (
            <div style={{ marginTop: 12, display: 'flex', gap: 8 }}>
              <button style={{ background: '#166534', color: '#4ade80', border: '1px solid #166534', borderRadius: 6, padding: '6px 16px', cursor: 'pointer', fontWeight: 600, fontSize: 13 }}
                onClick={() => api.approveDecision(a.approval_id, 'ALLOW_ONCE')}>
                Allow Once
              </button>
              <button style={{ background: '#7f1d1d', color: '#fca5a5', border: '1px solid #7f1d1d', borderRadius: 6, padding: '6px 16px', cursor: 'pointer', fontWeight: 600, fontSize: 13 }}
                onClick={() => api.approveDecision(a.approval_id, 'DENY_ONCE')}>
                Deny
              </button>
              <button style={{ background: '#1a1a2e', color: '#e0e0e0', border: '1px solid #2a2a4e', borderRadius: 6, padding: '6px 16px', cursor: 'pointer', fontSize: 13 }}
                onClick={() => api.approveDecision(a.approval_id, 'STOP_AGENT')}>
                Stop Agent
              </button>
            </div>
          )}
        </div>
      ))}
    </div>
  )
}

function ResidualsTab({ residuals }: any) {
  return residuals.length === 0 ? <Empty>No residuals</Empty> : (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
      {residuals.map((r: any) => (
        <div key={r.residual_id} style={{ background: '#12121a', border: '1px solid #ef444422', borderRadius: 8, padding: 16 }}>
          <div style={{ fontWeight: 600, color: '#fca5a5' }}>{r.plain_language_summary}</div>
          <div style={{ fontSize: 13, color: '#888', marginTop: 4 }}>Type: {r.residual_type}</div>
          <div style={{ display: 'flex', gap: 8, marginTop: 8 }}>
            <span style={{ padding: '2px 8px', borderRadius: 4, fontSize: 12, background: r.severity === 'Critical' ? '#7f1d1d55' : '#78350f55', color: r.severity === 'Critical' ? '#fca5a5' : '#fbbf24' }}>
              {r.severity}
            </span>
            <span style={{ padding: '2px 8px', borderRadius: 4, fontSize: 12, background: '#1a1a2e', color: '#888' }}>{r.response}</span>
            {r.data_exposed && <span style={{ padding: '2px 8px', borderRadius: 4, fontSize: 12, background: '#7f1d1d55', color: '#ef4444' }}>DATA EXPOSED</span>}
          </div>
        </div>
      ))}
    </div>
  )
}

function ReceiptsTab({ receipts }: any) {
  return receipts.length === 0 ? <Empty>No receipts</Empty> : (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
      {receipts.map((r: any) => (
        <div key={r.receipt_id} style={{ background: '#12121a', border: '1px solid #1a1a2e', borderRadius: 8, padding: 16 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div>
              <div style={{ fontWeight: 600, color: '#fff' }}>{r.declared_purpose}</div>
              <div style={{ fontSize: 13, color: '#888', marginTop: 2 }}>Agent: {r.agent_id} | Node: {r.observer_node_id}</div>
              <div style={{ fontSize: 12, color: '#666', marginTop: 4 }}>
                {new Date(r.executed_at).toLocaleString()} | Outcome: <span style={{ color: r.outcome === 'Success' ? '#4ade80' : '#ef4444' }}>{r.outcome}</span>
              </div>
            </div>
            <div style={{ fontSize: 12, color: r.verified ? '#4ade80' : '#666' }}>
              {r.verified ? '✓ Verified' : 'Pending'}
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}

function StatusBadge({ status }: { status: string }) {
  const colors: any = { active: '#166534', stopped: '#78350f', revoked: '#7f1d1d', pending: '#78350f', approved: '#166534', denied: '#7f1d1d', executing: '#1e3a5f' }
  const labelColors: any = { active: '#4ade80', stopped: '#fbbf24', revoked: '#fca5a5', pending: '#fbbf24', approved: '#4ade80', denied: '#fca5a5', executing: '#60a5fa' }
  return <span style={{ background: colors[status] || '#1a1a2e', color: labelColors[status] || '#888', padding: '2px 10px', borderRadius: 12, fontSize: 12, fontWeight: 600 }}>{status}</span>
}

function Empty({ children }: { children: string }) {
  return <div style={{ textAlign: 'center', color: '#444', padding: 40, fontSize: 14 }}>{children}</div>
}
