extern crate dotenv;
extern crate envy;
extern crate serde_derive;
extern crate starbucks_stores_api;

use dynomite::{
    dynamodb::{
        AttributeDefinition, CreateTableInput, DynamoDb, DynamoDbClient, GlobalSecondaryIndex,
        KeySchemaElement, Projection, ProvisionedThroughput,
    },
    retry::Policy,
    Retries,
};
use rusoto_core::Region;

use starbucks_stores_api::config::get_config;

/// Creates StarbucksLocations table with indices 
#[tokio::main]
async fn main() {
    let table_name = "StarbucksLocations";

    println!("Creating Table: {}", table_name);

    let environment = match get_config() {
        Ok(environment) => environment,
        Err(_) => panic!("No environment details found"),
    };

    let client = DynamoDbClient::new(Region::Custom {
        name: "us-east-1".into(),
        endpoint: environment.endpoint.into(),
    })
    .with_retries(Policy::default());

    match client
        .create_table(CreateTableInput {
            table_name: table_name.into(),
            key_schema: vec![KeySchemaElement {
                attribute_name: "StoreNumber".into(),
                key_type: "HASH".into(),
            }],
            attribute_definitions: vec![
                AttributeDefinition {
                    attribute_name: "Country".into(),
                    attribute_type: "S".into(),
                },
                AttributeDefinition {
                    attribute_name: "StateCityPostcode".into(),
                    attribute_type: "S".into(),
                },
                AttributeDefinition {
                    attribute_name: "StoreNumber".into(),
                    attribute_type: "S".into(),
                },
            ],
            global_secondary_indexes: Some(vec![GlobalSecondaryIndex {
                index_name: "StoreLocationIndex".into(),
                key_schema: vec![
                    KeySchemaElement {
                        attribute_name: "Country".into(),
                        key_type: "HASH".into(),
                    },
                    KeySchemaElement {
                        attribute_name: "StateCityPostcode".into(),
                        key_type: "RANGE".into(),
                    },
                ],
                projection: Projection {
                    projection_type: Some("ALL".into()),
                    non_key_attributes: None,
                },
                provisioned_throughput: Some(ProvisionedThroughput {
                    read_capacity_units: 1,
                    write_capacity_units: 1,
                }),
            }]),
            provisioned_throughput: Some(ProvisionedThroughput {
                read_capacity_units: 1,
                write_capacity_units: 1,
            }),
            ..CreateTableInput::default()
        })
        .await
    {
        Ok(output) => println!("Table created successfully: {:#?}", output),
        Err(e) => println!("Failed to create table: {}", e),
    }
}
