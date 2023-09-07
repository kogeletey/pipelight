use super::Getters;
use crate::globals::PORTAL;
use crate::workflow::types::{Logs, Pipeline};
use cast;

// Date and Time
use chrono::{DateTime, Local};

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>> {
        let portal;
        unsafe {
            portal = (*PORTAL).clone();
        };
        let logs: Vec<String> = cast::Logs::read(&format!(
            "{}/.pipelight/logs/",
            portal.target.directory_path.unwrap()
        ))?;
        let mut pipelines: Vec<Pipeline> = vec![];
        for json in logs {
            let pipeline = serde_json::from_str::<Pipeline>(&json).into_diagnostic()?;
            pipelines.push(pipeline);
        }
        // Sort by date ascending
        pipelines.sort_by(|a, b| {
            let a_date = a
                .clone()
                .event
                .unwrap()
                .date
                .parse::<DateTime<Local>>()
                .unwrap();
            let b_date = &b
                .clone()
                .event
                .unwrap()
                .date
                .parse::<DateTime<Local>>()
                .unwrap();
            a_date.cmp(b_date)
        });
        Ok(pipelines)
    }
    fn get_by_name(name: &str) -> Result<Pipeline> {
        let pipelines = Logs::get()?;
        let pipeline;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| p.name == *name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
            pipeline = pipelines.pop().unwrap();
            Ok(pipeline)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            Err(Error::msg(message))
        }
    }
}
impl Logs {
    pub fn get_many_by_name(name: &str) -> Result<Vec<Pipeline>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| p.name == *name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
            pipelines.sort_by(|a, b| {
                let a_date = a
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();

                let b_date = &b
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();
                a_date.cmp(b_date)
            });
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            Err(Error::msg(message))
        }
    }
    pub fn _get_many_by_sid(sid: &u32) -> Result<Vec<Pipeline>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| {
                if p.event.clone().unwrap().sid.is_some() {
                    let p_sid = p.event.clone().unwrap().sid.unwrap();
                    &p_sid == sid
                } else {
                    false
                }
            })
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by(|a, b| {
                let a_date = a
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();

                let b_date = &b
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();
                a_date.cmp(b_date)
            });
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline with sid {:?}, in logs", sid);
            Err(Error::msg(message))
        }
    }
}
