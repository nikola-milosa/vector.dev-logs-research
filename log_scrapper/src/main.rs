mod config;
mod config_watcher;
mod ic_node;
mod scrapper;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use log::error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let global_config = Arc::new(config::new_config());
    let global_config_for_watcher = Arc::clone(&global_config);

    let config = match config_watcher::read_config(&global_config) {
        Ok(config) => config,
        Err(e) => {
            error!("Error reading config: {:?}", e);
            return Err(e);
        }
    };

    let ic_nodes = Arc::new(Mutex::new(config));
    let ic_nodes_for_watcher = Arc::clone(&ic_nodes);
    let cursors: HashMap<String, String> = HashMap::new();

    thread::spawn(move || {
        config_watcher::watch_file(global_config_for_watcher, ic_nodes_for_watcher);
    });

    scrapper::start_scrape(&global_config, cursors, ic_nodes).await
}
