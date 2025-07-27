use tokio::{sync::mpsc::Receiver, time::{sleep, Duration}};
use nats::Connection;
use std::sync::Arc;
pub struct JobQueue {
    rx: Receiver<String>,
    nats: Connection,
}
impl JobQueue {
    pub fn new(rx: Receiver<String>, nats: Connection) -> Self {
        Self { rx, nats }
    }
    pub async fn run_worker(&self) {
        while let Some(task) = self.rx.recv().await {
            self.nats.publish("virta.tasks", &task).unwrap();
            println!("Processing task: {}", task);
            sleep(Duration::from_millis(500)).await;
        }
    }
}
use actix_web::{web, App, HttpServer, Responder};
use log::info;