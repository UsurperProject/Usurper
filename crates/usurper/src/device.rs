use std::{
    env::consts
};

use gethostname::gethostname;

pub enum Platform {
    Windows,
    Linux
}
pub struct device {
    platform: Platform,
    hostname: String
} // We're gonna put in locale, region etc all in here later

impl device {
    pub fn new() -> Self {
        let mut platform =
            if consts::OS == "windows" {
                Platform::Windows
            } else {
                Platform::Linux
            };
        

        device { 
            platform: platform,
            hostname:  gethostname().to_string_lossy().into_owned() 
        }
    }
}