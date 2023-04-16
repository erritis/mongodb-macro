use envmnt::{ExpandOptions, ExpansionType};

//FIXME:This method is a temporary solution. Also, while it should be built into the config macro 
///Replaces nested environment variables in a string
pub fn env_expand(env_str: &str) -> String {
    let mut options = ExpandOptions::new();
    options.expansion_type = Some(ExpansionType::Unix);
    envmnt::expand(env_str, Some(options))
}