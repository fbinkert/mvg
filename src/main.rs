use chrono::DateTime;
use mvg::client::MvgClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mvg = MvgClient::new();

    println!("Searching for 'Universität'...");
    if let Some(station) = mvg.get_station("Mittersendling").await? {
        println!("Found Station: {} ({})", station.name, station.id);

        // 2. Get departures
        let deps = mvg.get_departures(&station.id, 5, 0, None).await?;

        println!("\nNext Departures:");
        for d in deps {
            let time = DateTime::from_timestamp(d.time_ms / 1000, 0).unwrap_or_default();
            println!(
                "[{}] {:<5} -> {:<20} (Delay: {}m)",
                time.format("%H:%M"),
                d.label,
                d.destination,
                d.delay_in_minutes.unwrap_or(0)
            );
        }
    }
    Ok(())
}
