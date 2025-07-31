use reqwest::blocking::Client;
use reqwest::header::{ACCEPT_LANGUAGE, USER_AGENT};
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use iso8601_duration::Duration;
use humantime::format_duration;


#[derive(Debug, Error)]
pub enum PilotError {
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Username not found in page")]
    UsernameNotFound,

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Missing expected field in response")]
    MissingField,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Flight {
    pub id: u64,
    pub duration: String,
    pub start_time: String,
    pub url: String,
    pub route: Route,
    pub by: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Route {
    #[serde(rename = "type")]
    pub route_type: String,
    pub distance: f64,
    pub points: f64,
}

pub fn fetch_pilot_id(username: &str) -> Result<u64, PilotError> {
    let url = format!(
        "https://www.xcontest.org/world/en/pilots/detail:{}",
        username
    );
    let client = Client::new();
    let resp = client
        .get(&url)
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.9")
        .header(USER_AGENT, "Mozilla/5.0")
        .send()?
        .text()?;

    let re = regex::Regex::new(r"item\s*:\s*(\d+)").unwrap();

    if let Some(caps) = re.captures(&resp) {
        if let Some(m) = caps.get(1) {
            let pilot_id: u64 = m.as_str().parse().unwrap();
            return Ok(pilot_id);
        }
    }

    Err(PilotError::UsernameNotFound)
}

pub fn fetch_flights(pilot_id: u64, key: &String) -> Result<Vec<Flight>, PilotError> {
    let url = format!(
        "https://www.xcontest.org/api/data/?flights/world/2025&lng=en&key={key}&list%5Bstart%5D=0&list%5Bnum%5D=250&list%5Bsort%5D=time_claim&list%5Bdir%5D=down&filter%5Bpilot%5D={pilot_id}"
    );

    let client = Client::new();
    let resp = client
        .get(&url)
        .header(ACCEPT_LANGUAGE, "en-US,en")
        .header("Accept", "application/json")
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36")
        .send()?
        .error_for_status()?
        .json::<Value>()?;

    let Some(items) = resp.get("items").and_then(|v| v.as_array()) else {
        return Err(PilotError::MissingField);
    };

    let mut results = Vec::new();

    for item in items {
        let id = item
            .get("id")
            .and_then(|v| v.as_u64())
            .ok_or(PilotError::MissingField)?;
        let duration = item
            .pointer("/stats/duration")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let by: String = item
            .pointer("/pilot/name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();  
        let start_time = item
            .pointer("/pointStart/time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let url = item
            .pointer("/league/flight/link")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let route_obj = item
            .pointer("/league/route")
            .ok_or(PilotError::MissingField)?;
        let route: Route = serde_json::from_value(route_obj.clone())?;

        results.push(Flight {
            id,
            duration,
            start_time,
            by,
            url,
            route,
        });
    }

    Ok(results)
}

#[allow(dead_code)]
pub fn flight_summary(flight: &Flight) -> String {
    let duration_str: &String = &flight.duration;
    let duration_parsed = Duration::parse(duration_str).unwrap();
    let std_duration = duration_parsed.to_std().unwrap_or(std::time::Duration::from_secs(0));

    format!(
        "{}'s Flight on {}\nlasting {}\n{} with a distance of {:.1}km\nScoring {:.1} points\nURL: {}",
        flight.by,
        &flight.start_time[..10], // YYYY-MM-DD
        format_duration(std_duration),
        flight.route.route_type,
        flight.route.distance,
        flight.route.points,
        flight.url,
    )
}
