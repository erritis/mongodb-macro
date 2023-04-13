[![crates.io][crates-badge]][crates-url]
[![documentation][docs-badge]][docs-url]
[![MIT License][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/mongodb-macro.svg
[crates-url]: https://crates.io/crates/mongodb-macro
[docs-badge]: https://docs.rs/mongodb-macro/badge.svg
[docs-url]: https://docs.rs/mongodb-macro
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

mongodb-macro

MongoDB Macro is a crate with macros for quickly creating structures to work with mongodb

## Examples
```rust

use mongodb::{Client, bson::Bson};
use mongodb_macro::Parser;

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

Current version: 0.1.0

License: MIT
