use log::{error, info};
use serde_json::{Value, Value::Array};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{config::Config, ic_node::IcNode};

pub async fn start_scrape(
    global_config: &Config,
    mut cursors: HashMap<String, String>,
    ic_nodes: Arc<Mutex<Vec<IcNode>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut current_scrape: Vec<String> = Vec::new();
        {
            for ic_node in ic_nodes.lock().unwrap().iter() {
                let url = format_url(cursors.get(&ic_node.ip), &ic_node.ip, &global_config);
                let response = match reqwest::get(url).await {
                    Ok(response) => response,
                    Err(e) => {
                        error!(
                            "Error while getting response from node {}: {}",
                            ic_node.ip, e
                        );
                        continue;
                    }
                };
                let resp = response.text().await?;

                handle_response(&resp, &mut cursors, &ic_node, &mut current_scrape)
            }

            if let Ok(vec_url) = &global_config.vector_url {
                post_to_vector(vec_url, &current_scrape).await?;
            } else {
                let current_scrape_as_json = serde_json::to_string(&current_scrape).unwrap();
                info!("{}", current_scrape_as_json);
            }
        }

        thread::sleep(Duration::from_secs(
            global_config.scrape_interval.parse::<u64>().unwrap(),
        ));
    }
}

async fn post_to_vector(
    vec_url: &String,
    current_scrape: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(vec_url)
        .header("CONTENT_TYPE", "application/json")
        .json(&current_scrape)
        .send()
        .await;
    let response = match response {
        Ok(response) => response,
        Err(e) => {
            error!("Error while posting to vector: {}", e);
            return Err(Box::new(e));
        }
    };
    match response.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error while posting to vector: {}", e);
            Err(Box::new(e))
        }
    }
}

fn handle_response(
    resp: &String,
    cursors: &mut HashMap<String, String>,
    ic_node: &IcNode,
    current_scrape: &mut Vec<String>,
) {
    let response_as_json: Value = serde_json::from_str(&resp).unwrap();
    match response_as_json {
        Array(array) => {
            let last_entry = &array.get(array.len() - 1).unwrap()["__cursor"];
            if let Value::String(str) = last_entry {
                cursors.insert(ic_node.ip.to_string(), str.to_string());
            }

            for log in array {
                let mut new_entry = log.clone();
                new_entry["ic_node"] = Value::String(ic_node.ic_node.to_string());
                new_entry["dc"] = Value::String(ic_node.dc.to_string());
                new_entry["ic_subnet"] = Value::String(ic_node.ic_subnet.to_string());
                new_entry["ip"] = Value::String(ic_node.ip.to_string());
                current_scrape.push(serde_json::to_string(&new_entry).unwrap());
            }
        }
        _ => (),
    }
}

fn format_url(node_cursor: Option<&String>, ip: &String, config: &Config) -> String {
    let url = match config.use_ipv6 {
        true => format!(
            "http://[{}]:{}/{}",
            ip, config.journal_port, config.log_path
        ),
        false => format!("http://{}:{}{}", ip, config.journal_port, config.log_path),
    };
    match node_cursor {
        Some(cursor) => format!("{}?cursor={}", url, cursor),
        None => url,
    }
}
