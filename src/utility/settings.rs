use config::Config;
use serde::{Deserialize};

/// Settings for the render
#[derive(Deserialize)]
pub struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub file_type: String,
}

impl Settings {
    /// Creates a new 'Settings' instance from a 'settings.toml' in the top level directory
    pub fn new() -> Self {
        let config = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();

        config.try_deserialize::<Settings>().unwrap()
    }
}