use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub google_access_token: Option<String>,
    pub google_refresh_token: Option<String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            google_access_token: None,
            google_refresh_token: None,
        }
    }
}
