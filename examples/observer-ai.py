#!/usr/bin/env python3
"""
Inner I Observer-Aware AI Agent
=================================
A real AI agent that calls OpenRouter models, but only after
getting permission from the Inner I Observer Node.

Every API call goes through:
1. Register with Observer
2. Declare intent
3. Request capability (may require human approval)
4. If allowed → call OpenRouter
5. Report consequence back to Observer

Usage:
    python3 examples/observer-ai.py "Summarize my Documents folder"
    # → requests files.read → needs approval → you approve in dashboard → LLM runs

Env vars:
    OPENROUTER_API_KEY — your OpenRouter key (optional, falls back to simulation)
"""

import sys, os, json, time

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "sdks", "python"))
from inneri import InnerIAgent, check_observer

OPENROUTER_KEY = os.environ.get("OPENROUTER_API_KEY", "")
FREE_MODEL = "google/gemini-2.5-flash-lite:free"  # Free tier model


def call_llm(prompt: str) -> str:
    """Call OpenRouter LLM. Falls back to simulated response if no key."""
    if not OPENROUTER_KEY:
        return f"[Simulated AI response for: {prompt[:60]}...]"

    import requests
    resp = requests.post(
        "https://openrouter.ai/api/v1/chat/completions",
        headers={"Authorization": f"Bearer {OPENROUTER_KEY}"},
        json={
            "model": FREE_MODEL,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 200,
        },
        timeout=30,
    )
    resp.raise_for_status()
    return resp.json()["choices"][0]["message"]["content"]


def main():
    task = " ".join(sys.argv[1:]) if len(sys.argv) > 1 else "List the files in ~/Documents"

    print()
    print("╔══════════════════════════════════════════════════╗")
    print("║   Inner I Observer-Aware AI Agent                ║")
    print("╚══════════════════════════════════════════════════╝")
    print()
    print(f"  Task: {task}")
    print()

    if not check_observer():
        print("  ✗ Observer Node not running. Start: cargo run -p observer-node")
        sys.exit(1)

    # Determine what capabilities this task needs
    needed_capabilities = []
    task_lower = task.lower()

    if any(w in task_lower for w in ["file", "document", "folder", "read", "list"]):
        needed_capabilities.append(("files.read", "~/Documents"))
    if any(w in task_lower for w in ["email", "send", "draft", "mail"]):
        needed_capabilities.append(("email.send", "*"))
    if any(w in task_lower for w in ["browser", "web", "search", "url"]):
        needed_capabilities.append(("browser.navigate", "*"))
    if any(w in task_lower for w in ["calendar", "schedule", "meeting"]):
        needed_capabilities.append(("calendar.read", "*"))
    if not needed_capabilities:
        needed_capabilities.append(("user.notify", "*"))  # least privilege

    with InnerIAgent(
        name="Observer-Aware AI",
        provider="openrouter" if OPENROUTER_KEY else "simulated",
        purpose=task,
    ) as agent:
        # Request all needed capabilities
        approved = True
        for action, resource in needed_capabilities:
            result = agent.request_capability(action, resource)
            status = result.get("status", "")

            if status == "denied":
                print(f"  🚫 Cannot proceed — {action} denied")
                approved = False
                break
            elif status == "pending_approval":
                print()
                print("  ╔══════════════════════════════════════════╗")
                print("  ║  APPROVAL NEEDED                         ║")
                print(f"  ║  Action: {action} → {resource}          ║")
                print("  ║  Open http://localhost:7412              ║")
                print("  ║  Approvals tab → Allow → press Enter      ║")
                print("  ╚══════════════════════════════════════════╝")
                input()

        if not approved:
            print("  ✗ Task blocked by Observer policy.")
            return

        # Execute the AI call
        print(f"  🤖 Calling AI model ({'OpenRouter' if OPENROUTER_KEY else 'simulated'})...")
        response = call_llm(task)
        print(f"  ✓ AI Response: {response[:200]}")
        print()

        # Report consequence
        agent.report_consequence("ai_task_completed", f"Completed: {task[:80]}", resource=needed_capabilities[0][1])

    print()
    print("  ✓ Done. Check dashboard: http://localhost:7412")


if __name__ == "__main__":
    main()
