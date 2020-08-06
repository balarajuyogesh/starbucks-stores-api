extern crate dotenv;
extern crate envy;
extern crate serde_derive;
extern crate starbucks_stores_api;

use dynomite::{
  dynamodb::{DynamoDb, DynamoDbClient, PutItemInput},
  retry::Policy,
  Retries,
};
use rusoto_core::Region;

use starbucks_stores_api::config::get_config;

use csv::Reader;
use starbucks_stores_api::model::store_location::{StoreLocation, StoreLocationItem};
use std::error::Error;
use std::io;
use std::process;

async fn load() -> Result<(), Box<dyn Error>> {
  let table_name = "StarbucksLocations";

  println!("Loading to Table: {}", table_name);

  let environment = match get_config() {
    Ok(environment) => environment,
    Err(_) => panic!("No environment details found"),
  };

  let client = DynamoDbClient::new(Region::Custom {
    name: "us-east-1".into(),
    endpoint: environment.endpoint.into(),
  })
  .with_retries(Policy::default());

  let mut counter = 0;

  // Build the CSV reader and iterate over each record.
  let mut rdr = Reader::from_reader(io::stdin());
  let mut iter = rdr.deserialize();

  while let Some(result) = iter.next() {
    let record: StoreLocation = result?;
    let record_item = StoreLocationItem::new(&record);

    client
      .put_item(PutItemInput {
        table_name: table_name.clone().into(),
        item: record_item.clone().into(),
        ..PutItemInput::default()
      })
      .await?;

    counter = counter + 1;

    if counter % 100 == 0 {
      println!("Inserted {} records", counter);
    }
  }

  println!(
    "Loading complete. {} records loaded to {}",
    counter, table_name
  );

  Ok(())
}

#[tokio::main]
async fn main() {
  if let Err(err) = load().await {
    println!("error running load: {}", err);
    process::exit(1);
  }
}
