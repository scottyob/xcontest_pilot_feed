use rss::{ChannelBuilder, Item, ItemBuilder};
use crate::Flight;

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

            let description = format!(
                "{} flew a {} of {:.1}km scoring {:.1} points.\nFlight duration: {}\nURL: {}",
                flight.by,
                flight.route.route_type,
                flight.route.distance,
                flight.route.points,
                flight.duration,
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
        .description("Recent flights")
        .items(items)
        .build()
        .to_string()
}
