/*!
MongoDB Macro is a crate with macros for quickly creating structures to work with mongodb

# Examples
```no_run

use mongodb::{Client, bson::Bson};
use mongodb_macro::Parser;

mongodb_macro::config!(Opts);
//equal
//mongodb_macro::config!(Opts; "DB_NAME", "COLLECTION_NAME","DB_URL");

//or
//mongodb_macro::config!(Opts, "MONGO");
//equal
//mongodb_macro::config!(Opts; "MONGO_DB_NAME", "MONGO_COLLECTION_NAME","MONGO_DB_URL");

async fn main() -> std::io::Result<()> {

    let opts = Opts::parse();

    let client = mongodb::Client::with_uri_str(opts.db_url).await.expect("failed to connect");
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

#[doc(hidden)]
pub use clap::Parser;