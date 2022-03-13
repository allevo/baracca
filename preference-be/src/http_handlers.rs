use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::{hyper::StatusCode, path::FullPath, Filter, Rejection, Reply};

use crate::{
    discovery_service::{DiscoveryError, DiscoveryResult, DiscoveryService},
    house_service::{
        HouseDTO, HouseDTOInsert, HouseDTOInserted, HousesService, HousesServiceError,
        UpdateHouseDTO,
    },
};

pub async fn insert_house(
    request_body: HouseDTOInsert,
    houses_service: HousesService,
) -> Result<impl warp::Reply, Rejection> {
    Ok(houses_service.insert_house(request_body).await?)
}

pub async fn remove_house(
    house_id: String,
    houses_service: HousesService,
) -> Result<impl warp::Reply, Rejection> {
    houses_service.remove_house(house_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_houses(houses_service: HousesService) -> Result<impl warp::Reply, Rejection> {
    let houses = houses_service.get_houses().await?;
    Ok(warp::reply::json(&houses))
}

pub async fn get_house_by_id(
    house_id: String,
    houses_service: HousesService,
) -> Result<impl warp::Reply, Rejection> {
    let house = houses_service.get_house_by_id(house_id).await?;
    Ok(warp::reply::json(&house))
}

pub async fn update_house_by_id(
    house_id: String,
    update_field: UpdateHouseDTO,
    houses_service: HousesService,
) -> Result<impl warp::Reply, Rejection> {
    let house = houses_service
        .update_house_by_id(house_id, update_field)
        .await?;
    Ok(warp::reply::json(&house))
}

pub async fn discover(
    discovery_service: DiscoveryService,
    params: DiscoverQueryParameter,
) -> Result<impl warp::Reply, Rejection> {
    Ok(discovery_service.discover(&params.url).await?)
}

#[derive(Deserialize)]
pub struct DiscoverQueryParameter {
    url: String,
}

pub fn log_req() -> impl Filter<Extract = (), Error = Infallible> + Copy {
    warp::path::full()
        .map(|path: FullPath| {
            println!("path={}", path.as_str());
        })
        .untuple_one()
}

/// Error handler. This could be written better for handling other errors,
/// like "405: Method not allowed", or malformed JSON etc....
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let code;
    let message;

    warn!("Handle rejection: {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_owned();
    } else if let Some(err) = err.find::<HousesServiceError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = format!("{:?}", err);
    } else if let Some(err) = err.find::<DiscoveryError>() {
        code = StatusCode::NOT_FOUND;
        message = format!("{:?}", err);
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_owned();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

impl warp::reject::Reject for HousesServiceError {}
impl warp::reject::Reject for DiscoveryError {}

/// Needed for returning the structures directly from the handlers
impl warp::Reply for HouseDTOInserted {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

/// Needed for returning the structures directly from the handlers
impl warp::Reply for HouseDTO {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

/// Needed for returning the structures directly from the handlers
impl warp::Reply for DiscoveryResult {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}
