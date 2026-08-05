use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use crypto_signing::identity::ObserverIdentity;

/// Shared application state for the Observer Node.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub node_identity: Arc<ObserverIdentity>,
    pub started_at_unix_ms: i64,
}

impl AppState {
    pub async fn new(pool: SqlitePool) -> anyhow::Result<Self> {
        // Generate or load the node's observer identity
        let identity = match load_existing_identity(&pool).await? {
            Some(id) => {
                tracing::info!("Loaded existing node identity: {}", id.observer_id);
                id
            }
            None => {
                let (identity, _keypair) = ObserverIdentity::new("Inner I Observer Node")?;
                // Store the identity
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

        Ok(AppState {
            pool,
            node_identity: Arc::new(identity),
            started_at_unix_ms: chrono::Utc::now().timestamp_millis(),
        })
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
