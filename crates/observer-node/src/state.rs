use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, Mutex};
use crypto_signing::identity::ObserverIdentity;
use capability_broker::CapabilityBroker;
use observer_core::residual::ResidualRecord;
use observer_core::types::ProtectionLevel;

/// Shared application state for the Observer Node.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub node_identity: Arc<ObserverIdentity>,
    pub started_at_unix_ms: i64,
    pub capability_broker: Arc<Mutex<CapabilityBroker>>,
    residuals: Arc<Mutex<Vec<ResidualRecord>>>,
}

impl AppState {
    pub async fn new(pool: SqlitePool) -> anyhow::Result<Self> {
        let identity = match load_existing_identity(&pool).await? {
            Some(id) => {
                tracing::info!("Loaded existing node identity: {}", id.observer_id);
                id
            }
            None => {
                let (identity, _keypair) = ObserverIdentity::new("Inner I Observer Node")?;
                sqlx::query(
                    "INSERT INTO observer_identities (observer_id, display_name, public_key_hex, created_at_unix_ms)
                     VALUES (?, ?, ?, ?)"
                )
                .bind(&identity.observer_id)
                .bind(&identity.display_name)
                .bind(&identity.public_key_hex)
                .bind(identity.created_at_unix_ms)
                .execute(&pool)
                .await?;
                tracing::info!("Created new node identity: {}", identity.observer_id);
                identity
            }
        };

        // Default to AskMe protection level — user can change in settings
        let capability_broker = CapabilityBroker::new(ProtectionLevel::AskMe)?;

        Ok(AppState {
            pool,
            node_identity: Arc::new(identity),
            started_at_unix_ms: chrono::Utc::now().timestamp_millis(),
            capability_broker: Arc::new(Mutex::new(capability_broker)),
            residuals: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Add a residual record to the in-memory store.
    pub fn add_residual(&self, residual: ResidualRecord) {
        if let Ok(mut residuals) = self.residuals.lock() {
            residuals.push(residual);
        }
    }

    /// Get all residuals as JSON values.
    pub fn get_residuals(&self) -> Vec<serde_json::Value> {
        if let Ok(residuals) = self.residuals.lock() {
            residuals.iter().map(|r| serde_json::json!({
                "residual_id": r.residual_id,
                "residual_type": format!("{:?}", r.residual_type),
                "plain_language_summary": r.plain_language_summary,
                "severity": format!("{:?}", r.severity),
                "response": r.response,
                "data_exposed": r.data_exposed,
                "detected_at": r.detected_at_unix_ms,
            })).collect()
        } else {
            Vec::new()
        }
    }
}

async fn load_existing_identity(pool: &SqlitePool) -> anyhow::Result<Option<ObserverIdentity>> {
    let row = sqlx::query_as::<_, IdentityRow>(
        "SELECT observer_id, display_name, public_key_hex, created_at_unix_ms, device_fingerprint
         FROM observer_identities LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ObserverIdentity {
        observer_id: r.observer_id,
        display_name: r.display_name,
        public_key_hex: r.public_key_hex,
        created_at_unix_ms: r.created_at_unix_ms,
        device_fingerprint: r.device_fingerprint,
    }))
}

#[derive(sqlx::FromRow)]
struct IdentityRow {
    observer_id: String,
    display_name: String,
    public_key_hex: String,
    created_at_unix_ms: i64,
    device_fingerprint: Option<String>,
}