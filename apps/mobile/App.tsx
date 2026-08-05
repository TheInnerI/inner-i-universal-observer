import React, { useState, useEffect, useCallback } from 'react'
import {
  View, Text, TouchableOpacity, ScrollView, StyleSheet, ActivityIndicator,
  RefreshControl, Alert, SafeAreaView, StatusBar,
} from 'react-native'
import * as api from '../lib/api'

interface Agent { agent_id: string; display_name: string; provider: string; status: string; declared_purpose?: string }
interface Approval { approval_id: string; agent_display_name: string; action_description: string; risk_level: string; status: string }
interface Residual { residual_id: string; plain_language_summary: string; severity: string; response: string }
interface Receipt { receipt_id: string; declared_purpose: string; outcome: string; executed_at: number }

type Screen = 'home' | 'approvals' | 'agents' | 'activity' | 'residuals' | 'proof' | 'settings'

export default function App() {
  const [screen, setScreen] = useState<Screen>('home')
  const [health, setHealth] = useState<string>('...')
  const [agents, setAgents] = useState<Agent[]>([])
  const [approvals, setApprovals] = useState<Approval[]>([])
  const [residuals, setResiduals] = useState<Residual[]>([])
  const [receipts, setReceipts] = useState<Receipt[]>([])
  const [loading, setLoading] = useState(false)
  const [refreshing, setRefreshing] = useState(false)

  const fetchData = useCallback(async () => {
    try {
      const h = await api.healthCheck()
      setHealth(h.status)
      const [a, ap, r, rc] = await Promise.all([
        api.listAgents().catch(() => ({ agents: [] })),
        api.listApprovals().catch(() => ({ approvals: [] })),
        api.listResiduals().catch(() => ({ residuals: [] })),
        api.listReceipts().catch(() => ({ receipts: [] })),
      ])
      setAgents(a.agents || [])
      setApprovals(ap.approvals || [])
      setResiduals(r.residuals || [])
      setReceipts(rc.receipts || [])
    } catch (e: any) {
      setHealth('disconnected')
    }
  }, [])

  useEffect(() => { fetchData(); const i = setInterval(fetchData, 5000); return () => clearInterval(i) }, [fetchData])

  const onRefresh = async () => { setRefreshing(true); await fetchData(); setRefreshing(false) }

  const pendingCount = approvals.filter(a => a.status === 'pending').length
  const criticalResiduals = residuals.filter(r => r.severity === 'Critical').length

  const tabs: { key: Screen; label: string; badge?: number; color?: string }[] = [
    { key: 'home', label: 'Home' },
    { key: 'approvals', label: 'Approvals', badge: pendingCount, color: pendingCount > 0 ? '#f59e0b' : undefined },
    { key: 'agents', label: 'Agents' },
    { key: 'activity', label: 'Activity' },
    { key: 'residuals', label: 'Residuals', badge: criticalResiduals, color: criticalResiduals > 0 ? '#ef4444' : undefined },
    { key: 'proof', label: 'Proof' },
    { key: 'settings', label: 'Settings' },
  ]

  return (
    <SafeAreaView style={s.container}>
      <StatusBar barStyle="light-content" backgroundColor="#0a0a0f" />
      {/* Header */}
      <View style={s.header}>
        <View>
          <Text style={s.title}>Inner I</Text>
          <Text style={s.subtitle}>Observer: {health}</Text>
        </View>
        <TouchableOpacity style={s.stopButton} onPress={() => {
          Alert.alert('STOP ALL AI', 'This will stop all agents and revoke all grants.', [
            { text: 'Cancel', style: 'cancel' },
            { text: 'STOP ALL', style: 'destructive', onPress: async () => { await api.emergencyStop(); fetchData() } },
          ])
        }}>
          <Text style={s.stopText}>STOP ALL</Text>
        </TouchableOpacity>
      </View>

      {/* Tab Bar */}
      <View style={s.tabBar}>
        {tabs.map(tab => (
          <TouchableOpacity key={tab.key} style={s.tab} onPress={() => setScreen(tab.key)}>
            <Text style={[s.tabText, screen === tab.key && s.tabActive]}>{tab.label}</Text>
            {tab.badge ? <View style={[s.badge, { backgroundColor: tab.color || '#f59e0b' }]}><Text style={s.badgeText}>{tab.badge}</Text></View> : null}
          </TouchableOpacity>
        ))}
      </View>

      {/* Content */}
      <ScrollView style={s.content} refreshControl={<RefreshControl refreshing={refreshing} onRefresh={onRefresh} tintColor="#888" />}>
        {screen === 'home' && <HomeScreen agents={agents} approvals={approvals} residuals={residuals} pendingCount={pendingCount} />}
        {screen === 'approvals' && <ApprovalsScreen approvals={approvals} fetchData={fetchData} />}
        {screen === 'agents' && <AgentsScreen agents={agents} fetchData={fetchData} />}
        {screen === 'activity' && <ActivityScreen receipts={receipts} />}
        {screen === 'residuals' && <ResidualsScreen residuals={residuals} />}
        {screen === 'proof' && <ProofScreen fetchData={fetchData} />}
        {screen === 'settings' && <SettingsScreen />}
      </ScrollView>
    </SafeAreaView>
  )
}

function HomeScreen({ agents, approvals, residuals, pendingCount }: any) {
  return (
    <View style={{ padding: 16 }}>
      {/* Cards */}
      <View style={{ flexDirection: 'row', flexWrap: 'wrap', gap: 12, marginBottom: 20 }}>
        <StatCard label="Active Agents" value={agents.length} color="#3b82f6" />
        <StatCard label="Pending" value={pendingCount} color="#f59e0b" />
        <StatCard label="Residuals" value={residuals.length} color="#ef4444" />
      </View>

      {/* Pending Approvals */}
      <Text style={{ color: '#fff', fontSize: 16, fontWeight: '600', marginBottom: 8 }}>
        {pendingCount > 0 ? `⚠️ ${pendingCount} Approval${pendingCount > 1 ? 's' : ''} Needed` : '✓ No pending approvals'}
      </Text>
      {approvals.filter((a: any) => a.status === 'pending').slice(0, 3).map((a: Approval) => (
        <View key={a.approval_id} style={{ backgroundColor: '#12121a', borderRadius: 8, padding: 12, marginBottom: 8, borderWidth: 1, borderColor: '#f59e0b22' }}>
          <Text style={{ color: '#fff', fontWeight: '600' }}>{a.agent_display_name}</Text>
          <Text style={{ color: '#f59e0b', fontSize: 13, marginTop: 2 }}>{a.action_description}</Text>
          <Text style={{ color: '#666', fontSize: 12, marginTop: 4 }}>Risk: {a.risk_level}</Text>
        </View>
      ))}

      {/* Recent Residuals */}
      {residuals.slice(0, 3).map((r: Residual) => (
        <View key={r.residual_id} style={{ backgroundColor: '#12121a', borderRadius: 8, padding: 12, marginBottom: 8, borderWidth: 1, borderColor: '#ef444422' }}>
          <Text style={{ color: '#fca5a5', fontSize: 13, fontWeight: '600' }}>{r.plain_language_summary}</Text>
          <Text style={{ color: '#666', fontSize: 12, marginTop: 4 }}>{r.severity} | {r.response}</Text>
        </View>
      ))}
    </View>
  )
}

function StatCard({ label, value, color }: { label: string; value: number; color: string }) {
  return (
    <View style={{ flex: 1, minWidth: 100, backgroundColor: '#12121a', borderRadius: 8, padding: 16, borderWidth: 1, borderColor: color + '22' }}>
      <Text style={{ color: '#fff', fontSize: 28, fontWeight: '700' }}>{value}</Text>
      <Text style={{ color: '#888', fontSize: 13, marginTop: 2 }}>{label}</Text>
    </View>
  )
}

function ApprovalsScreen({ approvals, fetchData }: any) {
  const handleDecision = async (id: string, decision: string) => {
    await api.approveDecision(id, decision)
    fetchData()
  }

  if (approvals.length === 0) return <Empty>No approvals</Empty>

  return (
    <View style={{ padding: 16 }}>
      {approvals.map((a: Approval) => (
        <View key={a.approval_id} style={{ backgroundColor: '#12121a', borderRadius: 8, padding: 16, marginBottom: 8, borderWidth: 1, borderColor: a.status === 'pending' ? '#f59e0b44' : '#1a1a2e' }}>
          <Text style={{ color: '#fff', fontWeight: '600', fontSize: 15 }}>{a.agent_display_name}</Text>
          <Text style={{ color: '#f59e0b', fontSize: 13, marginTop: 4 }}>{a.action_description}</Text>
          <View style={{ flexDirection: 'row', gap: 8, marginTop: 6 }}>
            <Badge label={a.risk_level} bg="#78350f55" color={a.risk_level === 'high' ? '#ef4444' : '#fbbf24'} />
            <Badge label={a.status} bg="#1a1a2e" color="#888" />
          </View>
          {a.status === 'pending' && (
            <View style={{ flexDirection: 'row', gap: 8, marginTop: 12 }}>
              <TouchableOpacity style={{ flex: 1, backgroundColor: '#166534', borderRadius: 8, padding: 10, alignItems: 'center' }} onPress={() => handleDecision(a.approval_id, 'ALLOW_ONCE')}>
                <Text style={{ color: '#4ade80', fontWeight: '600' }}>Allow</Text>
              </TouchableOpacity>
              <TouchableOpacity style={{ flex: 1, backgroundColor: '#7f1d1d', borderRadius: 8, padding: 10, alignItems: 'center' }} onPress={() => handleDecision(a.approval_id, 'DENY_ONCE')}>
                <Text style={{ color: '#fca5a5', fontWeight: '600' }}>Deny</Text>
              </TouchableOpacity>
              <TouchableOpacity style={{ flex: 1, backgroundColor: '#1a1a2e', borderRadius: 8, padding: 10, alignItems: 'center' }} onPress={() => handleDecision(a.approval_id, 'STOP_AGENT')}>
                <Text style={{ color: '#e0e0e0', fontWeight: '600' }}>Stop</Text>
              </TouchableOpacity>
            </View>
          )}
        </View>
      ))}
    </View>
  )
}

function AgentsScreen({ agents, fetchData }: any) {
  if (agents.length === 0) return <Empty>No agents</Empty>
  return (
    <View style={{ padding: 16 }}>
      {agents.map((a: Agent) => (
        <View key={a.agent_id} style={{ backgroundColor: '#12121a', borderRadius: 8, padding: 16, marginBottom: 8, borderWidth: 1, borderColor: '#1a1a2e' }}>
          <View style={{ flexDirection: 'row', justifyContent: 'space-between' }}>
            <View style={{ flex: 1 }}>
              <Text style={{ color: '#fff', fontWeight: '600', fontSize: 15 }}>{a.display_name}</Text>
              <Text style={{ color: '#888', fontSize: 12, marginTop: 2 }}>Provider: {a.provider}</Text>
              <Text style={{ color: '#666', fontSize: 12, marginTop: 2 }}>{a.declared_purpose || 'No purpose declared'}</Text>
            </View>
            <Badge label={a.status} bg="#1a1a2e" color={a.status === 'active' ? '#4ade80' : '#888'} />
          </View>
          <TouchableOpacity style={{ marginTop: 12, backgroundColor: '#7f1d1d22', borderRadius: 6, padding: 8, alignItems: 'center' }}
            onPress={async () => { await api.stopAgent(a.agent_id); fetchData() }}>
            <Text style={{ color: '#fca5a5', fontSize: 13 }}>Stop Agent</Text>
          </TouchableOpacity>
        </View>
      ))}
    </View>
  )
}

function ActivityScreen({ receipts }: any) {
  if (receipts.length === 0) return <Empty>No activity</Empty>
  return (
    <View style={{ padding: 16 }}>
      {receipts.map((r: Receipt) => (
        <View key={r.receipt_id} style={{ backgroundColor: '#12121a', borderRadius: 8, padding: 12, marginBottom: 8, borderWidth: 1, borderColor: '#1a1a2e' }}>
          <Text style={{ color: '#fff', fontSize: 14, fontWeight: '600' }}>{r.declared_purpose}</Text>
          <View style={{ flexDirection: 'row', gap: 8, marginTop: 6 }}>
            <Badge label={r.outcome} bg={r.outcome === 'Success' ? '#16653444' : '#7f1d1d44'} color={r.outcome === 'Success' ? '#4ade80' : '#ef4444'} />
            <Text style={{ color: '#444', fontSize: 12, alignSelf: 'center' }}>{new Date(r.executed_at).toLocaleString()}</Text>
          </View>
        </View>
      ))}
    </View>
  )
}

function ResidualsScreen({ residuals }: any) {
  if (residuals.length === 0) return <Empty>No residuals — clean</Empty>
  return (
    <View style={{ padding: 16 }}>
      {residuals.map((r: Residual) => (
        <View key={r.residual_id} style={{ backgroundColor: '#12121a', borderRadius: 8, padding: 16, marginBottom: 8, borderWidth: 1, borderColor: r.severity === 'Critical' ? '#ef444444' : '#f59e0b22' }}>
          <Text style={{ color: '#fca5a5', fontSize: 14, fontWeight: '600' }}>{r.plain_language_summary}</Text>
          <View style={{ flexDirection: 'row', gap: 8, marginTop: 8 }}>
            <Badge label={r.severity} bg={r.severity === 'Critical' ? '#7f1d1d55' : '#78350f55'} color={r.severity === 'Critical' ? '#ef4444' : '#fbbf24'} />
            <Badge label={r.response} bg="#1a1a2e" color="#888" />
          </View>
        </View>
      ))}
    </View>
  )
}

function ProofScreen({ fetchData }: any) {
  const handleExport = async () => {
    const result = await api.exportProof()
    Alert.alert('Proof Exported', `Bundle ID: ${result.bundle_id}\nReceipts: ${result.receipts}\nResiduals: ${result.residuals}`)
    fetchData()
  }
  return (
    <View style={{ padding: 16 }}>
      <Text style={{ color: '#fff', fontSize: 16, fontWeight: '600', marginBottom: 12 }}>Proof & Verification</Text>
      <Text style={{ color: '#888', fontSize: 13, marginBottom: 16 }}>
        Export a signed Proof Bundle containing all receipts, residuals, consequences, and approval decisions. Bundles can be independently verified without the original Observer Node.
      </Text>
      <TouchableOpacity style={{ backgroundColor: '#3b82f6', borderRadius: 8, padding: 14, alignItems: 'center' }} onPress={handleExport}>
        <Text style={{ color: '#fff', fontWeight: '600', fontSize: 16 }}>Export Proof Bundle</Text>
      </TouchableOpacity>
    </View>
  )
}

function SettingsScreen() {
  return (
    <View style={{ padding: 16 }}>
      <Text style={{ color: '#fff', fontSize: 16, fontWeight: '600', marginBottom: 12 }}>Settings</Text>
      <SettingRow label="Observer Identity" value="Active" />
      <SettingRow label="Biometric Approval" value="Enabled" />
      <SettingRow label="Protection Level" value="Ask Me" />
      <SettingRow label="Paired Devices" value="0" />
      <SettingRow label="Relay" value="Not configured" />
      <SettingRow label="Local Only Mode" value="On" />
    </View>
  )
}

function SettingRow({ label, value }: { label: string; value: string }) {
  return (
    <View style={{ flexDirection: 'row', justifyContent: 'space-between', paddingVertical: 12, borderBottomWidth: 1, borderBottomColor: '#1a1a2e' }}>
      <Text style={{ color: '#888', fontSize: 14 }}>{label}</Text>
      <Text style={{ color: '#fff', fontSize: 14 }}>{value}</Text>
    </View>
  )
}

function Badge({ label, bg, color }: { label: string; bg: string; color: string }) {
  return (
    <View style={{ backgroundColor: bg, borderRadius: 6, paddingHorizontal: 8, paddingVertical: 2 }}>
      <Text style={{ color, fontSize: 11, fontWeight: '600' }}>{label}</Text>
    </View>
  )
}

function Empty({ children }: { children: string }) {
  return (
    <View style={{ padding: 40, alignItems: 'center' }}>
      <Text style={{ color: '#444', fontSize: 14 }}>{children}</Text>
    </View>
  )
}

const s = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#0a0a0f' },
  header: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', paddingHorizontal: 16, paddingVertical: 12, borderBottomWidth: 1, borderBottomColor: '#1a1a2e' },
  title: { color: '#fff', fontSize: 22, fontWeight: '700' },
  subtitle: { color: '#4ade80', fontSize: 12, marginTop: 2 },
  stopButton: { backgroundColor: '#7f1d1d', borderRadius: 8, paddingHorizontal: 16, paddingVertical: 10 },
  stopText: { color: '#fca5a5', fontWeight: '700', fontSize: 13 },
  tabBar: { flexDirection: 'row', borderBottomWidth: 1, borderBottomColor: '#1a1a2e', paddingHorizontal: 8 },
  tab: { flex: 1, alignItems: 'center', paddingVertical: 12, position: 'relative' },
  tabText: { color: '#666', fontSize: 12, fontWeight: '500' },
  tabActive: { color: '#fff', fontWeight: '600' },
  badge: { position: 'absolute', top: 6, right: 8, borderRadius: 8, paddingHorizontal: 5, paddingVertical: 1, minWidth: 16, alignItems: 'center' },
  badgeText: { color: '#000', fontSize: 10, fontWeight: '700' },
  content: { flex: 1 },
})
