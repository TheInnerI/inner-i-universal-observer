# Architecture

## Inner I Universal Observer

The Inner I Universal Observer is a distributed observer architecture that connects human authority, machine capabilities, runtime evidence, signed consequences, and reversible execution through the Inner I Observer Protocol (IIOP).

## Core Principle

> AI may act for a person, but it must never become invisible to the person.

## System Architecture

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

## Components

### Inner I Mobile (React Native / Expo)
The phone-first human authority surface. Handles:
- Observer identity creation
- QR code device pairing
- Approval requests (Allow Once, Deny, Always Allow, Stop Agent)
- Biometric confirmation for high-risk actions
- Execution receipts (human-readable)
- Residual alerts (violations detected)
- Emergency stop all AI
- Proof Bundle verification
- Offline read-only mode

### Inner I Observer Node (Rust)
The local enforcement engine. Handles:
- Agent registration and identity
- Capability broker (granular permissions)
- Policy evaluation (Observe / Ask Me / Strict / Sovereign)
- Invariant enforcement (Layer 0)
- Filesystem, network, and process enforcement
- Event observation and recording
- Residual generation
- Consequence recording
- Signed receipt creation
- Emergency stop execution
- Offline operation

### Inner I Control Center (Next.js / TypeScript)
Advanced web dashboard for developers and operators. Handles:
- Policy editing
- Agent management
- Device management
- Live execution monitoring
- Residual graphs
- Consequence analysis
- Proof Bundle viewing
- Team management
- Diagnostics

### Inner I Relay
Optional encrypted relay service. Handles:
- End-to-end encrypted message delivery
- Approval request forwarding across networks
- Short-lived offline message storage
- Device revocation
- Rate limiting

## IIOP: Inner I Observer Protocol

The universal protocol connecting human identities, mobile devices, Observer Nodes, agents, applications, and policy engines.

### Core Object Types (24 total)
1. ObserverIdentity
2. DeviceIdentity
3. AgentIdentity
4. ArtifactIdentity
5. PairingRequest
6. IntentDeclaration
7. CapabilityRequest
8. CapabilityGrant
9. CapabilityRevocation
10. InvariantSet
11. ExecutionRequest
12. ApprovalRequest
13. ApprovalDecision
14. EmergencyStop
15. ObservedEvent
16. StateSnapshot
17. ResidualRecord
18. ConsequenceRecord
19. CorrectionProposal
20. RollbackRequest
21. RollbackRecord
22. ExecutionReceipt
23. ProofBundle
24. ProofVerificationResult

### Protocol Envelope
Every IIOP message includes: protocol_version, message_id, message_type, timestamp, sender_id, recipient_id, observer_id, session_id, parent_message_id, payload_hash, previous_record_hash, signature, encryption_metadata, payload.

## Capability Model

Granular permissions across 17 capability classes:
calendar, email, contacts, files, photos, location, browser, network, payment, identity, credentials, process, device, model, home, robot, user.

## Protection Levels

- **Observe** — Record, notify, minimal auto-blocking
- **Ask Me** — Ask before sensitive actions
- **Strict** — Deny undeclared actions, require approval for high-risk
- **Sovereign** — Local-first, no cloud, deny network unless approved

## Identity & Security

- Ed25519 keypairs for all identities
- Platform-secure private key storage (iOS Keychain, Android Keystore)
- Signed approvals and revocations
- Append-only evidence stores
- Hash-chain integrity
- Independent proof verification

## Build Order

1. **Phase 1:** IIOP + Identity + Signing + QR Pairing
2. **Phase 2:** Observer Node (API, agent registration, capability broker)
3. **Phase 3:** Mobile App (onboarding, approvals, receipts, stop-all)
4. **Phase 4:** Residuals + Proof (engine, receipts, Proof Bundles)
5. **Phase 5:** Wake-Up Demo (safe + violating agent)
6. **Phase 6:** Control Center (advanced dashboard)
7. **Phase 7:** Relay + Hardening
