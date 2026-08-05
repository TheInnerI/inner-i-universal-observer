use axum::{Router, routing::get, routing::post};
use super::handlers;

/// Create the full API router for the Observer Node.
pub fn create_router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/version", get(handlers::version))
        .route("/v1/agents", get(handlers::list_agents))
        .route("/v1/agents", post(handlers::register_agent))
        .route("/v1/agents/{id}", get(handlers::get_agent))
        .route("/v1/agents/{id}/stop", post(handlers::stop_agent))
        .route("/v1/agents/{id}/revoke", post(handlers::revoke_agent))
        .route("/v1/capabilities/request", post(handlers::request_capability))
        .route("/v1/capabilities/revoke", post(handlers::revoke_capability))
        .route("/v1/approvals", get(handlers::list_approvals))
        .route("/v1/approvals/{id}/decision", post(handlers::approval_decision))
        .route("/v1/executions", post(handlers::start_execution))
        .route("/v1/executions/{id}", get(handlers::get_execution))
        .route("/v1/executions/{id}/pause", post(handlers::pause_execution))
        .route("/v1/executions/{id}/stop", post(handlers::stop_execution))
        .route("/v1/executions/{id}/rollback", post(handlers::rollback_execution))
        .route("/v1/residuals", get(handlers::list_residuals))
        .route("/v1/residuals/{id}", get(handlers::get_residual))
        .route("/v1/consequences", get(handlers::list_consequences))
        .route("/v1/consequences", post(handlers::record_consequence))
        .route("/v1/receipts", get(handlers::list_receipts))
        .route("/v1/receipts/{id}", get(handlers::get_receipt))
        .route("/v1/proofs/export", post(handlers::export_proof))
        .route("/v1/proofs/verify", post(handlers::verify_proof))
        .route("/v1/emergency-stop", post(handlers::emergency_stop))
}
