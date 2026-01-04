
# MVG CLI

A terminal client for the Munich Public Transport system (MVG). Check departures, manage favorites, and filter by direction.

## Features

* **Departures Lookup:** Quickly check departures for any station.
* **Dashboard:** A `monitor` command to show all your favorite connections at a glance.
* **Direction Filtering:** Filter favorites to only show trains going in a specific direction (e.g., only show U-Bahn heading towards "München Freiheit").
* **Live Data:** Fetches real-time delays and departure times.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
# Clone the repository
git clone https://github.com/fbinkert/mvg.git
cd mvg

# Build and run
cargo run -- --help

# Optional: Install globally
cargo install --path .

```

## Usage

### 1. Quick Lookup

Check the next 10 departures for a specific station.

```bash
mvg departures "Marienplatz" --limit 10

```

### 2. Managing Favorites

Save stations to your local configuration for quick access.

**Add a favorite:**

```bash
# Basic add
mvg fav add "Hauptbahnhof" --alias "Hbf"

# Add with direction filter (e.g., only trains going to Garching)
mvg fav add "Universität" --alias "Uni" --direction "Garching"

```

**List favorites:**

```bash
mvg fav list

```

**Remove a favorite:**

```bash
mvg fav remove "Hbf"

```

### 3. The Monitor

View the status of all your saved favorites in one view.

```bash
mvg monitor

```

## Disclaimer

This tool is an unofficial client. The underlying API is provided by MVG. Users of this tool must adhere to MVG's usage policy.
**Data Mining is strictly prohibited.**

> **Nutzungsbedingungen (Original German Text):**
> "Unsere Systeme dienen der direkten Kundeninteraktion. Die Verarbeitung unserer Inhalte oder Daten durch Dritte erfordert unsere ausdrückliche Zustimmung. Für private, nicht-kommerzielle Zwecke, wird eine gemäßigte Nutzung ohne unsere ausdrückliche Zustimmung geduldet. Jegliche Form von Data-Mining stellt keine gemäßigte Nutzung dar. Wir behalten uns vor, die Duldung grundsätzlich oder in Einzelfällen zu widerrufen. Fragen richten Sie bitte gerne an: <redaktion@mvg.de>"

By using this tool, you agree to limit your request rates ("moderate use") and to use the data solely for private, non-commercial purposes unless you have obtained express consent from MVG.

## License

MIT License. See [LICENSE](./LICENSE) for details.
