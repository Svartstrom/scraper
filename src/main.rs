use crate::scrape::*;
use std::path::Path;

pub mod scrape;

fn main() {
    let config_path = Path::new("./src/config.json");
    let cfg = {
        let inner_cfg = std::fs::read_to_string(&config_path).unwrap();
        serde_json::from_str::<CONFIG>(&inner_cfg).unwrap()
    };
    
    scrape_placera(&cfg);
}