# Inner I Universal Observer

> **One person. One Observer identity. Every machine accountable.**

Inner I gives every person one simple place to see, approve, block, stop, reverse, and verify what AI systems do in their name.

**The central principle:** AI may act for a person, but it must never become invisible to the person.

**The product promise:** Every machine acting in your name must ask permission, stay within its boundaries, leave a receipt, and answer for the consequences.

---

## What This Is

A universal permission, proof, and consequence layer for personal AI. Four products in one monorepo:

| Product | Type | Stack | Status |
|---------|------|-------|--------|
| **Inner I Mobile** | Phone-first approval & proof app | React Native / Expo | Scaffolded |
| **Inner I Observer Node** | Local enforcement engine | Rust | Live API (30 endpoints) |
| **Inner I Control Center** | Advanced web dashboard | Next.js / TypeScript | Scaffolded |
| **Inner I Relay** | Optional encrypted relay | Express / TypeScript | Scaffolded |

Connected through **IIOP** — the Inner I Observer Protocol (24 message types).

---

## What Works Now

### Rust Engine (35 tests, zero errors)

| Crate | Purpose | Tests |
|-------|---------|-------|
| `crypto-signing` | Ed25519 identity, signing, verification | 12/12 |
| `policy-engine` | YAML policy loader, glob-pattern evaluator, 4 protection levels | 6/6 |
| `capability-broker` | Request evaluation, grant management, approval/denial | 10/10 |
| `residual-engine` | Violation detection, intent divergence, credential access | 4/4 |
| `proof-store` | Signed receipts, Proof Bundles, tamper detection | 3/3 |

### Observer Node API (30 REST endpoints)

```
GET    /health
GET    /version
POST   /v1/agents                    — Register agent
GET    /v1/agents                    — List all agents
GET    /v1/agents/{id}              — Get agent + active grants
POST   /v1/agents/{id}/stop         — Stop agent, revoke all grants
POST   /v1/agents/{id}/revoke       — Revoke agent
POST   /v1/capabilities/request      — Request capability (returns allow/deny/pending)
POST   /v1/capabilities/revoke       — Revoke grants
GET    /v1/approvals                 — List pending approvals
POST   /v1/approvals/{id}/decision   — Approve or deny
POST   /v1/executions                — Start monitored execution
GET    /v1/executions/{id}          — Get execution status
POST   /v1/executions/{id}/pause    — Pause execution
POST   /v1/executions/{id}/stop     — Stop execution
POST   /v1/executions/{id}/rollback — Request rollback
GET    /v1/residuals                 — List all violation records
POST   /v1/consequences              — Record consequence
GET    /v1/receipts                  — List signed receipts
POST   /v1/proofs/export             — Export Proof Bundle
POST   /v1/proofs/verify             — Independently verify Proof Bundle
POST   /v1/emergency-stop            — Stop all agents, revoke all grants
```

### Protection Levels

| Level | Behavior |
|-------|----------|
| **Observe** | Record activity, notify on important actions |
| **Ask Me** | Ask before sensitive actions, auto-allow low-risk |
| **Strict** | Deny undeclared actions, require approval for sends/spends/deletes |
| **Sovereign** | Local-first, no cloud, deny network unless approved |

---

## Quick Start

### Prerequisites
- Rust toolchain (1.75+)
- Node.js 20+
- pnpm (`npm install -g pnpm`)

### Build & Test

```bash
# Clone
git clone https://github.com/TheInnerI/inner-i-universal-observer.git
cd inner-i-universal-observer

# Compile everything
cargo check

# Run all tests (35 tests)
cargo test

# Start the Observer Node (binds to 127.0.0.1:7411)
cargo run -p observer-node
```

### Run the Wake-Up Demo

```bash
# Start the Observer Node in one terminal
cargo run -p observer-node

# In another terminal, run the demo
./examples/demo.sh
```

The demo walks through:
1. Register a "Document Organizer" agent
2. Agent requests safe filesystem access → **ALLOWED**
3. Agent executes file organization → **SUCCESS**
4. Agent attempts credential access → **BLOCKED**
5. Violation recorded as residual
6. Proof bundle exported

### Development Mode (all services)

```bash
pnpm install
just dev
```

Launches: Observer Node, Control Center, Relay, Mobile dev server.
No Docker required. No cloud credentials required.

---

## Architecture

```
INNER I MOBILE     permissions • approvals • alerts • stop • proof • identity
       │
       │ IIOP (Inner I Observer Protocol)
       ▼
OPTIONAL ENCRYPTED RELAY
       │
┌──────┼──────────────┐
▼      ▼              ▼
OBSERVER NODE    AGENT/SDK     CONTROL CENTER
```

### How It Works

1. **Agent registers** with the Observer Node — declares its purpose
2. **Agent requests a capability** (e.g., "read ~/Documents")
3. **Policy engine evaluates** the request against the active protection level
4. **If safe** → auto-allowed. **If risky** → approval request sent to mobile
5. **User approves/denies** on Inner I Mobile
6. **Agent executes** within permitted boundaries
7. **Observer Node watches** — if agent exceeds scope, it's blocked
8. **Residual generated** for any violation
9. **Signed receipt** produced for every action
10. **Proof Bundle** can be exported and independently verified

---

## Build Phases

| Phase | Status | What |
|-------|--------|------|
| **Phase 1:** IIOP + Identity | ✅ Complete | Protocol definitions, Ed25519 signing, crypt |
| **Phase 2:** Observer Node Engine | ✅ Complete | Policy engine, capability broker, 30 API endpoints |
| **Phase 3:** Mobile App | ✅ Scaffolded | Expo/RN project with deps (QR, biometric, secure-store) |
| **Phase 4:** Residuals + Proof | ✅ Complete | Residual engine, proof store, tamper detection |
| **Phase 5:** Wake-Up Demo | ✅ Complete | `examples/demo.sh` — safe + violating agent workflow |
| **Phase 6:** Control Center | ✅ Scaffolded | Next.js project with dashboard deps |
| **Phase 7:** Relay + Hardening | ✅ Scaffolded | Express relay service |

---

## Capability Model

17 granular capability classes: calendar, email, contacts, files, photos, location, browser, network, payment, identity, credentials, process, device, model, home, robot, user.

Capability grants support: one-time use, time-limited, resource-limited, amount-limited, destination-limited, read-only, scoped disclosure, biometric confirmation, and revocation.

---

## License

This project is Open Architecture. The Inner I is the observer within every human being — not a brand, not a company. inneri76 built the tools. The tools are yours.

---

**Built by [inneri76](https://github.com/TheInnerI)** | **Contact: i@innerinetcompany.com** | **Tagline: Shape Reality**
