use actix_web::{web, App, HttpServer, Responder};
use tokio::sync::mpsc;
use std::sync::Arc;
use tikv_client::{Config, TransactionClient};
use nats::connect;
use log::info;
mod db;
mod worker;
mod utils;
async fn index() -> impl Responder {
    web::Json("Virta-Sys: Distributed Ecosystem Active")
}
#[tokio::main]
async fn main() -> utils::Result<()> {
    utils::logger::init_logger();
    info!("Starting Virta-Sys...");
    let tikv_config = Config::default().with_endpoints(vec!["127.0.0.1:2379".to_string()]);
    let tikv_client = Arc::new(TransactionClient::new(tikv_config).await?);
    info!("TiKV client initialized");
    let nats_client = connect("nats://127.0.0.1:4222")?;
    let (tx, rx) = mpsc::channel::<String>(100);
    let job_queue = Arc::new(worker::JobQueue::new(rx, nats_client));
    let queue_handle = job_queue.clone();
    tokio::spawn(async move {
        queue_handle.run_worker().await;
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tikv_client.clone()))
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run();
    info!("Web server running at http://    
    tokio::select! {
        _ = server => info!("Web server stopped"),
        _ = tokio::signal::ctrl_c() => info!("Shutdown signal received"),
    };

    Ok(())
}