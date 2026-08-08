'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  Shield, Activity, Users, AlertTriangle, FileCheck, Zap, RefreshCw,
  AlertOctagon, Settings, Package, Server, Key, X, Menu, Radio,
  CheckCircle, XCircle, Clock, Play, Square, Trash2, Download, Search,
} from 'lucide-react'
import * as api from '@/lib/api'

type Tab = 'overview' | 'agents' | 'approvals' | 'residuals' | 'receipts' | 'proof' | 'settings'

export default function Dashboard() {
  const [data, setData] = useState<any>({ agents: [], approvals: [], residuals: [], receipts: [], consequences: [] })
  const [health, setHealth] = useState<'connected' | 'disconnected' | 'checking'>('checking')
  const [error, setError] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<Tab>('overview')
  const [sidebarOpen, setSidebarOpen] = useState(true)

  const refresh = useCallback(async () => {
    try {
      const h = await api.healthCheck()
      setHealth(h.status === 'ok' ? 'connected' : 'disconnected')
      const [agents, approvals, residuals, receipts] = await Promise.all([
        api.listAgents().catch(() => ({ agents: [] })),
        api.listApprovals().catch(() => ({ approvals: [] })),
        api.listResiduals().catch(() => ({ residuals: [] })),
        api.listReceipts().catch(() => ({ receipts: [] })),
      ])
      setData({ agents: agents.agents || [], approvals: approvals.approvals || [], residuals: residuals.residuals || [], receipts: receipts.receipts || [] })
      setError(null)
    } catch {
      setHealth('disconnected')
      setError('Cannot reach Observer Node at 127.0.0.1:7411')
    }
  }, [])

  useEffect(() => { refresh(); const i = setInterval(refresh, 5000); return () => clearInterval(i) }, [refresh])

  const pendingCount = data.approvals.filter((a: any) => a.status === 'pending').length
  const criticalCount = data.residuals.filter((r: any) => r.severity === 'Critical').length

  const tabs: { id: Tab; label: string; icon: any; badge?: number; badgeColor?: string }[] = [
    { id: 'overview', label: 'Overview', icon: Activity },
    { id: 'agents', label: 'Agents', icon: Users },
    { id: 'approvals', label: 'Approvals', icon: FileCheck, badge: pendingCount, badgeColor: '#f59e0b' },
    { id: 'residuals', label: 'Residuals', icon: AlertTriangle, badge: criticalCount, badgeColor: '#ef4444' },
    { id: 'receipts', label: 'Receipts', icon: Shield },
    { id: 'proof', label: 'Proof', icon: Package },
    { id: 'settings', label: 'Settings', icon: Settings },
  ]

  return (
    <div style={{ minHeight: '100vh', background: '#08080c', color: '#c8c8d0', fontFamily: 'system-ui, -apple-system, sans-serif', display: 'flex' }}>
      {/* Sidebar */}
      <aside style={{
        width: sidebarOpen ? 220 : 56, background: '#0a0a10', borderRight: '1px solid #161625',
        display: 'flex', flexDirection: 'column', transition: 'width 0.2s', overflow: 'hidden',
        flexShrink: 0,
      }}>
        {/* Logo */}
        <div style={{ padding: '16px', display: 'flex', alignItems: 'center', gap: 10, borderBottom: '1px solid #161625' }}>
          <Shield size={22} color="#3b82f6" />
          {sidebarOpen && <span style={{ fontWeight: 700, fontSize: 15, color: '#fff' }}>Inner I</span>}
          <button onClick={() => setSidebarOpen(!sidebarOpen)} style={{ marginLeft: 'auto', background: 'none', border: 'none', color: '#555', cursor: 'pointer' }}>
            <Menu size={16} />
          </button>
        </div>

        {/* Nav */}
        <nav style={{ flex: 1, padding: '8px 0' }}>
          {tabs.map(tab => (
            <button key={tab.id} onClick={() => setActiveTab(tab.id)}
              style={{
                width: '100%', display: 'flex', alignItems: 'center', gap: 10,
                padding: '10px 16px', border: 'none', background: activeTab === tab.id ? '#1a1a2e' : 'transparent',
                color: activeTab === tab.id ? '#fff' : '#666', cursor: 'pointer',
                borderLeft: activeTab === tab.id ? '3px solid #3b82f6' : '3px solid transparent',
                fontSize: 13, fontWeight: activeTab === tab.id ? 600 : 400,
                transition: 'all 0.15s',
              }}>
              <tab.icon size={17} />
              {sidebarOpen && <span style={{ flex: 1, textAlign: 'left' }}>{tab.label}</span>}
              {sidebarOpen && tab.badge ? <Badge label={String(tab.badge)} bg={tab.badgeColor + '22'} color={tab.badgeColor!} /> : null}
            </button>
          ))}
        </nav>

        {/* Status */}
        <div style={{ padding: '12px 16px', borderTop: '1px solid #161625', fontSize: 11 }}>
          {sidebarOpen && (
            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
              <div style={{ width: 8, height: 8, borderRadius: '50%', background: health === 'connected' ? '#22c55e' : health === 'checking' ? '#f59e0b' : '#ef4444' }} />
              <span style={{ color: '#555' }}>{health === 'connected' ? 'Node connected' : health === 'checking' ? 'Checking...' : 'Node offline'}</span>
            </div>
          )}
        </div>
      </aside>

      {/* Main */}
      <main style={{ flex: 1, display: 'flex', flexDirection: 'column', minWidth: 0 }}>
        {/* Top bar */}
        <header style={{ padding: '12px 24px', borderBottom: '1px solid #161625', display: 'flex', alignItems: 'center', justifyContent: 'space-between', background: '#0a0a10' }}>
          <div>
            <h1 style={{ fontSize: 18, fontWeight: 700, color: '#fff', margin: 0 }}>Inner I Control Center</h1>
            <span style={{ fontSize: 11, color: health === 'connected' ? '#4ade80' : '#f87171' }}>
              {health === 'connected' ? '● Observer Node online' : health === 'checking' ? '◐ Connecting...' : '○ Observer Node offline — start with: cargo run -p observer-node'}
            </span>
          </div>
          <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
            {error && <span style={{ fontSize: 11, color: '#f87171' }}>{error}</span>}
            <button onClick={refresh} style={btnStyle}>
              <RefreshCw size={14} /> Refresh
            </button>
            <button onClick={async () => { if (confirm('STOP ALL AI ACTIVITY? This revokes all grants.')) { await api.emergencyStop(); refresh() } }}
              style={{ ...btnStyle, background: '#7f1d1d', borderColor: '#991b1b', color: '#fca5a5', fontWeight: 600 }}>
              <AlertOctagon size={16} /> STOP ALL
            </button>
          </div>
        </header>

        {/* Content */}
        <div style={{ flex: 1, overflow: 'auto', padding: 24 }}>
          {activeTab === 'overview' && <OverviewTab data={data} />}
          {activeTab === 'agents' && <AgentsTab agents={data.agents} refresh={refresh} />}
          {activeTab === 'approvals' && <ApprovalsTab approvals={data.approvals} refresh={refresh} />}
          {activeTab === 'residuals' && <ResidualsTab residuals={data.residuals} />}
          {activeTab === 'receipts' && <ReceiptsTab receipts={data.receipts} />}
          {activeTab === 'proof' && <ProofTab />}
          {activeTab === 'settings' && <SettingsTab health={health} />}
        </div>
      </main>
    </div>
  )
}

// ── Components ──

const btnStyle: React.CSSProperties = {
  background: '#1a1a2e', border: '1px solid #2a2a4e', color: '#c8c8d0',
  borderRadius: 6, padding: '8px 14px', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: 6, fontSize: 13,
}

function Badge({ label, bg, color }: { label: string; bg: string; color: string }) {
  return <span style={{ background: bg, color, padding: '1px 7px', borderRadius: 10, fontSize: 11, fontWeight: 600 }}>{label}</span>
}

function StatusBadge({ status }: { status: string }) {
  const colors: any = { active: '#166534', stopped: '#78350f', revoked: '#7f1d1d', pending: '#713f12', approved: '#166534', denied: '#7f1d1d', executing: '#1e3a5f', success: '#166534', failure: '#7f1d1d' }
  const labels: any = { active: '#4ade80', stopped: '#fbbf24', revoked: '#fca5a5', pending: '#fbbf24', approved: '#4ade80', denied: '#fca5a5', executing: '#60a5fa', success: '#4ade80', failure: '#fca5a5' }
  return <span style={{ background: colors[status] || '#1a1a2e', color: labels[status] || '#888', padding: '2px 10px', borderRadius: 12, fontSize: 12, fontWeight: 600, whiteSpace: 'nowrap' }}>{status}</span>
}

function Empty({ children }: { children: string }) {
  return <div style={{ textAlign: 'center', color: '#333', padding: 60, fontSize: 14, fontStyle: 'italic' }}>{children}</div>
}

// ── Tab: Overview ──
function OverviewTab({ data }: any) {
  const cards = [
    { label: 'Active Agents', value: data.agents.filter((a: any) => a.status === 'active').length, icon: Users, color: '#3b82f6' },
    { label: 'Pending Approvals', value: data.approvals.filter((a: any) => a.status === 'pending').length, icon: FileCheck, color: '#f59e0b' },
    { label: 'Residuals', value: data.residuals.length, icon: AlertTriangle, color: '#ef4444' },
    { label: 'Receipts', value: data.receipts.length, icon: Shield, color: '#22c55e' },
  ]
  return (
    <div>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))', gap: 16, marginBottom: 28 }}>
        {cards.map(c => (
          <div key={c.label} style={{ background: '#0e0e16', border: `1px solid ${c.color}22`, borderRadius: 10, padding: 20 }}>
            <c.icon size={24} color={c.color} />
            <div style={{ fontSize: 36, fontWeight: 700, color: '#fff', marginTop: 10 }}>{c.value}</div>
            <div style={{ fontSize: 12, color: '#666', marginTop: 4 }}>{c.label}</div>
          </div>
        ))}
      </div>

      {data.agents.length > 0 && (
        <>
          <SectionHeader>Active Agents</SectionHeader>
          {data.agents.filter((a: any) => a.status === 'active').slice(0, 5).map((a: any) => (
            <Row key={a.agent_id} left={<><strong style={{ color: '#fff' }}>{a.display_name}</strong><span style={{ fontSize: 11, color: '#555', display: 'block' }}>{a.declared_purpose || a.provider}</span></>} right={<StatusBadge status={a.status} />} />
          ))}
        </>
      )}

      {data.residuals.length > 0 && (
        <>
          <SectionHeader style={{ marginTop: 24 }}>Recent Residuals</SectionHeader>
          {data.residuals.slice(0, 5).map((r: any) => (
            <Row key={r.residual_id} left={<><span style={{ color: '#fca5a5', fontSize: 13, fontWeight: 600 }}>{r.plain_language_summary}</span><span style={{ fontSize: 11, color: '#555', display: 'block' }}>Severity: <span style={{ color: r.severity === 'Critical' ? '#ef4444' : '#f59e0b' }}>{r.severity}</span> • {r.response}</span></>} />
          ))}
        </>
      )}
    </div>
  )
}

// ── Tab: Agents ──
function AgentsTab({ agents, refresh }: any) {
  const [showForm, setShowForm] = useState(false)
  const [form, setForm] = useState({ display_name: '', provider: 'demo', declared_purpose: '' })

  if (agents.length === 0 && !showForm) return <Empty>No agents registered. Register one below.</Empty>
  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
        <SectionHeader style={{ margin: 0 }}>Registered Agents</SectionHeader>
        <button onClick={() => setShowForm(!showForm)} style={{ ...btnStyle, background: '#3b82f6', color: '#fff', borderColor: '#3b82f6', fontWeight: 600 }}>
          + Register Agent
        </button>
      </div>

      {showForm && (
        <div style={{ background: '#0e0e16', border: '1px solid #1a1a2e', borderRadius: 10, padding: 20, marginBottom: 16 }}>
          <input placeholder="Agent name" value={form.display_name} onChange={e => setForm({ ...form, display_name: e.target.value })}
            style={inputStyle} />
          <input placeholder="Purpose" value={form.declared_purpose} onChange={e => setForm({ ...form, declared_purpose: e.target.value })}
            style={{ ...inputStyle, marginTop: 8 }} />
          <button onClick={async () => { await api.registerAgent(form); setShowForm(false); setForm({ display_name: '', provider: 'demo', declared_purpose: '' }); refresh() }}
            style={{ ...btnStyle, background: '#166534', color: '#4ade80', borderColor: '#166534', marginTop: 8, fontWeight: 600 }}>
            Register
          </button>
        </div>
      )}

      {agents.map((a: any) => (
        <Row key={a.agent_id}
          left={<><strong style={{ color: '#fff' }}>{a.display_name}</strong><span style={{ fontSize: 11, color: '#555', display: 'block' }}>ID: {a.agent_id.slice(0, 8)}... • {a.provider} • {a.declared_purpose || 'No purpose'}</span>
            {a.active_grants?.length > 0 && <div style={{ marginTop: 6, display: 'flex', flexWrap: 'wrap', gap: 4 }}>{a.active_grants.map((g: any, i: number) => <Badge key={i} label={`${g.action}:${g.resource}`} bg="#1a2a1a" color="#4ade80" />)}</div>}
          </>}
          right={<div style={{ display: 'flex', gap: 8, alignItems: 'center' }}><StatusBadge status={a.status} />{a.status === 'active' && <button onClick={async () => { await api.stopAgent(a.agent_id); refresh() }} style={{ background: '#7f1d1d22', border: 'none', color: '#fca5a5', cursor: 'pointer', borderRadius: 6, padding: '4px 10px', fontSize: 12 }}><Square size={14} /> Stop</button>}</div>}
        />
      ))}
    </div>
  )
}

// ── Tab: Approvals ──
function ApprovalsTab({ approvals, refresh }: any) {
  if (approvals.length === 0) return <Empty>No approvals. Request a capability from an agent.</Empty>
  return (
    <div>
      <SectionHeader>Approval Requests</SectionHeader>
      {approvals.map((a: any) => (
        <Row key={a.approval_id}
          left={<><strong style={{ color: '#fff' }}>{a.agent_display_name}</strong><span style={{ color: '#f59e0b', fontSize: 12, display: 'block' }}>{a.action_description}</span><span style={{ fontSize: 11, color: '#555' }}>Risk: {a.risk_level}</span></>}
          right={<div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
            <StatusBadge status={a.status} />
            {a.status === 'pending' && <>
              <button onClick={async () => { await api.approveDecision(a.approval_id, 'ALLOW_ONCE'); refresh() }} style={{ background: '#166534', border: 'none', color: '#4ade80', borderRadius: 6, padding: '5px 12px', cursor: 'pointer', fontSize: 12, fontWeight: 600 }}><CheckCircle size={14} /> Allow</button>
              <button onClick={async () => { await api.approveDecision(a.approval_id, 'DENY_ONCE'); refresh() }} style={{ background: '#7f1d1d', border: 'none', color: '#fca5a5', borderRadius: 6, padding: '5px 12px', cursor: 'pointer', fontSize: 12, fontWeight: 600 }}><XCircle size={14} /> Deny</button>
            </>}
          </div>}
        />
      ))}
    </div>
  )
}

// ── Tab: Residuals ──
function ResidualsTab({ residuals }: any) {
  if (residuals.length === 0) return <Empty>No residuals detected. System is clean.</Empty>
  return (
    <div>
      <SectionHeader>Detected Residuals</SectionHeader>
      {residuals.map((r: any) => (
        <Row key={r.residual_id}
          left={<><strong style={{ color: '#fca5a5', fontSize: 14 }}>{r.plain_language_summary}</strong><span style={{ fontSize: 11, color: '#555', display: 'block' }}>Type: {r.residual_type} • {new Date(r.detected_at).toLocaleString()}</span></>}
          right={<div style={{ display: 'flex', gap: 4, flexDirection: 'column', alignItems: 'flex-end' }}>
            <Badge label={r.severity} bg={r.severity === 'Critical' ? '#7f1d1d55' : '#78350f55'} color={r.severity === 'Critical' ? '#ef4444' : '#fbbf24'} />
            <Badge label={r.response} bg="#1a1a2e" color="#888" />
            {r.data_exposed && <Badge label="DATA EXPOSED" bg="#7f1d1d55" color="#ef4444" />}
          </div>}
        />
      ))}
    </div>
  )
}

// ── Tab: Receipts ──
function ReceiptsTab({ receipts }: any) {
  if (receipts.length === 0) return <Empty>No receipts. Execute an approved action to generate one.</Empty>
  return (
    <div>
      <SectionHeader>Execution Receipts</SectionHeader>
      {receipts.map((r: any) => (
        <Row key={r.receipt_id}
          left={<><strong style={{ color: '#fff' }}>{r.declared_purpose}</strong><span style={{ fontSize: 11, color: '#555', display: 'block' }}>Agent: {r.agent_id?.slice(0, 8)}... • {new Date(r.executed_at).toLocaleString()}</span></>}
          right={<div style={{ display: 'flex', gap: 6, alignItems: 'center' }}><StatusBadge status={r.outcome || 'success'} /><Badge label="✓ Verified" bg="#16653444" color="#4ade80" /></div>}
        />
      ))}
    </div>
  )
}

// ── Tab: Proof ──
function ProofTab() {
  const [result, setResult] = useState<any>(null)
  const handleExport = async () => { const r = await api.exportProof(); setResult(r) }
  return (
    <div>
      <SectionHeader>Proof Bundles</SectionHeader>
      <p style={{ color: '#555', fontSize: 13, marginBottom: 16, lineHeight: 1.6 }}>
        Export a cryptographically signed Proof Bundle containing all receipts, residuals, consequences, and approval decisions.
        Bundles can be independently verified without the original Observer Node.
      </p>
      <button onClick={handleExport} style={{ ...btnStyle, background: '#3b82f6', color: '#fff', borderColor: '#3b82f6', fontWeight: 600, padding: '12px 24px', fontSize: 15 }}>
        <Download size={18} /> Export Proof Bundle
      </button>
      {result && (
        <div style={{ marginTop: 16, background: '#0e0e16', border: '1px solid #22c55e22', borderRadius: 10, padding: 20 }}>
          <strong style={{ color: '#4ade80' }}>✓ Bundle Exported</strong>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(140px, 1fr))', gap: 12, marginTop: 12 }}>
            <Stat label="Bundle ID" value={result.bundle_id?.slice(0, 8) + '...'} />
            <Stat label="Receipts" value={String(result.receipts)} />
            <Stat label="Residuals" value={String(result.residuals)} />
            <Stat label="Status" value={result.status} />
          </div>
        </div>
      )}
    </div>
  )
}

// ── Tab: Settings ──
function SettingsTab({ health }: { health: string }) {
  return (
    <div>
      <SectionHeader>System Settings</SectionHeader>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 0 }}>
        <SettingRow label="Observer Node" value={health === 'connected' ? 'Connected' : 'Offline'} valueColor={health === 'connected' ? '#4ade80' : '#f87171'} />
        <SettingRow label="Node URL" value="http://127.0.0.1:7411" />
        <SettingRow label="Protection Level" value="Ask Me (default)" />
        <SettingRow label="Relay" value="Not configured" />
        <SettingRow label="Database" value="SQLite (data/observer.db)" />
        <SettingRow label="IIOP Protocol" value="v0.1" />
        <SettingRow label="Identity" value="Ed25519" />
        <SettingRow label="Version" value="0.1.0" />
      </div>
      <div style={{ marginTop: 24 }}>
        <SectionHeader>About</SectionHeader>
        <p style={{ color: '#555', fontSize: 13, lineHeight: 1.6 }}>
          Inner I Universal Observer is a universal permission, proof, and consequence layer for personal AI.
          Built by inneri76. The tools are yours.
        </p>
        <p style={{ color: '#444', fontSize: 11, marginTop: 8 }}>Contact: i@innerinetcompany.com • GitHub: TheInnerI/inner-i-universal-observer</p>
      </div>
    </div>
  )
}

// ── Helpers ──

function SectionHeader({ children, style }: { children: string; style?: React.CSSProperties }) {
  return <h2 style={{ fontSize: 15, fontWeight: 600, color: '#fff', marginBottom: 12, ...style }}>{children}</h2>
}

function Row({ left, right }: { left: React.ReactNode; right?: React.ReactNode }) {
  return (
    <div style={{ background: '#0e0e16', border: '1px solid #161625', borderRadius: 8, padding: '14px 18px', marginBottom: 8, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
      <div style={{ flex: 1 }}>{left}</div>
      {right && <div style={{ flexShrink: 0, marginLeft: 12 }}>{right}</div>}
    </div>
  )
}

function Stat({ label, value }: { label: string; value: string }) {
  return <div><div style={{ fontSize: 11, color: '#555' }}>{label}</div><div style={{ color: '#fff', fontWeight: 600, marginTop: 2 }}>{value}</div></div>
}

function SettingRow({ label, value, valueColor }: { label: string; value: string; valueColor?: string }) {
  return (
    <div style={{ display: 'flex', justifyContent: 'space-between', padding: '14px 0', borderBottom: '1px solid #161625', fontSize: 13 }}>
      <span style={{ color: '#888' }}>{label}</span>
      <span style={{ color: valueColor || '#fff', fontWeight: 500 }}>{value}</span>
    </div>
  )
}

const inputStyle: React.CSSProperties = {
  width: '100%', background: '#0a0a10', border: '1px solid #1a1a2e', borderRadius: 6,
  padding: '10px 14px', color: '#fff', fontSize: 14, outline: 'none', boxSizing: 'border-box',
}