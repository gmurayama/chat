use std::{str::FromStr, time::Duration};

use deadpool_postgres::{
    tokio_postgres::NoTls, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime,
};
use tokio_postgres::types::ToSql;

use uuid::Uuid;

pub struct PostgresSettings {
    pub url: String,
    pub max_size: usize,
    pub timeout: Option<Duration>,
}

pub fn connection_pool(settings: PostgresSettings) -> eyre::Result<deadpool_postgres::Pool> {
    let cfg = tokio_postgres::Config::from_str(&settings.url)?;
    let manager = Manager::from_config(
        cfg,
        NoTls,
        ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );

    let pool = Pool::builder(manager)
        .max_size(settings.max_size)
        .wait_timeout(settings.timeout)
        .runtime(Runtime::Tokio1)
        .build()?;

    Ok(pool)
}

#[derive(Debug, Clone)]
pub struct PgSessionRepository<'a> {
    conn: &'a deadpool_postgres::Pool,
}

impl<'a> PgSessionRepository<'a> {
    pub fn new(conn: &'a deadpool_postgres::Pool) -> PgSessionRepository {
        PgSessionRepository { conn }
    }

    pub async fn add(&self, user_id: String, addr: String) -> eyre::Result<()> {
        let db = self.conn.get().await?;
        let user_id = Uuid::parse_str(user_id.as_str())?;

        let params: &[&(dyn ToSql + Sync)] = &[&user_id, &addr];
        let query = db.execute(
            "INSERT INTO sessions(user_id, addr) VALUES($1, $2)",
            params.into(),
        );

        tokio::time::timeout(Duration::from_secs(3), query).await??;

        Ok(())
    }
}
