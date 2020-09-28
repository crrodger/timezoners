use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    name: String,
}

//  If the content of this structure changes then delete config file from ~/Library/Preferences/<app-name> toml file
impl Default for Config {
    fn default() -> Self { 
        Self { 
            name: String::from("gtk-base"),
        }
    }
}