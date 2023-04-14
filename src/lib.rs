/*!
MongoDB Macro is a crate with macros for quickly creating structures when working with mongodb.
In particular, macros are implemented for quick initialization of structures of the "factory" pattern.
The purpose of this crate is to reduce the amount of boilerplate code when initializing standard structures.

# installation

Install using cargo:

`cargo install mongodb-macro`

Make sure you also add to the project:

> mongodb = "*"
>
> clap = { version = "*", features = ["derive", "env"] }

# Usage

## Macro: Client
```no_run

use mongodb::bson::Bson;

// env DB_URL should contain a link to the mongodb url
mongodb_macro::client!(ClientFactory; ClientFactoryOpts);
// or with a specified env
// mongodb_macro::client!(ClientFactory; ClientFactoryOpts; "MONGO_DB_URL");

async fn main() -> std::io::Result<()> {

    let factory = ClientFactory::parse();

    let client = factory.create().await.expect("failed to connect");

    let db = client.database("demo");
    let collection = db.collection::<Bson>("users");
    
    ...
}
```

## Macro: Config
```no_run

use mongodb::bson::Bson;

mongodb_macro::config!(Opts);
// equivalent to
// mongodb_macro::config!(Opts; "DB_NAME", "COLLECTION_NAME", "DB_URL");
//
// or with prefix
//
// mongodb_macro::config!(Opts, "MONGO");
// equivalent to
// mongodb_macro::config!(Opts; "MONGO_DB_NAME", "MONGO_COLLECTION_NAME", "MONGO_DB_URL");

async fn main() -> std::io::Result<()> {

    let opts = Opts::parse();

    let client = mongodb::Client::with_uri_str(&opts.db_url).await.expect("failed to connect");
    let db = client.database(&opts.db_name);
    let collection = db.collection::<Bson>(&opts.collection_name);
    
    ...
}
```
*/
#![warn(missing_docs)]

/// Macros for creating MongoDB configurations from environment variables
#[doc(hidden)]
pub mod opts;

/// Macros for creating MongoDB client configurations and factories from environment variables
#[doc(hidden)]
pub mod client;