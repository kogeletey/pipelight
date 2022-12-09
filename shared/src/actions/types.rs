use crate::types::{Pipeline, Step};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Stopped,
    Running,
    Failed,
    Succeeded,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineLogs {
    pub name: String,
    pub status: Status,
    pub steps_logs: Vec<StepLogs>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StepLogs {
    pub name: String,
    pub commands: Option<Vec<String>>,
}
