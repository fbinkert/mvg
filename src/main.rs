use std::{
    fs::{self, create_dir_all, read_to_string},
    path::PathBuf,
};

use ::futures::future;
use chrono::{DateTime, Local};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;
use mvg::{client::MvgClient, errors::MvgError, models::Departure};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Favorite {
    pub alias: String,
    pub station_name: String,
    pub station_id: String,
    pub direction_filter: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppConfig {
    pub favorites: Vec<Favorite>,
}

impl AppConfig {
    fn get_config_path() -> std::path::PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "mvg", "cli") {
            proj_dirs.config_dir().join("config.json")
        } else {
            PathBuf::from("mvg_config.json")
        }
    }

    pub fn load() -> Self {
        let path = Self::get_config_path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str::<Self>(&content) {
                return config;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::get_config_path();
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }
}

#[derive(Parser)]
#[command(name = "mvg")]
#[command(about = "Munich Public Transport CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lookup departures for a specific station
    Departures(DeparturesArgs),

    /// Fetch all favorites and display them
    Monitor,

    /// Manage your favorite stations
    Fav(FavArgs),
}

#[derive(Args)]
pub struct DeparturesArgs {
    /// Station name or Global ID
    pub station: String,

    /// Limit number of results
    #[arg(short, long, default_value_t = 5)]
    pub limit: usize,
}

#[derive(Args)]
pub struct FavArgs {
    #[command(subcommand)]
    pub command: FavCommands,
}

#[derive(Subcommand)]
pub enum FavCommands {
    /// List all saved favorites
    List,

    /// Add a new favorite
    Add {
        /// The name of the station to search for
        station_query: String,

        /// A short name for this favorite (e.g. "Work")
        #[arg(short, long)]
        alias: String,

        /// Only show departures heading towards this direction (substring match)
        #[arg(short, long)]
        direction: Option<String>,
    },

    /// Remove a favorite by alias
    Remove { alias: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = MvgClient::new();
    let mut config = AppConfig::load();

    match cli.command {
        Commands::Departures(args) => {
            if let Some(station) = client.get_station_by_name(&args.station).await? {
                println!("📍Found Station: {}", station.name.cyan());

                let deps = client
                    .get_departures(&station.id, args.limit, 0, None)
                    .await?;

                println!("\nNext Departures:");
                for d in deps {
                    let time = DateTime::from_timestamp(d.time_ms / 1000, 0)
                        .map(|dt| dt.with_timezone(&Local))
                        .unwrap_or_default();

                    let now = Local::now();
                    let diff = time.signed_duration_since(now).num_minutes();
                    let relative = if diff <= 0 {
                        "now".to_string()
                    } else {
                        format!("in {} min", diff)
                    };

                    println!(
                        "[{}] {:<5} -> {:<20} ({}, Delay: {}m)",
                        time.format("%H:%M"),
                        d.label,
                        d.destination,
                        relative,
                        d.delay_in_minutes.unwrap_or(0)
                    );
                }
            } else {
                println!("{} Station '{}' not found.", "❌".red(), args.station);
            }
        }
        Commands::Monitor => {
            if config.favorites.is_empty() {
                println!("No favorites configured...");
                return Ok(());
            }

            println!("-------- {} --------", "MVG MONITOR".blue().bold());

            let tasks: Vec<_> = config
                .favorites
                .iter()
                .map(|fav| get_monitor_data(&client, fav))
                .collect();

            let results = future::join_all(tasks).await;

            for (i, result) in results.into_iter().enumerate() {
                let fav = &config.favorites[i];

                println!("\n📍 {} ({})", fav.alias.green().bold(), fav.station_name);

                if let Some(dir) = &fav.direction_filter {
                    println!("   Filter: Only towards '{}'", dir.yellow());
                }

                match result {
                    Ok((_, deps)) => {
                        if deps.is_empty() {
                            println!("   No departures found.");
                        } else {
                            print_departures(&deps);
                        }
                    }
                    Err(_) => println!("   {}", "Failed to fetch data".red()),
                }
            }
        }

        Commands::Fav(args) => match args.command {
            FavCommands::List => {
                println!("Saved Favorites:");
                for (i, f) in config.favorites.iter().enumerate() {
                    let dir_info = f.direction_filter.as_deref().unwrap_or("Any");
                    println!(
                        "{}. {} -> {} [Direction: {}]",
                        i + 1,
                        f.alias.green().bold(),
                        f.station_name,
                        dir_info.yellow()
                    );
                }
            }
            FavCommands::Add {
                station_query,
                alias,
                direction,
            } => match client.get_station_by_name(&station_query).await? {
                Some(station) => {
                    let new_fav = Favorite {
                        alias: alias.clone(),
                        station_name: station.name,
                        station_id: station.id,
                        direction_filter: direction,
                    };
                    config.favorites.push(new_fav);
                    config.save()?;
                    println!("{} Added '{}'", "✔".green(), alias);
                }
                None => {
                    println!(
                        "{} Station '{}' not found. Favorite not added.",
                        "❌".red(),
                        station_query
                    );
                }
            },
            FavCommands::Remove { alias } => {
                let initial_len = config.favorites.len();
                config.favorites.retain(|f| f.alias != alias);

                if config.favorites.len() < initial_len {
                    config.save()?;
                    println!("{} Removed '{}'", "✔".green(), alias);
                } else {
                    println!("{} Alias '{}' not found.", "⚠".yellow(), alias);
                }
            }
        },
    }
    Ok(())
}

async fn get_monitor_data(
    client: &MvgClient,
    fav: &Favorite,
) -> Result<(String, Vec<Departure>), MvgError> {
    let deps = client.get_departures(&fav.station_id, 30, 0, None).await?;

    let filtered_deps: Vec<Departure> = deps
        .into_iter()
        .filter(|d| match &fav.direction_filter {
            Some(filter) => d
                .destination
                .to_lowercase()
                .contains(&filter.to_lowercase()),
            None => true,
        })
        .take(3)
        .collect();

    Ok((fav.alias.clone(), filtered_deps))
}

fn print_departures(deps: &[Departure]) {
    for d in deps {
        let time = DateTime::from_timestamp(d.time_ms / 1000, 0)
            .map(|dt| dt.with_timezone(&Local))
            .unwrap_or_default();

        let now = Local::now();
        let diff = time.signed_duration_since(now).num_minutes();
        let time_display = if diff <= 0 {
            "Now".to_string()
        } else {
            format!("{} min", diff)
        };

        let delay = d.delay_in_minutes.unwrap_or(0);
        let delay_str = if delay > 0 {
            format!("+{}", delay).red()
        } else {
            format!("+{}", delay).green()
        };

        println!(
            "   [{}] {:<4} {:<25} {:>8} (Delay: {})",
            time.format("%H:%M").to_string().cyan(), // Absolute time
            d.label.bold(),                          // Line (e.g., "S7")
            d.destination,                           // Destination
            time_display.bold(),                     // Relative time
            delay_str                                // Delay
        );
    }
}
