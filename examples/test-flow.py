#!/usr/bin/env python3
"""Non-interactive demo — tests the full Observer flow."""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "sdks", "python"))
from inneri import InnerIAgent, check_observer, list_residuals, list_agents

print("Observer:", check_observer())
agent = InnerIAgent("Doc Organizer", "demo", "Organize files")
agent.register()
r1 = agent.request_capability("files.read", "~/Documents")
print("Safe request:", r1.get("status"))
r2 = agent.request_capability("credentials.request", "~/.ssh/id_rsa")
print("Blocked request:", r2.get("status"), "-", r2.get("reason", ""))
try:
    agent.stop()
except Exception as e:
    pass  # stop may fail if agent already cleaned up
print("Residuals:", len(list_residuals()))
print("Agents:", len(list_agents()))
print("DONE — check http://localhost:7412")
