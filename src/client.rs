

/// Creates a new configuration structure to initialize the MongoDB client
/// 
/// Create a new configuration structure to initialize the MongoDB client with a standard environment variable
/// 
/// ```
/// use mongodb_macro::Parser;
/// mongodb_macro::client_config!(Opts);
///
/// fn main() {
///     std::env::set_var("MONGODB_HOST", "localhost");
///     std::env::set_var("DB_URL", "mongodb://root:root@${MONGODB_HOST}:27017");
/// 
///     let opts = Opts::parse();
/// 
///     assert_eq!(&opts.db_url, "mongodb://root:root@localhost:27017");
/// }
/// ```
/// 
/// Create a new configuration structure to initialize the MongoDB client with the specified environment variable
/// 
/// ```
/// use mongodb_macro::Parser;
/// mongodb_macro::client_config!(Opts; "MONGO_DB_URL");
///
/// fn main() {
///     std::env::set_var("MONGODB_HOST", "localhost");
///     std::env::set_var("MONGO_DB_URL", "mongodb://root:root@${MONGODB_HOST}:27017");
/// 
///     let opts = Opts::parse();
/// 
///     assert_eq!(&opts.db_url, "mongodb://root:root@localhost:27017");
/// }
/// ```
#[macro_export]
macro_rules! client_config {
    ($opts:ident) => ($crate::client_config!{$opts; "DB_URL"});

    ($opts:ident; $db_url:tt) => {

        #[derive(Clone, Debug, PartialEq, Eq, $crate::Parser)]
        pub struct $opts {
            /// env by default DB_URL
            #[clap(env = $db_url)]
            pub db_url: $crate::Env,
        }
    };
}

/// Creates a new factory to create a MongoDB client
/// 
/// Create mongodb client factory with standard environment variable for db url
/// 
/// ```
/// mongodb_macro::client!(ClientFactory; ClientFactoryOpts);
///
/// fn main() {
///     std::env::set_var("MONGODB_HOST", "localhost");
///     std::env::set_var("DB_URL", "mongodb://root:root@${MONGODB_HOST}:27017");
/// 
///     let factory = ClientFactory::parse();
/// 
///     // let client = factory.create().await.expect("failed to connect");
/// 
///     assert_eq!(&factory.config().db_url, "mongodb://root:root@localhost:27017");
/// }
/// ```
/// 
/// Create mongodb client factory with specified environment variable for db url
/// 
/// ```
/// mongodb_macro::client!(ClientFactory; ClientFactoryOpts; "MONGO_DB_URL");
///
/// fn main() {
///     std::env::set_var("MONGODB_HOST", "localhost");
///     std::env::set_var("MONGO_DB_URL", "mongodb://root:root@${MONGODB_HOST}:27017");
/// 
///     let factory = ClientFactory::parse();
///
///     // let client = factory.create().await.expect("failed to connect");
/// 
///     assert_eq!(&factory.config().db_url, "mongodb://root:root@localhost:27017");
/// }
/// ```
#[macro_export]
macro_rules! client {
    ($client_factory:ident; $opts:ident) => ($crate::client!{$client_factory; $opts; "DB_URL"});
    ($client_factory:ident; $opts:ident; $db_url:tt) => {

        $crate::client_config!($opts; $db_url);

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $client_factory($opts);

        impl $client_factory {
            fn parse() -> Self {
                use $crate::Parser;

                let opts = $opts::parse();

                Self(opts)
            }

            pub fn config(&self) -> &$opts {
                &self.0
            }

            pub async fn create(&self) -> Result<::mongodb::Client, ::mongodb::error::Error> {
                ::mongodb::Client::with_uri_str(&self.0.db_url).await
            }
        }
    };
}