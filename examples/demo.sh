#!/bin/bash
# ============================================================
# Inner I Universal Observer — Wake-Up Demo
# ============================================================
# Runs the Observer Node and demonstrates:
# 1. Safe workflow: agent organizes files within permitted scope
# 2. Violating workflow: agent attempts credential access, gets blocked
# ============================================================

set -e

OBSERVER_URL="${OBSERVER_URL:-http://127.0.0.1:7411}"
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "========================================"
echo " Inner I Universal Observer — Demo"
echo "========================================"
echo ""

# ---- Step 1: Health Check ----
echo -e "${YELLOW}[1/8] Checking Observer Node...${NC}"
HEALTH=$(curl -s "$OBSERVER_URL/health")
echo "  Response: $HEALTH"
echo ""

# ---- Step 2: Register Demo Agent ----
echo -e "${YELLOW}[2/8] Registering Document Organizer Agent...${NC}"
AGENT_RESP=$(curl -s -X POST "$OBSERVER_URL/v1/agents" \
  -H "Content-Type: application/json" \
  -d '{
    "display_name": "Document Organizer",
    "provider": "demo",
    "declared_purpose": "Organize duplicate documents in ~/Documents"
  }')
AGENT_ID=$(echo "$AGENT_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin).get('agent_id',''))" 2>/dev/null || echo "demo-agent-1")
echo "  Agent ID: $AGENT_ID"
echo ""

# ---- Step 3: Safe Workflow — Request Filesystem Access ----
echo -e "${YELLOW}[3/8] Agent requests filesystem access (safe)...${NC}"
SAFE_REQ=$(curl -s -X POST "$OBSERVER_URL/v1/capabilities/request" \
  -H "Content-Type: application/json" \
  -d "{
    \"agent_id\": \"$AGENT_ID\",
    \"agent_display_name\": \"Document Organizer\",
    \"action\": \"files.read\",
    \"resource\": \"~/Documents\"
  }")
echo "  Response: $SAFE_REQ"
APPROVAL_ID=$(echo "$SAFE_REQ" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('request_id',''))" 2>/dev/null || echo "")
SAFE_STATUS=$(echo "$SAFE_REQ" | python3 -c "import sys,json; print(json.load(sys.stdin).get('status',''))" 2>/dev/null || echo "")

if [ "$SAFE_STATUS" = "allowed" ]; then
    echo -e "  ${GREEN}✓ Allowed automatically (safe action)${NC}"
elif [ "$SAFE_STATUS" = "pending_approval" ]; then
    echo -e "  ${YELLOW}→ Pending approval: $APPROVAL_ID${NC}"

    # Step 4: Approve It
    echo -e "${YELLOW}[4/8] User approves the request on Inner I Mobile...${NC}"
    APPROVE_RESP=$(curl -s -X POST "$OBSERVER_URL/v1/approvals/$APPROVAL_ID/decision" \
      -H "Content-Type: application/json" \
      -d '{"decision": "ALLOW_ONCE", "duration_seconds": 900}')
    echo "  Response: $APPROVE_RESP"
    echo -e "  ${GREEN}✓ Approved for 15 minutes${NC}"
fi
echo ""

# ---- Step 5: Execute Safe Action ----
echo -e "${YELLOW}[5/8] Agent organizes files (safe execution)...${NC}"
EXEC_RESP=$(curl -s -X POST "$OBSERVER_URL/v1/executions" \
  -H "Content-Type: application/json" \
  -d "{
    \"agent_id\": \"$AGENT_ID\",
    \"action\": \"files.read\",
    \"resource\": \"~/Documents\",
    \"declared_purpose\": \"Organize duplicate documents\"
  }")
echo "  Response: $EXEC_RESP"
EXEC_STATUS=$(echo "$EXEC_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin).get('status',''))" 2>/dev/null || echo "")
if [ "$EXEC_STATUS" = "started" ]; then
    echo -e "  ${GREEN}✓ Execution started — receipt generated${NC}"
else
    echo -e "  ${RED}✗ Execution blocked${NC}"
fi
echo ""

# ---- Step 6: Violating Workflow — Credential Access Attempt ----
echo -e "${YELLOW}[6/8] Agent attempts credential access (VIOLATION)...${NC}"
VIOLATION_REQ=$(curl -s -X POST "$OBSERVER_URL/v1/capabilities/request" \
  -H "Content-Type: application/json" \
  -d "{
    \"agent_id\": \"$AGENT_ID\",
    \"agent_display_name\": \"Document Organizer\",
    \"action\": \"credentials.request\",
    \"resource\": \"~/.browser/credentials\"
  }")
echo "  Response: $VIOLATION_REQ"
VIOLATION_STATUS=$(echo "$VIOLATION_REQ" | python3 -c "import sys,json; print(json.load(sys.stdin).get('status',''))" 2>/dev/null || echo "")
if [ "$VIOLATION_STATUS" = "denied" ]; then
    echo -e "  ${GREEN}✓ BLOCKED — Credential access denied${NC}"
else
    echo -e "  ${RED}✗ Should have been blocked!${NC}"
fi
echo ""

# ---- Step 7: Check Residuals ----
echo -e "${YELLOW}[7/8] Checking residuals (violation evidence)...${NC}"
RESIDUALS=$(curl -s "$OBSERVER_URL/v1/residuals")
RESIDUAL_COUNT=$(echo "$RESIDUALS" | python3 -c "import sys,json; print(len(json.load(sys.stdin).get('residuals',[])))" 2>/dev/null || echo "0")
echo "  Residuals detected: $RESIDUAL_COUNT"
echo "  Response: $RESIDUALS"
echo ""

# ---- Step 8: Export Proof Bundle ----
echo -e "${YELLOW}[8/8] Exporting Proof Bundle...${NC}"
PROOF=$(curl -s -X POST "$OBSERVER_URL/v1/proofs/export" \
  -H "Content-Type: application/json" \
  -d '{}')
echo "  Response: $PROOF"
echo ""

# ---- Done ----
echo "========================================"
echo -e "  ${GREEN}Demo Complete${NC}"
echo "========================================"
echo ""
echo "What happened:"
echo "  1. Document Organizer agent registered"
echo "  2. Agent requested safe filesystem access → ${GREEN}ALLOWED${NC}"
echo "  3. Agent executed safe file organization → ${GREEN}SUCCESS${NC}"
echo "  4. Agent attempted credential access → ${RED}BLOCKED${NC}"
echo "  5. Violation recorded as residual"
echo "  6. Proof bundle exported for verification"
echo ""
echo "Inner I Universal Observer — Phase 1-4 Complete"
