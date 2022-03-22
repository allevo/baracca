use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DiscoveryError {
    NotFound(String),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for DiscoveryError {
    fn from(r: reqwest::Error) -> Self {
        Self::ReqwestError(r)
    }
}

#[derive(Clone)]
pub struct DiscoveryService;

impl DiscoveryService {
    pub fn new() -> Self {
        Self
    }

    pub async fn discover(&self, url: &str) -> Result<DiscoveryResult, DiscoveryError> {
        info!("discover url={}", url);
        fetch_data(url).await
    }
}

async fn fetch_data(s: &str) -> Result<DiscoveryResult, DiscoveryError> {
    let body = reqwest::get(s).await?;

    if !body.status().is_success() {
        return Err(DiscoveryError::NotFound(s.to_owned()));
    }

    let body = body.text().await?;

    let mut discovery_result = DiscoveryResult::default();

    extract_planimetry(&body, &mut discovery_result);
    extract_data(&body, &mut discovery_result);
    extract_prices(&body, &mut discovery_result);

    extract2(&body, &mut discovery_result);

    Ok(discovery_result)
}

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct DiscoveryResult {
    city: Option<String>,
    zone: Option<String>,
    street: Option<String>,
    lat: Option<f64>,
    lng: Option<f64>,
    rooms_number: Option<u8>,
    square_meters: Option<u32>,
    cost: Option<u32>,
}

fn extract2(body: &str, discovery_result: &mut DiscoveryResult) {
    let rooms_number = body.lines().find(|l| l.contains(" locali<")).and_then(|l| {
        l.replace("<li>", "")
            .replace("</li>", "")
            .replace(" locali", "")
            .trim()
            .parse::<u8>()
            .ok()
    });
    let square_meters = body
        .lines()
        .find(|l| l.contains("m²") && l.contains("<li>"))
        .and_then(|l| {
            l.split_once("m²")
                .and_then(|l| l.0.replace("<li>", "").trim().parse::<u32>().ok())
        });
    let cost = body.lines().find(|l| l.contains("€/mese")).and_then(|l| {
        l.replace("<strong class=\"price\">", "")
            .replace("</strong>", "")
            .replace("€/mese", "")
            .replace(".", "")
            .trim()
            .parse::<u32>()
            .ok()
    });

    let lat = body
        .lines()
        .find(|l| l.contains("latitude: '"))
        .and_then(|l| {
            l.replace("latitude: '", "")
                .replace("',", "")
                .trim()
                .parse::<f64>()
                .ok()
        });
    let lng = body
        .lines()
        .find(|l| l.contains("longitude: '"))
        .and_then(|l| {
            l.replace("longitude: '", "")
                .replace("',", "")
                .trim()
                .parse::<f64>()
                .ok()
        });
    let m: Vec<_> = body
        .lines()
        .tuple_windows()
        .filter(|(l, _)| l.contains("header-map-list"))
        .map(|(_, l)| l)
        .collect();
    let street = m.first().map(|l| l.trim().to_string());
    let city = m.last().map(|l| l.trim().to_string());
    let zone = m.get(1).map(|l| l.trim().to_string());

    discovery_result.city = discovery_result.city.clone().or(city);
    discovery_result.zone = discovery_result.zone.clone().or(zone);
    discovery_result.street = discovery_result.street.clone().or(street);
    discovery_result.lat = discovery_result.lat.or(lat);
    discovery_result.lng = discovery_result.lng.or(lng);
    discovery_result.rooms_number = discovery_result.rooms_number.or(rooms_number);
    discovery_result.square_meters = discovery_result.square_meters.or(square_meters);
    discovery_result.cost = discovery_result.cost.or(cost);
}

fn extract_data(body: &str, discovery_result: &mut DiscoveryResult) {
    let position_description: Option<_> = body.lines().find(|l| l.contains("id=\"js-hydration\">"));

    let position_description = match position_description {
        None => return,
        Some(position_description) => position_description,
    };

    let (start, end) = match (
        position_description.find('>'),
        position_description.rfind('<'),
    ) {
        (Some(start), Some(end)) => (start, end),
        _ => return,
    };

    let position_description = &position_description[(start + 1)..end];

    let mut map_config: MapConfig = match serde_json::from_str(position_description) {
        Err(_) => return,
        Ok(mc) => mc,
    };

    let p = match map_config.listing.properties.pop() {
        None => return,
        Some(p) => p,
    };
    let location = match p.location {
        None => return,
        Some(location) => location,
    };

    discovery_result.city = location.city.and_then(|c| c.name);
    discovery_result.lat = discovery_result.lat.or(location.latitude);
    discovery_result.lng = discovery_result.lng.or(location.longitude);
    discovery_result.street = location
        .address
        .and_then(|s| location.street_number.map(|n| format!("{}, {}", s, n)));
    discovery_result.zone = location.microzone.and_then(|c| c.name);
}

fn extract_planimetry(body: &str, discovery_result: &mut DiscoveryResult) {
    let lines: Vec<_> = body
        .lines()
        .tuple_windows::<(_, _)>()
        .filter(|(_, l)| {
            l.contains("=\"im-mainFeatures__label\">locali")
                || l.contains("=\"im-mainFeatures__label\">superficie")
        })
        .map(|(l, _)| l)
        .collect();

    discovery_result.rooms_number = lines
        .get(0)
        .map(|l| l.to_string().trim().parse::<u8>().unwrap());
    discovery_result.square_meters = lines
        .get(1)
        .map(|l| l.to_string().trim().parse::<u32>().unwrap());
}

fn extract_prices(body: &str, discovery_result: &mut DiscoveryResult) {
    let lines: Vec<_> = body
        .lines()
        .tuple_windows::<(_, _, _)>()
        .filter(|(l, _, _)| l.contains(">prezzo<") || l.contains(">spese condominio<"))
        .filter(|(_, _, l)| l.contains("€"))
        .map(|(_, _, l)| l.trim())
        .collect();

    let cost: u32 = lines
        .into_iter()
        .filter_map(|l| {
            l.to_string()
                .replace("€", "")
                .replace(".", "")
                .replace("/mese", "")
                .trim()
                .parse::<u32>()
                .ok()
        })
        .sum();

    if cost > 0 {
        discovery_result.cost = Some(cost);
    }
}

#[derive(Deserialize)]
struct MapConfig {
    listing: Listing,
}
#[derive(Deserialize, Debug)]
struct Listing {
    properties: Vec<Property>,
}
#[derive(Deserialize, Debug)]
struct Property {
    location: Option<Location>,
    // surfaceValue: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Location {
    latitude: Option<f64>,
    longitude: Option<f64>,
    address: Option<String>,
    #[serde(rename(deserialize = "streetNumber"))]
    street_number: Option<String>,
    microzone: Option<MicroZone>,
    city: Option<MicroZone>,
}
#[derive(Deserialize, Serialize, Debug)]
struct MicroZone {
    name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_flow() {
        use httpmock::prelude::*;
        let server = MockServer::start();

        let url = "/foo";
        let server_mock = server.mock(|when, then| {
            when.method(GET).path(url);
            then.status(200)
                .header("content-type", "text/html")
                .body(include_str!("page.test.html"));
        });
        let service = DiscoveryService;
        let a = service.discover(&server.url(url)).await.unwrap();

        server_mock.assert();

        assert_eq!(
            a,
            DiscoveryResult {
                city: Some("Milano".to_string()),
                zone: Some("Dergano".to_string()),
                street: Some("Via Pellegrino Rossi, 13".to_string()),
                lat: Some(45.5081),
                lng: Some(9.1775),
                rooms_number: Some(2),
                square_meters: Some(60),
                cost: Some(2100)
            }
        );
    }

    #[tokio::test]
    async fn test_flow2() {
        use httpmock::prelude::*;
        let server = MockServer::start();

        let url = "/foo";
        let server_mock = server.mock(|when, then| {
            when.method(GET).path(url);
            then.status(200)
                .header("content-type", "text/html")
                .body(include_str!("page2.test.html"));
        });
        let service = DiscoveryService;
        let a = service.discover(&server.url(url)).await.unwrap();

        server_mock.assert();

        assert_eq!(
            a,
            DiscoveryResult {
                city: Some("Milano".to_string()),
                zone: Some("Area Residenziale de angeli".to_string()),
                street: Some("Viale Ranzoni, 19".to_string()),
                lat: Some(45.4688239),
                lng: Some(9.1451057),
                rooms_number: Some(2),
                square_meters: Some(70),
                cost: Some(1200)
            }
        );
    }
}
