use std::fmt::Display;
use enum_display::EnumDisplay;

use std::{
    env::consts
};

use gethostname::gethostname;

#[derive(EnumDisplay)]
pub enum Platform {
    Windows,
    Linux
}
pub struct Device {
    pub platform: Platform,
    pub hostname: String
} // We're gonna put in locale, region etc all in here later

impl Device {
    pub fn new() -> Self {
        let mut platform =
            if consts::OS == "windows" {
                Platform::Windows
            } else {
                Platform::Linux
            };
        

        Device { 
            platform: platform,
            hostname:  gethostname().to_string_lossy().into_owned() 
        }
    }
}