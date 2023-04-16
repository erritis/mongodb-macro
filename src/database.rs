

/// Creates a new configuration structure to initialize the MongoDB database
/// 
/// Create a new configuration structure to initialize the MongoDB database with a standard environment variable
/// 
/// ```
/// mongodb_macro::database_config!(Opts);
///
/// fn main() {
///     std::env::set_var("DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("DB_NAME", "test");
/// 
///     let opts = Opts::parse();
/// }
/// ```
/// 
/// Create a new configuration structure to initialize the MongoDB database with the specified environment variable
/// 
/// ```
/// mongodb_macro::database_config!(Opts; ("MONGO_DB_URL", "MONGO_DB_NAME"));
///
/// fn main() {
///     std::env::set_var("MONGO_DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("MONGO_DB_NAME", "test");
/// 
///     let opts = Opts::parse();
/// }
/// ```
#[macro_export]
macro_rules! database_config {
    ($opts:ident) => ($crate::database_config!{$opts; ("DB_URL", "DB_NAME")});

    ($opts:ident; ($db_url:tt, $db_name:tt)) => {

        #[derive(Clone, Debug, PartialEq, Eq, ::clap::Parser)]
        pub struct $opts {
            /// env by default DB_URL
            #[clap(env = $db_url)]
            pub db_url: String,

            /// env by default DB_NAME
            #[clap(env = $db_name)]
            pub db_name: String,
        }
    };
}

/// Creates a new factory to create a MongoDB database
/// 
/// Create mongodb database factory with standard environment variable for database url and database name
/// 
/// ```
/// mongodb_macro::database!(DbFactory; DbFactoryOpts);
///
/// fn main() {
///     std::env::set_var("DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("DB_NAME", "test");
/// 
///     let factory = DbFactory::parse();
/// 
///     // let db = factory.create().await.expect("failed to connect");
/// }
/// ```
/// 
/// Create mongodb database factory with specified environment variable for database url and database name
/// 
/// ```
/// mongodb_macro::database!(DbFactory; DbFactoryOpts; ("MONGO_DB_URL", "MONGO_DB_NAME"));
///
/// fn main() {
///     std::env::set_var("MONGO_DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("MONGO_DB_NAME", "test");
/// 
///     let factory = DbFactory::parse();
///
///     // let db = factory.create().await.expect("failed to connect");
/// }
/// ```
#[macro_export]
macro_rules! database {
    ($db_factory:ident; $opts:ident) => ($crate::database!{$db_factory; $opts; ("DB_URL", "DB_NAME")});

    ($db_factory:ident; $opts:ident; ($db_url:tt, $db_name:tt)) => {

        $crate::database_config!($opts; ($db_url, $db_name));

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $db_factory($opts);

        impl $db_factory {
            fn parse() -> Self {
                let opts = $opts::parse();

                opts.db_url = $crate::env_expand(&opts.db_url);
                opts.db_name = $crate::env_expand(&opts.db_name);

                Self(opts)
            }

            pub fn config(&self) -> &$opts {
                &self.0
            }

            pub async fn create(&self) -> Result<::mongodb::Database, ::mongodb::error::Error> {
                let client = ::mongodb::Client::with_uri_str(&self.0.db_url).await?;
                Ok(client.database(&self.0.db_name))
            }
        }
    };
}