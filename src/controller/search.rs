use serde_derive::Deserialize;
use dynomite::dynamodb::DynamoDbClient;
use warp::{reject, reply::json, Reply, Rejection};

use crate::model::store_finder::search_store;

type Result<T> = std::result::Result<T, Rejection>;

#[derive(Deserialize)]
pub struct SearchQuery {
    country: String,
    state: String,
    city: Option<String>,
    postcode: Option<String>,
}

pub async fn search_stores(query: SearchQuery, client: DynamoDbClient) -> Result<impl Reply> {
  let stores = search_store(&client, query.country, query.state, query.city, query.postcode)
      .await
      .map_err(|e| reject::custom(e))?;
  Ok(json::<Vec<_>>(&stores))
}