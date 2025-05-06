use anyhow::{Result, anyhow};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::models::incident::Incident;

#[derive(Debug, Default)]
pub struct IncidentService {
    pub limit: u32,
}

impl IncidentService {
    pub fn new(limit: u32) -> Result<Self> {
        if limit > 5 || limit < 3 {
            return Err(anyhow!("Invalid limit"));
        }
        Ok(IncidentService { limit })
    }

    pub async fn create_file(&self, file: &str) -> Result<()> {
        File::create(file).await?;
        Ok(())
    }

    pub async fn save_to_file(&self, incident: Incident, file: &str) -> Result<()> {
        let mut file = File::create(file).await?;
        for _ in 0..self.limit {
            file.write_all(incident.to_string().as_bytes()).await?;
        }
        Ok(())
    }
}
