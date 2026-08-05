# Security Policy

## Core Principle

The Inner I Universal Observer is a security product. It must never weaken the user's security posture.

## Identity & Keys

- All identities use Ed25519 keypairs
- Private keys stored in platform-secure storage (iOS Keychain, Android Keystore, OS credential stores)
- Observer Node keys encrypted at rest
- Biometric confirmation required for high-risk approvals
- All approvals are cryptographically signed
- Revocations are signed and immediately effective

## Protocol Security (IIOP)

- Every message includes: protocol version, message ID, timestamp, sender, recipient, observer ID, session ID, payload hash, previous record hash, signature
- Replay protection via message IDs and timestamps
- Encrypted relay payloads (end-to-end)
- Short-lived approval tokens
- Device revocation support

## Observer Node Security

- Local API bound to loopback by default (127.0.0.1)
- Authenticated local API with token auth
- Append-only evidence stores
- Tamper detection on proof chains
- Strict input validation
- Path traversal defenses
- Symlink escape defenses
- Rate limiting on all endpoints

## Relay Security

- End-to-end encrypted message payloads
- Relay has NO access to private keys
- Relay has NO access to plaintext evidence
- Relay CANNOT approve actions
- Relay CANNOT modify policies
- Short-lived message storage
- Replay protection
- Abuse rate limiting

## Reporting

Report security issues to: i@innerinetcompany.com

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (Phase 1) | ✅ Development |
