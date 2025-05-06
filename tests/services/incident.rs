use std::{fs::metadata, path::Path};

use cucmber_example::{models::incident::Incident, services::incident::IncidentService};
use cucumber::{World, given, then, when};

#[derive(World, Debug, Default)]
pub struct IncidentWorld {
    limit: i32,
    service: IncidentService,
    incident: Incident,
    file_path: String,
}

#[given(expr = "Michael wanted to create service with limit {int}")]
async fn service_create(w: &mut IncidentWorld, limit: i32) {
    assert!(limit > 0, "Limit must be greater than zero");
    w.limit = limit;
}

#[given(expr = "he wants to save an incident with name {word} and content {word}")]
async fn incident_create(w: &mut IncidentWorld, name: String, content: String) {
    w.incident = Incident::new(name, content);
}

#[when(expr = "he initializes the service")]
async fn service_init(w: &mut IncidentWorld) {
    let service = IncidentService::new(w.limit as u32);
    assert!(
        service.is_ok(),
        "Failed to create service with limit {}",
        w.limit
    );
    w.service = service.unwrap();
}

#[when(expr = "he creates file {word}")]
async fn file_creation(w: &mut IncidentWorld, file_path: String) {
    assert!(
        w.service.create_file(&file_path).await.is_ok(),
        "Failed to create file {}",
        file_path
    );
    let path = Path::new(&file_path);
    assert!(path.exists(), "File is not exits {}", file_path);
    w.file_path = file_path;
}

#[when(expr = "he saves the incident")]
async fn save_incident(w: &mut IncidentWorld) {
    let result = w
        .service
        .save_to_file(w.incident.clone(), &w.file_path)
        .await;
    assert!(result.is_ok(), "Failed to save incident");
}

#[then("incident is saved")]
async fn is_saved(w: &mut IncidentWorld) {
    let path = Path::new(&w.file_path);
    let metadata = metadata(path);
    if let Ok(metadata) = metadata {
        assert!(metadata.len() > 0, "File is empty");
    } else {
        panic!("Failed to get metadata");
    }
}
