use std::str::FromStr;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::UpdateOptions,
    results::InsertOneResult,
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct HousesService {
    collection: Collection<HouseEntity>,
}

type Result<T> = std::result::Result<T, HousesServiceError>;

#[derive(Debug)]
pub enum HousesServiceError {
    MongoDbError(mongodb::error::Error),
    ObjectId(mongodb::bson::oid::Error),
    HouseNotFound(String),
    UnExpectedMongoDbType,
}

impl From<mongodb::error::Error> for HousesServiceError {
    fn from(e: mongodb::error::Error) -> Self {
        Self::MongoDbError(e)
    }
}
impl From<mongodb::bson::oid::Error> for HousesServiceError {
    fn from(e: mongodb::bson::oid::Error) -> Self {
        Self::ObjectId(e)
    }
}

impl HousesService {
    pub fn new(collection: Collection<HouseEntity>) -> Self {
        Self { collection }
    }

    pub async fn insert_house(&self, house: HouseDTOInsert) -> Result<HouseDTOInserted> {
        let house: HouseEntity = house.into();

        info!("Inserting....");
        let res = self.collection.insert_one(house, None).await?;
        info!("Inserted");

        res.try_into()
    }

    pub async fn remove_house(&self, house_id: String) -> Result<()> {
        let id = ObjectId::from_str(&house_id)?;

        let res = self
            .collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "removed": true } },
                UpdateOptions::default(),
            )
            .await?;

        if res.modified_count == 0 {
            return Err(HousesServiceError::HouseNotFound(house_id));
        }

        Ok(())
    }

    pub async fn get_houses(&self) -> Result<Vec<HouseDTO>> {
        let cur = self
            .collection
            .find(
                doc! {
                    "removed": false
                },
                None,
            )
            .await?;
        let houses: Vec<HouseEntity> = cur.try_collect().await?;
        let houses = houses.into_iter().map(HouseDTO::from).collect();

        Ok(houses)
    }

    pub async fn get_house_by_id(&self, id: String) -> Result<HouseDTO> {
        let obj_id = ObjectId::from_str(&id)?;

        let ret = self
            .collection
            .find_one(
                doc! {
                    "_id": obj_id,
                },
                None,
            )
            .await?;

        let h = match ret {
            None => return Result::Err(HousesServiceError::HouseNotFound(id)),
            Some(h) => h,
        };

        Ok(h.into())
    }

    pub async fn update_house_by_id(&self, id: String, update_field: UpdateHouseDTO) -> Result<()> {
        let obj_id = ObjectId::from_str(&id)?;

        let filter = doc! {"_id": obj_id};
        let update = doc! { "$set": {
            "vote": update_field.vote,
            "comment": update_field.comment,
        } };

        let ret = self
            .collection
            .find_one_and_update(filter, update, None)
            .await?;

        if ret.is_none() {
            return Result::Err(HousesServiceError::HouseNotFound(id));
        }

        Ok(())
    }
}

#[derive(Deserialize, Default)]
pub struct HouseDTOInsert {
    pub link: String,
    pub vote: Option<u8>,
    pub comment: Option<String>,

    // Came from discovery_service::DiscoveryResult
    pub city: Option<String>,
    pub zone: Option<String>,
    pub street: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub rooms_number: Option<u8>,
    pub square_meters: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HouseEntity {
    _id: ObjectId,
    link: String,
    vote: Option<u8>,
    comment: Option<String>,

    removed: bool,

    // Came from discovery_service::DiscoveryResult
    city: Option<String>,
    zone: Option<String>,
    street: Option<String>,
    lat: Option<f64>,
    lng: Option<f64>,
    rooms_number: Option<u8>,
    square_meters: Option<u32>,
}

impl From<HouseDTOInsert> for HouseEntity {
    fn from(h: HouseDTOInsert) -> Self {
        Self {
            _id: ObjectId::new(),
            link: h.link,
            vote: h.vote,
            comment: h.comment,
            removed: false,
            city: h.city,
            zone: h.zone,
            street: h.street,
            lat: h.lat,
            lng: h.lng,
            rooms_number: h.rooms_number,
            square_meters: h.square_meters,
        }
    }
}

#[derive(Serialize)]
pub struct HouseDTOInserted {
    pub id: String,
}

impl TryFrom<InsertOneResult> for HouseDTOInserted {
    type Error = HousesServiceError;

    fn try_from(value: InsertOneResult) -> Result<Self> {
        match value.inserted_id.as_object_id() {
            Some(id) => Ok(HouseDTOInserted { id: id.to_hex() }),
            None => Err(HousesServiceError::UnExpectedMongoDbType),
        }
    }
}

#[derive(Serialize)]
pub struct HouseDTO {
    pub id: String,
    pub link: String,
    pub vote: Option<u8>,
    pub comment: Option<String>,

    // Came from discovery_service::DiscoveryResult
    city: Option<String>,
    zone: Option<String>,
    street: Option<String>,
    lat: Option<f64>,
    lng: Option<f64>,
    rooms_number: Option<u8>,
    square_meters: Option<u32>,
}

impl From<HouseEntity> for HouseDTO {
    fn from(e: HouseEntity) -> Self {
        Self {
            id: e._id.to_hex(),
            link: e.link,
            vote: e.vote,
            comment: e.comment,
            city: e.city,
            zone: e.zone,
            street: e.street,
            lat: e.lat,
            lng: e.lng,
            rooms_number: e.rooms_number,
            square_meters: e.square_meters,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateHouseDTO {
    comment: Option<String>,
    vote: Option<i32>,
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use mongodb::options::{ClientOptions, ResolverConfig};
    use mongodb::Client;

    use super::*;

    #[tokio::test]
    async fn test_flow() {
        pretty_env_logger::try_init().ok();

        let collection = connect_mongo().await;

        let service = HousesService::new(collection);

        let house = HouseDTOInsert {
            link: "http://the.link/foo".to_string(),
            vote: Some(0),
            comment: Some("the-comment".to_string()),
            ..HouseDTOInsert::default()
        };

        let id = service.insert_house(house).await.unwrap();
        let id = id.id;

        let houses = service.get_houses().await.unwrap();

        assert_eq!(houses.len(), 1);
        houses.iter().find(|h| h.id == id).unwrap();

        service.remove_house(id).await.unwrap();

        let houses = service.get_houses().await.unwrap();
        assert_eq!(houses.len(), 0);
    }

    async fn connect_mongo() -> mongodb::Collection<HouseEntity> {
        let mut client_options = ClientOptions::parse_with_resolver_config(
            get_mongo_url(),
            ResolverConfig::cloudflare(),
        )
        .await
        .unwrap();

        client_options.app_name = Some("preference-be-test".to_string());

        let client = Client::with_options(client_options).unwrap();

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let now = now.as_secs();
        let database_name = format!("test-{}", now);
        let db = client.database(&database_name);

        db.collection("houses")
    }

    fn get_mongo_url() -> String {
        let host = std::env::var("MONGODB_HOST").unwrap_or("localhost".to_string());
        let port = std::env::var("MONGODB_PORT").unwrap_or("27017".to_string());
        format!("mongodb://{}:{}/test", host, port)
    }
}
