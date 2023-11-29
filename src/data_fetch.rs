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

pub trait AssignmentManipulation {
    fn first_date(&self) -> NaiveDate;
}

impl AssignmentManipulation for Vec<Assignment> {
    fn first_date(&self) -> NaiveDate {
        self.get(0)
            .expect("Response isn't empty")
            .info
            .as_ref()
            .unwrap()
            .due_at
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

        let due_at = NaiveDate::parse_from_str(&raw_info.due_at, "%Y-%m-%dT%H:%M:%SZ")
            .map_err(serde::de::Error::custom)?;

        Ok(AssignmentInfo { due_at })
    }
}

pub async fn get_assignments(access_token: String) -> Result<Vec<Assignment>, reqwest::Error> {
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

    let valid: Vec<Assignment> = api_response
        .into_iter()
        .filter(|assignment| assignment.to_owned().info.is_some())
        .collect();

    Ok(valid)
}
