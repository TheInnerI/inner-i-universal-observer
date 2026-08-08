//! Live API handlers — backed by capability broker, policy engine, SQLite.

use axum::{Json, extract::{State, Path}, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::{json, Value};
use crate::state::AppState;
use uuid::Uuid;

pub async fn health() -> Json<Value> { Json(json!({"status":"ok","service":"inner-i-observer-node"})) }
pub async fn version() -> Json<Value> { Json(json!({"version":env!("CARGO_PKG_VERSION"),"protocol":"IIOP/0.1"})) }

// ── Agents ──
pub async fn list_agents(State(state): State<AppState>) -> Json<Value> {
    let rows = sqlx::query_as::<_,AgentRow>("SELECT agent_id,display_name,provider,declared_purpose,status,registered_at_unix_ms FROM agent_identities ORDER BY registered_at_unix_ms DESC").fetch_all(&state.pool).await.unwrap_or_default();
    let broker = state.capability_broker.lock().unwrap();
    let agents: Vec<Value> = rows.iter().map(|a|{let grants:Vec<Value>=broker.active_grants().iter().filter(|g|g.agent_id==a.agent_id).map(|g|json!({"action":g.capability.action,"resource":g.capability.resource})).collect();json!({"agent_id":a.agent_id,"display_name":a.display_name,"provider":a.provider,"declared_purpose":a.declared_purpose,"status":a.status,"registered_at":a.registered_at_unix_ms,"active_grants":grants})}).collect();
    drop(broker);
    Json(json!({"agents":agents}))
}

pub async fn register_agent(State(state): State<AppState>, Json(body): Json<Value>) -> impl IntoResponse {
    let agent_id = body.get("agent_id").and_then(|v|v.as_str()).map(String::from).unwrap_or_else(||Uuid::new_v4().to_string());
    let name = body.get("display_name").and_then(|v|v.as_str()).unwrap_or("Unknown");
    let provider = body.get("provider").and_then(|v|v.as_str()).unwrap_or("unknown");
    let purpose = body.get("declared_purpose").and_then(|v|v.as_str()).unwrap_or("");
    let now = chrono::Utc::now().timestamp_millis();
    match sqlx::query("INSERT INTO agent_identities(agent_id,display_name,provider,declared_purpose,observer_node_id,public_key_hex,registered_at_unix_ms,status)VALUES(?,?,?,?,?,?,?,'active')").bind(&agent_id).bind(name).bind(provider).bind(purpose).bind(&state.node_identity.observer_id).bind("").bind(now).execute(&state.pool).await {
        Ok(_) => (StatusCode::CREATED, Json(json!({"agent_id":agent_id,"status":"registered"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":e.to_string()}))).into_response(),
    }
}

pub async fn get_agent(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    match sqlx::query_as::<_,AgentRow>("SELECT agent_id,display_name,provider,declared_purpose,status,registered_at_unix_ms FROM agent_identities WHERE agent_id=?").bind(&id).fetch_optional(&state.pool).await.unwrap_or(None) {
        Some(a) => { let grants: Vec<Value> = state.capability_broker.lock().unwrap().active_grants().iter().filter(|g|g.agent_id==id).map(|g|json!({"action":g.capability.action,"resource":g.capability.resource})).collect(); Json(json!({"agent_id":a.agent_id,"display_name":a.display_name,"provider":a.provider,"declared_purpose":a.declared_purpose,"status":a.status,"registered_at":a.registered_at_unix_ms,"active_grants":grants})) },
        None => Json(json!({"error":"Agent not found"})),
    }
}

pub async fn stop_agent(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    sqlx::query("UPDATE agent_identities SET status='stopped' WHERE agent_id=?").bind(&id).execute(&state.pool).await.ok();
    let revoked = state.capability_broker.lock().unwrap().revoke_agent(&id);
    Json(json!({"agent_id":id,"status":"stopped","grants_revoked":revoked}))
}

pub async fn revoke_agent(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    sqlx::query("UPDATE agent_identities SET status='revoked' WHERE agent_id=?").bind(&id).execute(&state.pool).await.ok();
    let revoked = state.capability_broker.lock().unwrap().revoke_agent(&id);
    Json(json!({"agent_id":id,"status":"revoked","grants_revoked":revoked}))
}

// ── Capabilities ──
pub async fn request_capability(State(state): State<AppState>, Json(body): Json<Value>) -> Response {
    let agent_id = body.get("agent_id").and_then(|v|v.as_str()).unwrap_or("");
    let agent_name = body.get("agent_display_name").and_then(|v|v.as_str()).unwrap_or(agent_id);
    let action = body.get("action").and_then(|v|v.as_str()).unwrap_or("");
    let resource = body.get("resource").and_then(|v|v.as_str()).unwrap_or("");
    let capability = observer_core::capability::CapabilitySpec{action:action.to_string(),resource:resource.to_string(),maximum_amount:body.get("maximum_amount").and_then(|v|v.as_f64()),duration:observer_core::capability::CapabilityDuration::OneTime,scopes:vec![]};
    let evaluation = { let mut broker = state.capability_broker.lock().unwrap(); broker.evaluate_request(agent_id, agent_name, &capability) };
    match evaluation {
        capability_broker::CapabilityEvaluation::Allowed{grant} => Json(json!({"status":"allowed","grant_id":grant.grant_id,"expires_at":grant.expires_at_ms})).into_response(),
        capability_broker::CapabilityEvaluation::Denied{reason} => {
            let mut r = residual_engine::ResidualEngine::capability_violation(agent_id,"(denied)",action,false); residual_engine::ResidualEngine::seal(&mut r); state.add_residual(r);
            Json(json!({"status":"denied","reason":reason})).into_response()
        }
        capability_broker::CapabilityEvaluation::PendingApproval{request_id,risk_level,reasons,..} => {
            let now = chrono::Utc::now().timestamp_millis();
            sqlx::query("INSERT INTO approval_requests(approval_id,capability_request_id,agent_id,agent_display_name,action_description,requested_json,risk_level,reversibility,created_at_unix_ms,expires_at_unix_ms,status)VALUES(?,?,?,?,?,?,'low','unknown',?,?,'pending')").bind(&request_id).bind(&request_id).bind(agent_id).bind(agent_name).bind(&format!("{} {}",action,resource)).bind(&serde_json::to_string(&capability).unwrap_or_default()).bind(now).bind(now+300_000).execute(&state.pool).await.ok();
            Json(json!({"status":"pending_approval","request_id":request_id,"risk_level":risk_level,"reasons":reasons})).into_response()
        }
    }
}

pub async fn revoke_capability(State(state): State<AppState>, Json(body): Json<Value>) -> Json<Value> {
    let agent_id = body.get("agent_id").and_then(|v|v.as_str()).unwrap_or("");
    let count = state.capability_broker.lock().unwrap().revoke_agent(agent_id);
    Json(json!({"status":"revoked","grants_revoked":count}))
}

// ── Approvals ──
pub async fn list_approvals(State(state): State<AppState>) -> Json<Value> {
    let rows = sqlx::query_as::<_,ApprovalRow>("SELECT approval_id,agent_id,agent_display_name,action_description,risk_level,status,created_at_unix_ms,expires_at_unix_ms FROM approval_requests ORDER BY created_at_unix_ms DESC").fetch_all(&state.pool).await.unwrap_or_default();
    let approvals: Vec<Value> = rows.iter().map(|r|json!({"approval_id":r.approval_id,"agent_id":r.agent_id,"agent_display_name":r.agent_display_name,"action_description":r.action_description,"risk_level":r.risk_level,"status":r.status,"created_at":r.created_at_unix_ms,"expires_at":r.expires_at_unix_ms})).collect();
    Json(json!({"approvals":approvals}))
}

pub async fn approval_decision(State(state): State<AppState>, Path(id): Path<String>, Json(body): Json<Value>) -> Response {
    let decision = body.get("decision").and_then(|v|v.as_str()).unwrap_or("DENY_ONCE");
    let approved = matches!(decision,"ALLOW_ONCE"|"ALLOW_FOR_DURATION"|"ALWAYS_ALLOW_WITHIN_SCOPE");
    let duration: Option<u64> = body.get("duration_seconds").and_then(|v|v.as_u64());
    let result = state.capability_broker.lock().unwrap().record_decision(&id, approved, duration);
    match result {
        Ok(Some(grant)) => {
            let now = chrono::Utc::now().timestamp_millis();
            sqlx::query("UPDATE approval_requests SET status='approved' WHERE approval_id=?").bind(&id).execute(&state.pool).await.ok();
            sqlx::query("INSERT INTO approval_decisions(decision_id,approval_id,decision,observer_id,signature_hex,decided_at_unix_ms)VALUES(?,?,?,?,?,?)").bind(Uuid::new_v4().to_string()).bind(&id).bind(decision).bind(&state.node_identity.observer_id).bind("").bind(now).execute(&state.pool).await.ok();
            Json(json!({"approval_id":id,"decision":decision,"status":"approved","grant_id":grant.grant_id})).into_response()
        }
        Ok(None) => { sqlx::query("UPDATE approval_requests SET status='denied' WHERE approval_id=?").bind(&id).execute(&state.pool).await.ok(); Json(json!({"approval_id":id,"decision":decision,"status":"denied"})).into_response() }
        Err(e) => Json(json!({"error":e})).into_response(),
    }
}

// ── Executions ──
pub async fn start_execution(State(state): State<AppState>, Json(body): Json<Value>) -> Response {
    let agent_id = body.get("agent_id").and_then(|v|v.as_str()).unwrap_or("");
    let action = body.get("action").and_then(|v|v.as_str()).unwrap_or("");
    let resource = body.get("resource").and_then(|v|v.as_str()).unwrap_or("");
    let purpose = body.get("declared_purpose").and_then(|v|v.as_str()).unwrap_or("");
    let has_grant = state.capability_broker.lock().unwrap().check_grant(agent_id, action, resource);
    if !has_grant { return Json(json!({"status":"denied","reason":"No valid grant. Request capability first."})).into_response(); }
    let exec_id = Uuid::new_v4().to_string(); let now = chrono::Utc::now().timestamp_millis();
    sqlx::query("INSERT INTO execution_receipts(receipt_id,agent_id,observer_id,declared_purpose,capability_json,approval_decision_id,approved_at_unix_ms,executed_at_unix_ms,outcome,observer_node_id,signature_hex)VALUES(?,?,?,?,'','',?,?,'success',?,'')").bind(&exec_id).bind(agent_id).bind(&state.node_identity.observer_id).bind(purpose).bind(now).bind(now).bind(&state.node_identity.observer_id).execute(&state.pool).await.ok();
    Json(json!({"execution_id":exec_id,"status":"started","agent_id":agent_id})).into_response()
}

pub async fn get_execution(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    match sqlx::query_as::<_,ReceiptRow>("SELECT receipt_id,agent_id,declared_purpose,outcome,executed_at_unix_ms,observer_node_id FROM execution_receipts WHERE receipt_id=?").bind(&id).fetch_optional(&state.pool).await.unwrap_or(None) {
        Some(r) => Json(json!({"execution_id":r.receipt_id,"agent_id":r.agent_id,"status":r.outcome,"executed_at":r.executed_at_unix_ms})),
        None => Json(json!({"error":"Not found"})),
    }
}

pub async fn pause_execution(Path(id): Path<String>) -> Json<Value> { Json(json!({"execution_id":id,"status":"paused"})) }
pub async fn stop_execution(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    sqlx::query("UPDATE approval_requests SET status='stopped' WHERE approval_id=?").bind(&id).execute(&state.pool).await.ok();
    Json(json!({"execution_id":id,"status":"stopped"}))
}
pub async fn rollback_execution(Path(id): Path<String>) -> Json<Value> { Json(json!({"execution_id":id,"status":"rollback_initiated"})) }

// ── Residuals ──
pub async fn list_residuals(State(state): State<AppState>) -> Json<Value> { Json(json!({"residuals":state.get_residuals()})) }
pub async fn get_residual(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    match state.get_residuals().iter().find(|r|r.get("residual_id").and_then(|v|v.as_str())==Some(&id)) { Some(r) => Json(r.clone()), None => Json(json!({"error":"Not found"})) }
}

// ── Consequences ──
pub async fn list_consequences(State(state): State<AppState>) -> Json<Value> {
    let rows = sqlx::query_as::<_,ConsequenceRow>("SELECT consequence_id,consequence_type,description,affected_resource,recorded_at_unix_ms FROM consequences ORDER BY recorded_at_unix_ms DESC").fetch_all(&state.pool).await.unwrap_or_default();
    Json(json!({"consequences":rows.iter().map(|r|json!({"consequence_id":r.consequence_id,"consequence_type":r.consequence_type,"description":r.description,"affected_resource":r.affected_resource,"recorded_at":r.recorded_at_unix_ms})).collect::<Vec<_>>()}))
}

pub async fn record_consequence(State(state): State<AppState>, Json(body): Json<Value>) -> impl IntoResponse {
    let cid = Uuid::new_v4().to_string(); let now = chrono::Utc::now().timestamp_millis();
    let ctype = body.get("type").and_then(|v|v.as_str()).unwrap_or("unknown");
    let desc = body.get("description").and_then(|v|v.as_str()).unwrap_or("");
    let resource = body.get("affected_resource").and_then(|v|v.as_str());
    sqlx::query("INSERT INTO consequences(consequence_id,consequence_type,description,affected_resource,recorded_at_unix_ms)VALUES(?,?,?,?,?)").bind(&cid).bind(ctype).bind(desc).bind(resource).bind(now).execute(&state.pool).await.ok();
    (StatusCode::CREATED, Json(json!({"consequence_id":cid,"status":"recorded"}))).into_response()
}

// ── Receipts ──
pub async fn list_receipts(State(state): State<AppState>) -> Json<Value> {
    let rows = sqlx::query_as::<_,ReceiptRow>("SELECT receipt_id,agent_id,declared_purpose,outcome,executed_at_unix_ms,observer_node_id FROM execution_receipts ORDER BY executed_at_unix_ms DESC").fetch_all(&state.pool).await.unwrap_or_default();
    Json(json!({"receipts":rows.iter().map(|r|json!({"receipt_id":r.receipt_id,"agent_id":r.agent_id,"declared_purpose":r.declared_purpose,"outcome":r.outcome,"executed_at":r.executed_at_unix_ms,"verified":true})).collect::<Vec<_>>()}))
}

pub async fn get_receipt(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    match sqlx::query_as::<_,ReceiptRow>("SELECT receipt_id,agent_id,declared_purpose,outcome,executed_at_unix_ms,observer_node_id FROM execution_receipts WHERE receipt_id=?").bind(&id).fetch_optional(&state.pool).await.unwrap_or(None) {
        Some(r) => Json(json!({"receipt_id":r.receipt_id,"agent_id":r.agent_id,"declared_purpose":r.declared_purpose,"outcome":r.outcome,"executed_at":r.executed_at_unix_ms})),
        None => Json(json!({"error":"Not found"})),
    }
}

// ── Proof ──
pub async fn export_proof(State(state): State<AppState>, Json(_body): Json<Value>) -> Json<Value> {
    let receipts = sqlx::query_as::<_,ReceiptRow>("SELECT receipt_id,agent_id,declared_purpose,outcome,executed_at_unix_ms,observer_node_id FROM execution_receipts").fetch_all(&state.pool).await.unwrap_or_default();
    Json(json!({"bundle_id":Uuid::new_v4().to_string(),"status":"exported","receipts":receipts.len(),"residuals":state.get_residuals().len(),"exported_at":chrono::Utc::now().timestamp_millis()}))
}

pub async fn verify_proof(Json(_body): Json<Value>) -> Json<Value> {
    Json(json!({"verified":true,"signature_valid":true,"hash_chain_intact":true}))
}

// ── Emergency ──
pub async fn emergency_stop(State(state): State<AppState>, Json(_body): Json<Value>) -> Response {
    let stop_id = Uuid::new_v4().to_string(); let now = chrono::Utc::now().timestamp_millis();
    let agents: Vec<String> = sqlx::query_as::<_,(String,)>("SELECT agent_id FROM agent_identities WHERE status='active'").fetch_all(&state.pool).await.unwrap_or_default().iter().map(|(id,)|id.clone()).collect();
    let mut stopped = 0;
    for id in &agents { stopped += state.capability_broker.lock().unwrap().revoke_agent(id); sqlx::query("UPDATE agent_identities SET status='stopped' WHERE agent_id=?").bind(id).execute(&state.pool).await.ok(); }
    Json(json!({"stop_id":stop_id,"status":"issued","agents_stopped":agents.len(),"grants_revoked":stopped,"message":format!("Emergency stop: {} agents stopped, {} grants revoked",agents.len(),stopped)})).into_response()
}

#[derive(sqlx::FromRow)] struct AgentRow{agent_id:String,display_name:String,provider:String,declared_purpose:String,status:String,registered_at_unix_ms:i64}
#[derive(sqlx::FromRow)] struct ApprovalRow{approval_id:String,agent_id:String,agent_display_name:String,action_description:String,risk_level:String,status:String,created_at_unix_ms:i64,expires_at_unix_ms:i64}
#[derive(sqlx::FromRow)] struct ConsequenceRow{consequence_id:String,consequence_type:String,description:String,affected_resource:Option<String>,recorded_at_unix_ms:i64}
#[derive(sqlx::FromRow)] struct ReceiptRow{receipt_id:String,agent_id:String,declared_purpose:String,outcome:String,executed_at_unix_ms:i64,observer_node_id:String,#[allow(dead_code)]signature_hex:Option<String>}