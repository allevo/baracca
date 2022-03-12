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
            }
        );
    }
}
