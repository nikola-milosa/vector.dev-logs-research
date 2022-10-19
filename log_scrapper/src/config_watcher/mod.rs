use log::{error, info};
use notify::{watcher, DebouncedEvent::Write, RecursiveMode, Watcher};
use std::{
    error::Error,
    fs,
    sync::{mpsc::channel, Arc, Mutex},
    time::Duration,
};

use crate::{config::Config, ic_node::IcNode};

pub fn read_config(config: &Config) -> Result<Vec<IcNode>, Box<dyn Error>> {
    info!("Reading config file {}", config.config_path);
    let config_file = match fs::read_to_string(&config.config_path) {
        Ok(config_file) => config_file,
        Err(e) => {
            error!("Error reading config file: {}", e);
            error!(
                "Current config file: {}/{}",
                std::env::current_dir().unwrap().display(),
                &config.config_path
            );
            std::process::exit(1);
        }
    };
    let ic_nodes: Vec<IcNode> = match serde_json::from_str(&config_file) {
        Ok(ic_nodes) => ic_nodes,
        Err(e) => {
            error!("Error parsing config file: {}", e);
            error!(
                "Current config file: {}/{}",
                std::env::current_dir().unwrap().display(),
                &config.config_path
            );
            return Err(Box::new(e));
        }
    };
    Ok(ic_nodes)
}

pub fn watch_file(config: Arc<Config>, ic_nodes: Arc<Mutex<Vec<IcNode>>>) {
    let (tx, rx) = channel();

    let mut watcher = match watcher(tx, Duration::from_secs(1)) {
        Ok(watcher) => watcher,
        Err(e) => {
            error!("Error creating watcher: {}", e);
            std::process::exit(1);
        }
    };

    info!("Config path: {}", config.config_path);

    match watcher.watch(&config.config_path, RecursiveMode::Recursive) {
        Ok(_) => info!("Watching config file"),
        Err(e) => error!("Error watching config file: {}", e),
    }

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Write(_) => {
                    info!("Config file changed, Reloading . . .");
                    let mut ref_to_nodes = match ic_nodes.lock() {
                        Ok(nodes) => nodes,
                        Err(e) => {
                            error!("Error getting lock on ic_nodes: {}", e);
                            continue;
                        }
                    };
                    match read_config(&config) {
                        Ok(nodes) => {
                            *ref_to_nodes = nodes;
                            info!("Config reloaded");
                        }
                        Err(e) => error!("Error reading config: {}", e),
                    }
                }
                _ => (),
            },
            Err(e) => error!("Watch error : {:?}", e),
        }
    }
}
