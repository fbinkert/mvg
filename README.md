# MVG Client for Rust

A robust, asynchronous, and unofficial Rust client for the Munich Transport Corporation (MVG) APIs. This library provides easy access to public transport data in Munich, including station searches, real-time departures, and line information.

## Features

* **Station Search:** Find stations by name (`Marienplatz`) or global ID (`de:09162:100`).
* **Real-time Departures:** Get upcoming departures with delay info, filtering by transport type (U-Bahn, Bus, Tram, etc.).
* **Geo-Location:** Find stations nearby specific GPS coordinates.
* **Master Data:** Retrieve full lists of stations and lines from the ZDM (Central Data Management) backend.
* **Async/Await:** Built on `reqwest` and `tokio` for non-blocking I/O.


## Usage Example

```rust
use mvg_client::{MvgClient, models::TransportType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MvgClient::default();

    // Search for a station
    if let Some(station) = client.get_station("Hauptbahnhof").await? {
        println!("Found: {}", station.name);

        // Get next 5 departures
        let departures = client.get_departures(&station.id, 5, 0, None).await?;
        for dep in departures {
            println!("{} - {} ({} min)", dep.line_label, dep.destination, dep.time_to_departure());
        }
    }
    Ok(())
}

```

## Disclaimer & Terms of Use

This library is an **unofficial** client. It is not endorsed by or affiliated with Münchner Verkehrsgesellschaft (MVG).

### MVG Usage Policy

The underlying API is provided by MVG. Users of this library must adhere to MVG's usage policy.
**Data Mining is strictly prohibited.**

> **Nutzungsbedingungen (Original German Text):**
> "Unsere Systeme dienen der direkten Kundeninteraktion. Die Verarbeitung unserer Inhalte oder Daten durch Dritte erfordert unsere ausdrückliche Zustimmung. Für private, nicht-kommerzielle Zwecke, wird eine gemäßigte Nutzung ohne unsere ausdrückliche Zustimmung geduldet. Jegliche Form von Data-Mining stellt keine gemäßigte Nutzung dar. Wir behalten uns vor, die Duldung grundsätzlich oder in Einzelfällen zu widerrufen. Fragen richten Sie bitte gerne an: redaktion@mvg.de"

By using this library, you agree to limit your request rates ("moderate use") and to use the data solely for private, non-commercial purposes unless you have obtained express consent from MVG.

## License

MIT License. See [LICENSE](./LICENSE) for details.

