[![crates.io][crates-badge]][crates-url]
[![documentation][docs-badge]][docs-url]
[![MIT License][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/mongodb-macro.svg
[crates-url]: https://crates.io/crates/mongodb-macro
[docs-badge]: https://docs.rs/mongodb-macro/badge.svg
[docs-url]: https://docs.rs/mongodb-macro
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

# mongodb-macro

MongoDB Macro is a crate with macros for quickly creating structures when working with mongodb.
In particular, macros are implemented for quick initialization of structures of the "factory" pattern.
The purpose of this crate is to reduce the amount of boilerplate code when initializing standard structures.

## installation

Install using cargo:

`cargo install mongodb-macro`

Make sure you also add to the project:

> mongodb = "*"
>
> clap = { version = "*", features = ["derive", "env"] }

## Usage
### Macro: Collection
```rust

use mongodb::bson::Bson;

// env DB_URL should contain a link to the mongodb url
// env DB_NAME should contain the database name
// env COLLECTION_NAME should contain the collection name
mongodb_macro::collection!(CollectionFactory; CollectionFactoryOpts);
// or with a specified env
// mongodb_macro::collection!(CollectionFactory; CollectionFactoryOpts; ("MONGO_DB_URL", "MONGO_DB_NAME", "MONGO_COLLECTION_NAME"));

async fn main() -> std::io::Result<()> {

    let factory = CollectionFactory::parse();

    let collection = factory.create::<Bson>().await.expect("failed to connect");

    ...
}
```

### Macro: Database
```rust

use mongodb::bson::Bson;

// env DB_URL should contain a link to the mongodb url
// env DB_NAME should contain the database name
mongodb_macro::database!(DbFactory; DbFactoryOpts);
// or with a specified env
// mongodb_macro::database!(DbFactory; DbFactoryOpts; ("MONGO_DB_URL", "MONGO_DB_NAME"));

async fn main() -> std::io::Result<()> {

    let factory = DbFactory::parse();

    let db = factory.create().await.expect("failed to connect");

    let collection = db.collection::<Bson>("users");

    ...
}
```

### Macro: Client
```rust

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

### Macro: Config
```rust

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

Current version: 1.0.1

License: MIT
