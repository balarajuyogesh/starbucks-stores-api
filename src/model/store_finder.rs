use crate::model::store_location::StoreLocationItem;
use crate::types::error::CustomError;
use dynomite::{
  attr_map,
  dynamodb::{DynamoDbClient, QueryInput},
  DynamoDbExt, FromAttributes,
};
use futures::{future, TryStreamExt};
use serde::Serialize;
use std::fmt::Write;

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct StoreResult {
  #[serde(rename = "storeNumber")]
  pub store_number: String,
  #[serde(rename = "storeName")]
  pub store_name: String,
  #[serde(rename = "streetAddress")]
  pub street_address: String,
  #[serde(rename = "city")]
  pub city: String,
  #[serde(rename = "state")]
  pub state: String,
  #[serde(rename = "country")]
  pub country: String,
  #[serde(rename = "postcode")]
  pub postcode: String,
  #[serde(rename = "longitude")]
  pub longitude: String,
  #[serde(rename = "latitude")]
  pub latitude: String,
}

impl StoreResult {
  pub fn new(item: &StoreLocationItem) -> StoreResult {
    StoreResult {
      store_number: item.store_number.clone(),
      store_name: item.store_name.clone(),
      street_address: item.street_address.clone(),
      city: item.city.clone(),
      state: item.state.clone(),
      country: item.country.clone(),
      postcode: item.postcode.clone(),
      longitude: item.longitude.clone(),
      latitude: item.latitude.clone(),
    }
  }
}

pub async fn search_store(
  client: &DynamoDbClient,
  country: String,
  state: String,
  city: Option<String>,
  postcode: Option<String>,
) -> Result<Vec<StoreResult>, CustomError> {
  let mut results = vec![];
  let mut state_city_postcode = state;
  match city {
    Some(val) => write!(&mut state_city_postcode, "#{}", val).unwrap(),
    _ => (),
  };

  match postcode {
    Some(val) => write!(&mut state_city_postcode, "#{}", val).unwrap(),
    _ => (),
  };

  let table_name = "StarbucksLocations";

  client
    .clone()
    .query_pages(QueryInput {
      limit: Some(100),
      table_name: table_name.clone().into(),
      index_name: Some("StoreLocationIndex".into()),
      key_condition_expression: Some(
        "Country = :country AND begins_with(StateCityPostcode, :statecitypostcode)".into(),
      ),
      expression_attribute_values: Some(attr_map!(
          ":country" => country.to_string(),
          ":statecitypostcode" => state_city_postcode.to_string(),
      )),
      ..QueryInput::default()
    })
    .try_for_each(|item| {
      results.push(StoreResult::new(
        &StoreLocationItem::from_attrs(item).unwrap(),
      ));
      future::ready(Ok(()))
    })
    .await.map_err(|_| CustomError::DBError)?;

  Ok(results)
}
