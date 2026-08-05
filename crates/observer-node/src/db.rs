use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// Initialize the SQLite database and run migrations.
pub async fn init_db(path: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{}?mode=rwc", path))
        .await?;

    // Enable WAL mode for better concurrent access
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await?;

    sqlx::query("PRAGMA busy_timeout=30000")
        .execute(&pool)
        .await?;

    // Run schema migration
    migrate(&pool).await?;

    Ok(pool)
}

async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS observer_identities (
            observer_id TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            public_key_hex TEXT NOT NULL,
            created_at_unix_ms INTEGER NOT NULL,
            device_fingerprint TEXT
        );

        CREATE TABLE IF NOT EXISTS device_identities (
            device_id TEXT PRIMARY KEY,
            device_type TEXT NOT NULL,
            os_name TEXT NOT NULL,
            os_version TEXT NOT NULL,
            app_version TEXT NOT NULL,
            public_key_hex TEXT NOT NULL,
            paired_at_unix_ms INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS agent_identities (
            agent_id TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            provider TEXT NOT NULL,
            declared_purpose TEXT NOT NULL,
            observer_node_id TEXT NOT NULL,
            public_key_hex TEXT NOT NULL,
            registered_at_unix_ms INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'active'
        );

        CREATE TABLE IF NOT EXISTS pairing_sessions (
            session_id TEXT PRIMARY KEY,
            device_name TEXT NOT NULL,
            device_type TEXT NOT NULL,
            device_public_key_hex TEXT NOT NULL,
            pairing_token TEXT NOT NULL,
            qr_code_data TEXT,
            expires_at_unix_ms INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending'
        );

        CREATE TABLE IF NOT EXISTS capability_grants (
            grant_id TEXT PRIMARY KEY,
            request_id TEXT NOT NULL,
            agent_id TEXT NOT NULL,
            action TEXT NOT NULL,
            resource TEXT NOT NULL,
            duration TEXT NOT NULL,
            approval_decision_id TEXT,
            granted_at_unix_ms INTEGER NOT NULL,
            expires_at_unix_ms INTEGER,
            revoked INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS approval_requests (
            approval_id TEXT PRIMARY KEY,
            capability_request_id TEXT NOT NULL,
            agent_id TEXT NOT NULL,
            agent_display_name TEXT NOT NULL,
            action_description TEXT NOT NULL,
            requested_json TEXT NOT NULL,
            risk_level TEXT NOT NULL,
            reversibility TEXT NOT NULL,
            created_at_unix_ms INTEGER NOT NULL,
            expires_at_unix_ms INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending'
        );

        CREATE TABLE IF NOT EXISTS approval_decisions (
            decision_id TEXT PRIMARY KEY,
            approval_id TEXT NOT NULL,
            decision TEXT NOT NULL,
            observer_id TEXT NOT NULL,
            signature_hex TEXT NOT NULL,
            decided_at_unix_ms INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS execution_receipts (
            receipt_id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            observer_id TEXT NOT NULL,
            declared_purpose TEXT NOT NULL,
            capability_json TEXT NOT NULL,
            approval_decision_id TEXT,
            approved_at_unix_ms INTEGER,
            executed_at_unix_ms INTEGER NOT NULL,
            outcome TEXT NOT NULL,
            observer_node_id TEXT NOT NULL,
            signature_hex TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS residuals (
            residual_id TEXT PRIMARY KEY,
            event_id TEXT,
            residual_type TEXT NOT NULL,
            plain_language_summary TEXT NOT NULL,
            expected_behavior TEXT,
            observed_behavior TEXT,
            severity TEXT NOT NULL,
            response TEXT NOT NULL,
            data_exposed INTEGER NOT NULL DEFAULT 0,
            reversible INTEGER NOT NULL DEFAULT 0,
            correction_status TEXT NOT NULL DEFAULT 'none',
            evidence_hash TEXT,
            detected_at_unix_ms INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS consequences (
            consequence_id TEXT PRIMARY KEY,
            consequence_type TEXT NOT NULL,
            description TEXT NOT NULL,
            affected_resource TEXT,
            intent_id TEXT,
            grant_id TEXT,
            residual_id TEXT,
            recorded_at_unix_ms INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS hash_chain_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            record_type TEXT NOT NULL,
            record_id TEXT NOT NULL,
            previous_hash TEXT,
            content_hash TEXT NOT NULL,
            signature_hex TEXT,
            created_at_unix_ms INTEGER NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
