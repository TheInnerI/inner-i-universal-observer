# Inner I Universal Observer

> **One person. One Observer identity. Every machine accountable.**

Inner I gives every person one simple place to see, approve, block, stop, reverse, and verify what AI systems do in their name.

**The central principle:** AI may act for a person, but it must never become invisible to the person.

**The product promise:** Every machine acting in your name must ask permission, stay within its boundaries, leave a receipt, and answer for the consequences.

---

## What This Is

A universal permission, proof, and consequence layer for personal AI.

Four products in one monorepo:

| Product | Type | Stack |
|---------|------|-------|
| **Inner I Mobile** | Phone-first approval & proof app | React Native / Expo |
| **Inner I Observer Node** | Local enforcement engine | Rust |
| **Inner I Control Center** | Advanced web dashboard | Next.js / TypeScript |
| **Inner I Relay** | Optional encrypted relay | Minimal service |

Connected through **IIOP** — the Inner I Observer Protocol.

## Quick Start

```bash
pnpm install
just dev
```

Development mode launches:
- Observer Node (Rust)
- Control Center (Next.js)
- Relay service
- Mobile dev server

No Docker required. No cloud credentials required.

## Architecture

```
INNER I MOBILE     permissions • approvals • alerts • stop • proof • identity
       │
       │ IIOP
       ▼
OPTIONAL ENCRYPTED RELAY
       │
┌──────┼──────────────┐
▼      ▼              ▼
OBSERVER NODE    AGENT/SDK     CONTROL CENTER
```

## Build Order

- **Phase 1:** IIOP protocol + identity + signing + QR pairing ✅ (current)
- **Phase 2:** Observer Node (local API, agent registration, capability broker)
- **Phase 3:** Mobile app (onboarding, approvals, receipts, stop-all)
- **Phase 4:** Residuals + proof (Residual engine, receipts, Proof Bundles)
- **Phase 5:** Wake-up demo (safe + violating agent demo)
- **Phase 6:** Control Center (advanced dashboard)
- **Phase 7:** Relay + hardening

## License

This project is Open Architecture. The Inner I is the observer within every human being — not a brand, not a company. inneri76 built the tools. The tools are yours.

---

**Built by [inneri76](https://github.com/TheInnerI)** | **Contact: i@innerinetcompany.com** | **Tagline: Shape Reality**
