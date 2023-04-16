/// Quick and easy creates a new configuration to connect to MongoDB
/// 
/// Creating a configuration structure when using one database in a project:
/// 
/// ```
/// mongodb_macro::config!(Opts);
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
/// Creating a configuration structure when using multiple databases in a project:
/// 
/// This sets the prefix to the environment variables.
/// 
/// ```
/// mongodb_macro::config!(Opts, "MONGO");
///
/// fn main() {
///     std::env::set_var("MONGO_DB_URL", "mongodb://root:root@localhost:27017");
///     std::env::set_var("MONGO_DB_NAME", "test");
///     std::env::set_var("MONGO_COLLECTION_NAME", "users");
/// 
///     let opts = Opts::parse();
/// }
/// ```
/// 
/// Creating a configuration structure with explicit fields:
/// 
/// ```
/// mongodb_macro::config!(Opts; "MONGO_DB_URL", "MONGO_DB_NAME", "MONGO_COLLECTION_NAME");
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
macro_rules! config {
    ($opts:ident, $prefix:tt) => ($crate::config!{$opts, $prefix; ("DB_URL", "DB_NAME", "COLLECTION_NAME")});

    ($opts:ident, $prefix:tt; ($db_url:tt, $db_name:tt, $collection_name:tt)) => {

        #[derive(Clone, Debug, PartialEq, Eq, ::clap::Parser)]
        pub struct $opts {

            /// env by default DB_URL
            #[clap(env = concat!($prefix, "_", $db_url))]
            pub db_url: String,

            /// env by default DB_NAME
            #[clap(env = concat!($prefix, "_", $db_name))]
            pub db_name: String,

            /// env by default COLLECTION_NAME
            #[clap(env = concat!($prefix, "_", $collection_name))]
            pub collection_name: String,
        }
    };
    
    ($opts:ident) => ($crate::config!{$opts; "DB_URL", "DB_NAME", "COLLECTION_NAME"});

    ($opts:ident; $db_url:tt, $db_name:tt, $collection_name:tt) => {

        #[derive(Clone, Debug, PartialEq, Eq, ::clap::Parser)]
        pub struct $opts {

            /// env by default DB_URL
            #[clap(env = $db_url)]
            pub db_url: String,

            /// env by default DB_NAME
            #[clap(env = $db_name)]
            pub db_name: String,

            /// env by default COLLECTION_NAME
            #[clap(env = $collection_name)]
            pub collection_name: String,
        }
    };
}