use cucumber::World;
use services::incident::IncidentWorld;

pub mod services;

#[tokio::main]
async fn main() {
    IncidentWorld::run("tests/features/incident.feature").await;
}
