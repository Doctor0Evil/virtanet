use tikv_client::{Transaction, TransactionClient};
use utils::Result;
use std::sync::Arc;

pub struct DistributedStore {
    client: Arc<TransactionClient>,
}

impl DistributedStore {
    pub async fn new(endpoints: Vec<String>) -> Result<Self> {
        let config = Config::default().with_endpoints(endpoints);
        let client = Arc::new(TransactionClient::new(config).await?);
        Ok(Self { client })
    }

    pub async fn put(&self, key: &str, value: &str) -> Result<()> {
        let mut txn = self.client.begin_optimistic().await?;
        txn.put(key.to_string(), value.to_string()).await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut txn = self.client.begin_optimistic().await?;
        let value = txn.get(key.to_string()).await?;
        Ok(value.map(|v| String::from_utf8(v).unwrap()))
    }
}
use actix_web::{web, App, HttpServer, Responder};