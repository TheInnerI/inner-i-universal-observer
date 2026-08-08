#!/usr/bin/env python3
"""
Inner I Observer SDK (Python)
===============================
Lightweight client for AI agents to integrate with the Inner I Observer Node.
Agents register, declare intent, request capabilities, respect decisions,
and report consequences.
"""
import requests
import json
import uuid
import time
from typing import Optional, Dict, Any, List
from dataclasses import dataclass, field, asdict

OBSERVER_URL = "http://127.0.0.1:7411"


@dataclass
class CapabilitySpec:
    action: str
    resource: str
    maximum_amount: Optional[float] = None
    duration: str = "one_time"
    scopes: List[str] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "action": self.action,
            "resource": self.resource,
            "maximum_amount": self.maximum_amount,
            "duration": self.duration,
            "scopes": self.scopes,
        }


class InnerIAgent:
    """
    An AI agent that operates under Inner I Observer governance.
    Every action goes through: register → declare intent → request capability →
    receive decision → execute if allowed → report consequence.
    """

    def __init__(self, name: str, provider: str = "agent-sdk", purpose: str = ""):
        self.name = name
        self.provider = provider
        self.purpose = purpose
        self.agent_id: Optional[str] = None
        self.grants: List[Dict] = []

    # ── Registration ──

    def register(self) -> str:
        """Register with the Observer Node and get an agent ID."""
        resp = requests.post(
            f"{OBSERVER_URL}/v1/agents",
            json={
                "display_name": self.name,
                "provider": self.provider,
                "declared_purpose": self.purpose,
            },
        )
        resp.raise_for_status()
        data = resp.json()
        self.agent_id = data["agent_id"]
        print(f"  ✓ Registered: {self.name} ({self.agent_id[:8]}...)")
        return self.agent_id

    # ── Capability Request ──

    def request_capability(self, action: str, resource: str, **kwargs) -> Dict[str, Any]:
        """Request permission to perform an action. Returns the decision."""
        if not self.agent_id:
            raise RuntimeError("Agent not registered. Call register() first.")

        cap = CapabilitySpec(action=action, resource=resource, **kwargs)
        resp = requests.post(
            f"{OBSERVER_URL}/v1/capabilities/request",
            json={
                "agent_id": self.agent_id,
                "agent_display_name": self.name,
                "action": cap.action,
                "resource": cap.resource,
                "maximum_amount": cap.maximum_amount,
            },
        )
        resp.raise_for_status()
        decision = resp.json()
        self._handle_decision(decision, cap)
        return decision

    def _handle_decision(self, decision: Dict, cap: CapabilitySpec):
        status = decision.get("status", "unknown")
        if status == "allowed":
            self.grants.append(decision)
            print(f"  ✅ ALLOWED: {cap.action} → {cap.resource}")
        elif status == "denied":
            print(f"  🚫 DENIED: {cap.action} → {cap.resource}")
            print(f"     Reason: {decision.get('reason', 'unknown')}")
        elif status == "pending_approval":
            print(f"  ⏳ PENDING: {cap.action} → {cap.resource}")
            print(f"     Request ID: {decision.get('request_id', '')[:8]}...")
            print(f"     Risk: {decision.get('risk_level', 'unknown')}")
            print(f"     Open Control Center to approve/deny: http://localhost:7412")

    # ── Execute ──

    def execute(self, action: str, resource: str, purpose: str = "") -> Dict:
        """Execute an action that has been approved."""
        if not self.agent_id:
            raise RuntimeError("Agent not registered.")

        resp = requests.post(
            f"{OBSERVER_URL}/v1/executions",
            json={
                "agent_id": self.agent_id,
                "action": action,
                "resource": resource,
                "declared_purpose": purpose or self.purpose,
            },
        )
        resp.raise_for_status()
        result = resp.json()
        if result.get("status") == "started":
            print(f"  ▶ EXECUTED: {action} → {resource}")
        else:
            print(f"  ✗ BLOCKED: {action} → {resource} — {result.get('reason')}")
        return result

    # ── Consequence ──

    def report_consequence(self, ctype: str, description: str, resource: str = ""):
        """Report the outcome of an action."""
        resp = requests.post(
            f"{OBSERVER_URL}/v1/consequences",
            json={
                "type": ctype,
                "description": description,
                "affected_resource": resource,
            },
        )
        resp.raise_for_status()
        print(f"  📋 Recorded: {ctype} — {description}")

    # ── Status ──

    def status(self) -> Dict:
        """Get current agent status including active grants."""
        resp = requests.get(f"{OBSERVER_URL}/v1/agents/{self.agent_id}")
        resp.raise_for_status()
        return resp.json()

    def stop(self):
        """Stop this agent and revoke all grants."""
        resp = requests.post(f"{OBSERVER_URL}/v1/agents/{self.agent_id}/stop")
        resp.raise_for_status()
        print(f"  ■ Stopped: {self.name}")

    def __enter__(self):
        self.register()
        return self

    def __exit__(self, *args):
        self.stop()


# ── Health Check ──

def check_observer() -> bool:
    """Check if the Observer Node is running."""
    try:
        resp = requests.get(f"{OBSERVER_URL}/health", timeout=2)
        return resp.json().get("status") == "ok"
    except Exception:
        return False


# ── Emergency Stop ──

def emergency_stop():
    """Stop all AI activity."""
    resp = requests.post(f"{OBSERVER_URL}/v1/emergency-stop", json={})
    resp.raise_for_status()
    return resp.json()


# ── List all ──

def list_agents():
    resp = requests.get(f"{OBSERVER_URL}/v1/agents")
    return resp.json().get("agents", [])


def list_residuals():
    resp = requests.get(f"{OBSERVER_URL}/v1/residuals")
    return resp.json().get("residuals", [])


def list_receipts():
    resp = requests.get(f"{OBSERVER_URL}/v1/receipts")
    return resp.json().get("receipts", [])


def export_proof():
    resp = requests.post(f"{OBSERVER_URL}/v1/proofs/export", json={})
    return resp.json()
