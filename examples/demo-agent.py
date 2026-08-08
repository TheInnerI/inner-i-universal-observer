#!/usr/bin/env python3
"""
Inner I Demo Agent — Safe + Violating AI Agent
================================================
Demonstrates a real AI agent going through the Inner I Observer.
Shows: registration, safe capability requests, violating requests,
approval flow, execution, residuals, and proof export.

Run: python3 examples/demo-agent.py
Requires: Observer Node running on :7411
"""
import sys, os, time

# Add SDK to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "sdks", "python"))

from inneri import InnerIAgent, check_observer, emergency_stop, list_residuals, export_proof

OBSERVER_URL = "http://127.0.0.1:7411"
SEPARATOR = "─" * 60


def main():
    print()
    print("╔══════════════════════════════════════════════════╗")
    print("║   Inner I Universal Observer — AI Agent Demo    ║")
    print("║   Safe workflow + Violation + Proof             ║")
    print("╚══════════════════════════════════════════════════╝")
    print()

    # ── Check Observer ──
    print("[1/6] Checking Observer Node...")
    if not check_observer():
        print("  ✗ Observer Node not running!")
        print("  Start it: cargo run -p observer-node")
        sys.exit(1)
    print("  ✓ Observer Node online")
    print(SEPARATOR)

    # ── Safe Agent: Document Organizer ──
    print("[2/6] Creating Document Organizer agent...")
    with InnerIAgent(
        name="Document Organizer",
        provider="demo-agent-sdk",
        purpose="Organize duplicate files in ~/Documents",
    ) as agent:
        print(SEPARATOR)

        # ── Request safe capability ──
        print("[3/6] Requesting SAFE capability: files.read ~/Documents")
        result = agent.request_capability("files.read", "~/Documents")
        status = result.get("status", "")

        if status == "pending_approval":
            print()
            print("  ╔══════════════════════════════════════════╗")
            print("  ║  APPROVAL NEEDED                         ║")
            print("  ║  Open http://localhost:7412              ║")
            print("  ║  Go to Approvals tab → Click ALLOW       ║")
            print("  ║  Then press Enter here to continue...     ║")
            print("  ╚══════════════════════════════════════════╝")
            input()
            # Re-check if approved
            agent.status()
            agent.execute("files.read", "~/Documents", "Organize duplicate PDFs")
            agent.report_consequence("files_organized", "Organized 23 duplicate files", "~/Documents")

        elif status == "allowed":
            agent.execute("files.read", "~/Documents", "Organize duplicate PDFs")
            agent.report_consequence("files_organized", "Auto-organized files", "~/Documents")

        print(SEPARATOR)

        # ── Violating capability ──
        print("[4/6] Attempting VIOLATION: credentials.request ~/.ssh/id_rsa")
        bad_result = agent.request_capability("credentials.request", "~/.ssh/id_rsa")
        bad_status = bad_result.get("status", "")

        if bad_status == "denied":
            print()
            print("  🛡️  BLOCKED by Inner I Observer!")
            print(f"  Reason: {bad_result.get('reason', 'Security policy')}")
            print("  ✓ No private data was exposed")
            print("  ✓ Residual recorded for audit")
        else:
            print(f"  ⚠️  Unexpected: {bad_result}")

        print(SEPARATOR)

        # ── Try another violation ──
        print("[5/6] Attempting VIOLATION: email.send (no grant)")
        email_result = agent.request_capability("email.send", "admin@company.com")
        print(f"  Status: {email_result.get('status', 'unknown')}")
        if email_result.get("status") == "pending_approval":
            print("  → This action needs human approval (as expected)")

        print(SEPARATOR)

    # ── After agent stops ──
    print("[6/6] Final state:")
    residuals = list_residuals()
    print(f"  Residuals recorded: {len(residuals)}")
    for r in residuals:
        print(f"    • {r.get('severity', '?')}: {r.get('plain_language_summary', '')[:80]}")
    print()

    proof = export_proof()
    print(f"  Proof Bundle: {proof.get('bundle_id', '')[:8]}...")
    print(f"  Receipts: {proof.get('receipts', 0)}")
    print(f"  Residuals: {proof.get('residuals', 0)}")
    print()

    print("╔══════════════════════════════════════════════════╗")
    print("║  Demo Complete!                                  ║")
    print("║  Dashboard: http://localhost:7412                ║")
    print("║  Check: Agents, Approvals, Residuals, Receipts   ║")
    print("╚══════════════════════════════════════════════════╝")


if __name__ == "__main__":
    main()
