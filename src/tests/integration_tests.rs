use crate::client::MvgClient;

#[tokio::test]
#[ignore]
async fn test_live_list_stations() {
    let client = MvgClient::new();
    let result = client.list_stations().await;

    assert!(
        result.is_ok(),
        "Failed to fetch station list: {:?}",
        result.err()
    );
    let stations = result.unwrap();
    assert!(!stations.is_empty());
    // Verify a known station exists in the master list
    assert!(stations.iter().any(|s| s.name.contains("Hauptbahnhof")));
}

#[tokio::test]
#[ignore]
async fn test_live_get_station_by_query() {
    let client = MvgClient::new();
    // Search for "Odeonsplatz"
    let result = client.get_station_by_name("Odeonsplatz").await.unwrap();

    assert!(result.is_some());
    let station = result.unwrap();
    assert!(station.name.contains("Odeonsplatz"));
    assert_eq!(station.place, "München");
}

#[tokio::test]
#[ignore]
async fn test_live_get_station_by_id() {
    let client = MvgClient::new();
    // "de:09162:2" is Marienplatz
    let station = client.get_station_by_id("de:09162:2").await.unwrap();
    assert_eq!(station.id, "de:09162:2");
    assert!(station.name.contains("Marienplatz"));
}

#[tokio::test]
#[ignore]
async fn test_live_get_departures() {
    let client = MvgClient::new();
    // Fetch departures for Sendlinger Tor (de:09162:1)
    let result = client.get_departures("de:09162:1", 5, 0, None).await;

    assert!(result.is_ok(), "API error: {:?}", result.err());
    let departures = result.unwrap();
    // Even late at night, Sendlinger Tor usually has something in the queue
    assert!(!departures.is_empty());
}

#[tokio::test]
#[ignore]
async fn test_live_nearby_stations() {
    let client = MvgClient::new();
    // Coordinates for the MVG Headquarters (Emmy-Noether-Straße 2)
    let lat = 48.17143;
    let lon = 11.53096;

    let result = client.nearby(lat, lon).await.unwrap();
    assert!(!result.is_empty());
    // "Westfriedhof" should be nearby
    assert!(result.iter().any(|s| s.name.contains("Westfriedhof")));
}

#[tokio::test]
#[ignore]
async fn test_live_get_lines() {
    let client = MvgClient::new();
    let result = client.get_lines().await;

    assert!(result.is_ok());
    let lines = result.unwrap();
    assert!(!lines.is_empty());
    // Check for a known line, e.g., "U3" or "U6"
    assert!(lines.iter().any(|l| l.name.as_deref() == Some("U3")));
}

#[tokio::test]
#[ignore]
async fn test_live_get_global_ids() {
    let client = MvgClient::new();
    let result = client.get_station_global_ids().await;

    assert!(result.is_ok());
    let ids = result.unwrap();
    assert!(!ids.is_empty());
    // Verify a known Global ID format is present
    assert!(ids.contains(&"de:09162:6".to_string()));
}
