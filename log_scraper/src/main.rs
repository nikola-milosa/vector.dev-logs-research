use log::error;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Value::Array};
use std::{collections::HashMap, env, fs, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config_path = env::var("CONFIG_PATH").unwrap_or("config.json".to_string());
    let default_port = env::var("API_PORT").unwrap_or("5000".to_string());
    let scrape_interval = env::var("SCRAPE_INTERVAL").unwrap_or("2".to_string());
    let vector_url = env::var("VECTOR_INSTANCE_IP");

    let mut cursors: HashMap<String, String> = HashMap::new();

    loop {
        let config_file = fs::read_to_string(&config_path).unwrap();
        let config_info: Vec<Config> = serde_json::from_str(&config_file).unwrap();

        let mut current_scrape: Vec<String> = Vec::new();

        for config in config_info {
            for node_ip in config.node_ips {
                let node_cursor = cursors.get(&node_ip);

                let url: String;
                if let Some(cursor) = node_cursor {
                    url = format!("http://{}:{}/logs/{}", node_ip, default_port, cursor);
                } else {
                    url = format!("http://{}:{}/logs", node_ip, default_port);
                }

                let resp = reqwest::get(url).await?.text().await?;

                let response_as_json: Value = serde_json::from_str(&resp).unwrap();
                match response_as_json {
                    Array(array) => {
                        let last_entry = &array.get(array.len() - 1).unwrap()["__cursor"];
                        if let Value::String(str) = last_entry {
                            cursors.insert(node_ip.to_string(), str.to_string());
                        }

                        for log in array {
                            let mut new_entry = log.clone();
                            new_entry["node_ip"] = Value::String(node_ip.to_string());
                            new_entry["dc"] = Value::String(config.dc.to_string());
                            new_entry["subnet"] = Value::String(config.subnet.to_string());
                            current_scrape.push(serde_json::to_string(&new_entry).unwrap());
                        }
                    }
                    _ => (),
                }
            }
        }

        if let Ok(vec_url) = &vector_url {
            let client = reqwest::Client::new();
            let response = client
                .post(vec_url)
                .header("CONTENT_TYPE", "application/json")
                .json(&current_scrape)
                .send()
                .await?;
            if response.status() != 200 {
                error!("Error sending to vector: {:?}", response.text().await?);
            }
        } else {
            let current_scrape_as_json = serde_json::to_string(&current_scrape).unwrap();
            println!("{}", current_scrape_as_json);
        }

        thread::sleep(Duration::from_secs(scrape_interval.parse::<u64>().unwrap()));
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    node_ips: Vec<String>,
    dc: String,
    subnet: String,
}
