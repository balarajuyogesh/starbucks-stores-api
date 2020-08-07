# Starbuck Stores API

Rust implementation to query Starbucks store, based on Alex DeBrie's amazing [DynamoDB Guide](https://www.dynamodbguide.com/hierarchical-data)

## Data Source

Data downloaded from [Kaggle](https://www.kaggle.com/starbucks/store-locations)

## DyanmoDB setup

I prefer using local DynamoDB for testing purposes. To setup your local DynamoDB, follow the [instructions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.DownloadingAndRunning.html)

## Preparing the environment

Create a `.env` in your project root and add `endpoint` for your Local DynamoDB

```text
endpoint=http://localhost:9090/
```

Build the project

```rust
cargo build
```

Create the Dynamo Table

```rust
cargo run --bin create
```

Load the data

```rust
cargo run --bin load < src/bin/directory.csv
```

Cool now you are all set to run your API.

## Running the API

Start the API

```rust
cargo run --bin web
```

## Query Stores

### Find all stores in a state

```curl
curl http://127.0.0.1:9000/search?country=US&state=NY
```

```json
[
  {
        "storeNumber": "28442-249823",
        "storeName": "Teavana - Crossgates Mall",
        "streetAddress": "1 CROSSGATE MALL RD, #B205A",
        "city": "ALBANY",
        "state": "NY",
        "country": "US",
        "postcode": "12203",
        "longitude": "-73.85",
        "latitude": "42.69"
    },
    {
        "storeNumber": "75393-105057",
        "storeName": "College St. Rose",
        "streetAddress": "420 Western Ave, Hilton Garden Inn at Albany Medical Ctr",
        "city": "Albany",
        "state": "NY",
        "country": "US",
        "postcode": "122031400",
        "longitude": "-73.79",
        "latitude": "42.66"
    },
    ...
]
```

### Find all stores in a city

```curl
curl http://127.0.0.1:9000/search?country=US&state=NY&city=ALBANY
```

### Find all stores in a postcode

```curl
curl http://127.0.0.1:9000/search?country=US&state=NY&city=ALBANY&postcode=12203
```
