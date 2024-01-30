use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assignment {
    #[serde(rename = "context_name")]
    pub course: String,
    pub title: String,
    #[serde(rename = "assignment")]
    pub info: Option<AssignmentInfo>,
    #[serde(rename = "html_url")]
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct AssignmentInfo {
    pub due_at: NaiveDate,
}

pub trait FirstDate {
    fn first_date(&self) -> &NaiveDate;
}

impl FirstDate for Vec<Assignment> {
    fn first_date(&self) -> &NaiveDate {
        return &self
            .get(0)
            .expect("Response isn't empty")
            .info
            .as_ref()
            .unwrap()
            .due_at;
    }
}

impl<'de> Deserialize<'de> for AssignmentInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawAssignmentInfo {
            due_at: String,
        }

        let raw_info = RawAssignmentInfo::deserialize(deserializer)?;

        if let Ok(due_at) = NaiveDate::parse_from_str(&raw_info.due_at, "%Y-%m-%dT%H:%M:%SZ") {
            return Ok(AssignmentInfo { due_at });
        } else if let Ok(due_at) = NaiveDate::parse_from_str(&raw_info.due_at, "%Y-%m-%d") {
            return Ok(AssignmentInfo { due_at });
        }

        return Err(serde::de::Error::custom("Invalid date format"));
    }
}

#[derive(Debug)]
struct JSONOldError;
impl std::error::Error for JSONOldError {}
impl std::fmt::Display for JSONOldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON file is too old")
    }
}

async fn fetch_assignments(access_token: String) -> Result<Vec<Assignment>, reqwest::Error> {
    let api_endpoint: String = format!(
        "https://canvas.butte.edu/api/v1/users/self/upcoming_events?access_token={}",
        access_token
    );

    let api_response: Vec<Assignment> = reqwest::Client::new()
        .get(api_endpoint)
        .send()
        .await?
        .json()
        .await?;

    return Ok(api_response
        .into_iter()
        .filter(|assignment| assignment.to_owned().info.is_some())
        .collect());
}

fn access_json() -> Result<Vec<Assignment>, Box<dyn std::error::Error>> {
    let file = File::open("canvas_assignment_data.json")?;
    let reader = BufReader::new(file);
    let a: Vec<Assignment> = serde_json::from_reader(reader)?;

    let oldest_date = a.first_date();
    let current_date = chrono::Local::now().date_naive();

    if oldest_date.lt(&current_date) {
        return Err(Box::new(JSONOldError));
    }

    return Ok(a);
}

pub async fn get_assignments(access_token: String) -> Result<Vec<Assignment>, reqwest::Error> {
    // return assignments early if a json already exists
    if let Ok(a) = access_json() {
        return Ok(a);
    }

    let assignments = fetch_assignments(access_token).await?;
    let file = File::create("canvas_assignment_data.json").expect("Unable to write JSON file");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &assignments).unwrap();
    writer.flush().expect("Unable to write JSON file");

    return Ok(assignments);
}
