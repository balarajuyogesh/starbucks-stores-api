use dynomite::Item;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct StoreLocation {
  #[serde(rename = "Brand")]
  pub brand: String,
  #[serde(rename = "Store Number")]
  pub store_number: String,
  #[serde(rename = "Store Name")]
  pub store_name: String,
  #[serde(rename = "Ownership Type")]
  pub ownership_type: String,
  #[serde(rename = "Street Address")]
  pub street_address: String,
  #[serde(rename = "City")]
  pub city: String,
  #[serde(rename = "State/Province")]
  pub state: String,
  #[serde(rename = "Country")]
  pub country: String,
  #[serde(rename = "Postcode")]
  pub postcode: String,
  #[serde(rename = "Phone Number")]
  pub phone_number: String,
  #[serde(rename = "Timezone")]
  pub timezone: String,
  #[serde(rename = "Longitude")]
  pub longitude: String,
  #[serde(rename = "Latitude")]
  pub latitude: String,
}

#[derive(Item, Debug, Clone)]
pub struct StoreLocationItem {
  #[dynomite(partition_key, rename = "StoreNumber")]
  pub store_number: String,
  #[dynomite(rename = "StoreName", default)]
  pub store_name: String,
  #[dynomite(rename = "StreetAddress", default)]
  pub street_address: String,
  #[dynomite(rename = "City", default)]
  pub city: String,
  #[dynomite(rename = "State", default)]
  pub state: String,
  #[dynomite(rename = "Country", default)]
  pub country: String,
  #[dynomite(rename = "Postcode", default)]
  pub postcode: String,
  #[dynomite(rename = "PhoneNumber", default)]
  pub phone_number: String,
  #[dynomite(rename = "Longitude", default)]
  pub longitude: String,
  #[dynomite(rename = "Latitude", default)]
  pub latitude: String,
  #[dynomite(rename = "StateCityPostcode", default)]
  pub state_city_postcode: String,
}

impl StoreLocationItem {
  pub fn new(row: &StoreLocation) -> StoreLocationItem {
    StoreLocationItem {
      store_number: row.store_number.clone(),
      store_name: row.store_name.clone(),
      street_address: row.street_address.clone(),
      city: row.city.clone(),
      state: row.state.clone(),
      country: row.country.clone(),
      postcode: row.postcode.clone(),
      phone_number: row.phone_number.clone(),
      longitude: row.longitude.clone(),
      latitude: row.latitude.clone(),
      state_city_postcode: format!("{}#{}#{}", row.state.to_uppercase(), row.city.to_uppercase(), row.postcode.to_uppercase()).into(),
    }
  }
}
