

/// Creates a new configuration structure to initialize the MongoDB collection
/// 
/// Create a new configuration structure to initialize the MongoDB collection with a standard environment variable
/// 
/// ```
/// mongodb_macro::collection_config!(Opts);
///
/// fn main() {
///     std::env::set_var("DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("DB_NAME", "test");
///     std::env::set_var("COLLECTION_NAME", "users");
/// 
///     let opts = Opts::parse();
/// }
/// ```
/// 
/// Create a new configuration structure to initialize the MongoDB collection with the specified environment variable
/// 
/// ```
/// mongodb_macro::collection_config!(Opts; ("MONGO_DB_URL", "MONGO_DB_NAME", "MONGO_COLLECTION_NAME"));
///
/// fn main() {
///     std::env::set_var("MONGO_DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("MONGO_DB_NAME", "test");
///     std::env::set_var("MONGO_COLLECTION_NAME", "users");
/// 
///     let opts = Opts::parse();
/// }
/// ```
#[macro_export]
macro_rules! collection_config {
    ($opts:ident) => ($crate::config!{$opts});

    ($opts:ident; ($db_url:tt, $db_name:tt, $collection_name:tt)) => 
    ($crate::config!{$opts; $db_url, $db_name, $collection_name});
}

/// Creates a new factory to create a MongoDB collection
/// 
/// Create mongodb collection factory with standard environment variable for database url, database name and collection name
/// 
/// ```
/// mongodb_macro::collection!(CollectionFactory; CollectionFactoryOpts);
///
/// fn main() {
///     std::env::set_var("DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("DB_NAME", "test");
///     std::env::set_var("COLLECTION_NAME", "users");
/// 
///     let factory = CollectionFactory::parse();
/// 
///     // let collection = factory.create<Bson>().await.expect("failed to connect");
/// }
/// ```
/// 
/// Create mongodb collection factory with specified environment variable for database url, database name and collection name
/// 
/// ```
/// mongodb_macro::collection!(CollectionFactory; CollectionFactoryOpts; ("MONGO_DB_URL", "MONGO_DB_NAME", "MONGO_COLLECTION_NAME"));
///
/// fn main() {
///     std::env::set_var("MONGO_DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("MONGO_DB_NAME", "test");
///     std::env::set_var("MONGO_COLLECTION_NAME", "users");
/// 
///     let factory = CollectionFactory::parse();
///
///     // let collection = factory.create<Bson>().await.expect("failed to connect");
/// }
/// ```
/// 
/// Create a mongodb collection factory with nested environment variables into standard environment variables
/// 
/// ```
/// mongodb_macro::collection!(CollectionFactory; CollectionFactoryOpts);
///
/// fn main() {
///     std::env::set_var("MONGODB_HOST", "localhost");
///     std::env::set_var("DB_URL", "mongodb://root:root@${MONGODB_HOST}:27017");
///     std::env::set_var("DB_NAME", "test");
///     std::env::set_var("COLLECTION_NAME", "users");
/// 
///     let factory = CollectionFactory::parse();
/// 
///     // let collection = factory.create<Bson>().await.expect("failed to connect");
/// 
///     assert_eq!(factory.config().db_url, "mongodb://root:root@localhost:27017");
/// }
/// ```
#[macro_export]
macro_rules! collection {
    ($collection_factory:ident; $opts:ident) => ($crate::collection!{$collection_factory; $opts; ("DB_URL", "DB_NAME", "COLLECTION_NAME")});

    ($collection_factory:ident; $opts:ident; ($db_url:tt, $db_name:tt, $collection_name:tt)) => {

        $crate::collection_config!($opts; ($db_url, $db_name, $collection_name));

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $collection_factory($opts);

        impl $collection_factory {
            fn parse() -> Self {
                let mut opts = $opts::parse();

                opts.db_url = $crate::env_expand(&opts.db_url);
                opts.db_name = $crate::env_expand(&opts.db_name);
                opts.collection_name = $crate::env_expand(&opts.collection_name);

                Self(opts)
            }

            pub fn config(&self) -> &$opts {
                &self.0
            }

            pub async fn create<T>(&self) -> Result<::mongodb::Collection<T>, ::mongodb::error::Error> {
                let client = ::mongodb::Client::with_uri_str(&self.0.db_url).await?;
                let db = client.database(&self.0.db_name);
                Ok(db.collection::<T>(&self.0.collection_name))
            }
        }
    };
}