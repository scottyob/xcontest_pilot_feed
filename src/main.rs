mod pilot;
mod flight_rss;

use pilot::{fetch_pilot_id};
use serde::Deserialize;
use std::{fs, error::Error};
use std::collections::HashMap;

use crate::pilot::{fetch_flights, Flight};
use crate::flight_rss::generate_rss;

#[derive(Debug, Deserialize)]
struct Config {
    key: String,
    url: String,
    users: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_content = fs::read_to_string("config.yml")?;
    let config: Config = serde_yaml::from_str(&config_content)?;

    // Try to load the cache
    let cache_path = "pilotIDs.cache.yml";
    let mut pilot_id_map: HashMap<String, u64> = if let Ok(cache_content) = fs::read_to_string(cache_path) {
        serde_yaml::from_str(&cache_content)?
    } else {
        HashMap::new()
    };

    let mut cache_updated = false;

    for user in &config.users {
        // Update the user ID if it doesn't exist in the cache
        if pilot_id_map.get(user).is_none() {
            match fetch_pilot_id(user) {
                Ok(id) => {
                    pilot_id_map.insert(user.clone(), id);
                    cache_updated = true;
                }
                Err(e) => eprintln!("Error fetching pilot ID for '{}': {}", user, e),
            }
        }
    }

    // Update cache file if new pilots were added
    if cache_updated {
        let cache_yaml = serde_yaml::to_string(&pilot_id_map)?;
        fs::write(cache_path, cache_yaml)?;
    }

    let mut flights: Vec<Flight> = Vec::new();

    // Enumerate the pilot IDs
    for (user, id) in &pilot_id_map {

        match fetch_flights(*id, &config.key) {
            Ok(user_flights) => {
                // for flight in &flights {
                //     println!("{}\n---", flight_summary(flight));
                // }
                flights.extend(user_flights);
            },
            Err(e) => eprintln!("Error fetching flights for user '{}': {}", user, e),
        }
    }

    // Generate RSS feed
    let site_rss = generate_rss(&flights, &config.url);
    println!("{}", site_rss);

    Ok(())
}
