use humantime::format_duration;
use rss::{ChannelBuilder, Item, ItemBuilder};
use crate::Flight;
use chrono::Utc;

pub fn generate_rss(flights: &[Flight], site_url: &str) -> String {
    let mut sorted_flights = flights.to_vec();
    sorted_flights.sort_by_key(|f| f.start_time.clone());
    sorted_flights.reverse();

    let items: Vec<Item> = sorted_flights
        .iter()
        .map(|flight| {
            let date = &flight.start_time[..10]; // "YYYY-MM-DD"
            let title = format!(
                "{}: {} - {} {:.1}km ({:.1} pts)",
                flight.by,
                date,
                flight.route.route_type,
                flight.route.distance,
                flight.route.points
            );

            let duration_str: &String = &flight.duration;
            let duration_parsed = iso8601_duration::Duration::parse(duration_str).unwrap();
            let std_duration = duration_parsed.to_std().unwrap_or(std::time::Duration::from_secs(0));

            let description = format!(
                "{} flew a {} of {:.1}km scoring {:.1} points.\nFlight duration: {}\nURL: {}",
                flight.by,
                flight.route.route_type,
                flight.route.distance,
                flight.route.points,
                format_duration(std_duration),
                flight.url,
            );

            ItemBuilder::default()
                .title(Some(title))
                .link(Some(flight.url.clone()))
                .description(Some(description))
                .pub_date(Some(date.to_string()))
                .guid(Some(rss::GuidBuilder::default().value(flight.id.to_string()).build()))
                .build()
        })
        .collect();

    ChannelBuilder::default()
        .title("XContest Flight Feed")
        .link(site_url)
        .pub_date(Utc::now().to_rfc2822())    // Current date and time
        .description("Recent flights")
        .items(items)
        .build()
        .to_string()
}
