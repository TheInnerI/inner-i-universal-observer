import express from 'express'
import cors from 'cors'
import { v4 as uuidv4 } from 'uuid'

// ============================================================
// Inner I Relay — Encrypted message relay for IIOP
// ============================================================
// Delivers encrypted approval requests, responses, alerts
// between Observer Nodes and Inner I Mobile devices.
// Does NOT hold private keys, cannot decrypt payloads,
// cannot approve actions, cannot modify policies.

const app = express()
app.use(cors())
app.use(express.json({ limit: '1mb' }))

const PORT = parseInt(process.env.PORT || '7413', 10)

// ---- In-memory stores ----

interface Device {
  deviceId: string
  deviceName: string
  deviceType: string
  publicKey: string
  registeredAt: number
  lastSeen: number
}

interface Message {
  messageId: string
  from: string
  to: string
  payload: string  // encrypted — relay cannot read
  timestamp: number
  delivered: boolean
}

const devices: Map<string, Device> = new Map()
const messages: Map<string, Message> = new Map()
const offlineQueue: Map<string, Message[]> = new Map()

// ---- Device Registration ----

app.post('/v1/devices/register', (req, res) => {
  const { deviceName, deviceType, publicKey } = req.body
  if (!deviceName || !publicKey) {
    return res.status(400).json({ error: 'deviceName and publicKey required' })
  }
  const deviceId = uuidv4()
  const now = Date.now()
  const device: Device = { deviceId, deviceName, deviceType: deviceType || 'unknown', publicKey, registeredAt: now, lastSeen: now }
  devices.set(deviceId, device)
  console.log(`Device registered: ${deviceName} (${deviceId})`)
  res.status(201).json({ deviceId, status: 'registered', registeredAt: now })
})

app.get('/v1/devices', (_req, res) => {
  res.json({ devices: Array.from(devices.values()).map(d => ({ deviceId: d.deviceId, deviceName: d.deviceName, deviceType: d.deviceType, lastSeen: d.lastSeen })) })
})

app.get('/v1/devices/:id', (req, res) => {
  const device = devices.get(req.params.id)
  if (!device) return res.status(404).json({ error: 'Device not found' })
  res.json({ deviceId: device.deviceId, deviceName: device.deviceName, deviceType: device.deviceType, publicKey: device.publicKey, registeredAt: device.registeredAt, lastSeen: device.lastSeen })
})

app.delete('/v1/devices/:id', (req, res) => {
  const existed = devices.delete(req.params.id)
  if (!existed) return res.status(404).json({ error: 'Device not found' })
  // Clean up messages for this device
  messages.forEach((msg, id) => { if (msg.to === req.params.id) messages.delete(id) })
  offlineQueue.delete(req.params.id)
  res.json({ status: 'revoked', deviceId: req.params.id })
})

// ---- Message Relay ----

app.post('/v1/messages', (req, res) => {
  const { from, to, payload } = req.body
  if (!from || !to || !payload) {
    return res.status(400).json({ error: 'from, to, and payload required' })
  }

  const messageId = uuidv4()
  const now = Date.now()

  // Check if target device is online (we ping heartbeat via lastSeen)
  const targetDevice = devices.get(to)
  const isOnline = targetDevice ? (now - targetDevice.lastSeen < 60000) : false // 60s heartbeat

  const message: Message = {
    messageId,
    from,
    to,
    payload, // encrypted — relay never reads
    timestamp: now,
    delivered: isOnline,
  }

  messages.set(messageId, message)

  if (!isOnline && targetDevice) {
    // Queue for offline delivery
    const queue = offlineQueue.get(to) || []
    queue.push(message)
    offlineQueue.set(to, queue)
    console.log(`Message ${messageId} queued for offline device ${to}`)
  }

  res.status(202).json({
    messageId,
    status: isOnline ? 'delivered' : 'queued',
    timestamp: now,
  })
})

// ---- Message Retrieval (polling) ----

app.get('/v1/messages/:deviceId', (req, res) => {
  const deviceId = req.params.deviceId

  // Update lastSeen
  const device = devices.get(deviceId)
  if (device) device.lastSeen = Date.now()

  // Deliver queued messages
  const queued = offlineQueue.get(deviceId) || []
  if (queued.length > 0) {
    queued.forEach(m => m.delivered = true)
    offlineQueue.delete(deviceId)
  }

  // Get all messages for this device
  const deviceMessages = Array.from(messages.values())
    .filter(m => m.to === deviceId)
    .sort((a, b) => b.timestamp - a.timestamp)
    .slice(0, 50)

  res.json({ messages: deviceMessages, queuedDelivered: queued.length })
})

// ---- Heartbeat ----

app.post('/v1/devices/:id/heartbeat', (req, res) => {
  const device = devices.get(req.params.id)
  if (!device) return res.status(404).json({ error: 'Device not found' })
  device.lastSeen = Date.now()
  res.json({ status: 'ok', lastSeen: device.lastSeen })
})

// ---- Health ----

app.get('/health', (_req, res) => {
  res.json({
    status: 'ok',
    service: 'inner-i-relay',
    version: '0.1.0',
    protocol: 'IIOP/0.1',
    devices: devices.size,
    queuedMessages: messages.size,
    uptime: process.uptime(),
  })
})

// ---- Start ----

app.listen(PORT, () => {
  console.log(`Inner I Relay listening on http://127.0.0.1:${PORT}`)
  console.log('Endpoints: /health, /v1/devices, /v1/messages')
  console.log('⚠️  Relay does NOT hold private keys or decrypt payloads')
})
