use log::{debug, error, info, trace, warn};
use notify::{watcher, DebouncedEvent::NoticeWrite, RecursiveMode, Watcher};
use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    env_logger::init();
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    let watch_path = env::var("watch_path").unwrap();
    info!("Watching path: {}", watch_path);

    let host_url = env::var("docker_host_url").unwrap();
    info!("Host url: {}", host_url);

    let vector_container = env::var("vector_container").unwrap();
    info!("Vector container: {}", vector_container);

    watcher.watch(watch_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                NoticeWrite(_) => {
                    println!("It was a notice write");
                    let client = reqwest::blocking::Client::new();

                    let mut response = client
                        .post(format!(
                            "{}/containers/{}/kill?signal=SIGHUP",
                            host_url, vector_container
                        ))
                        .send();
                    match response {
                        Ok(_) => {
                            info!("Initiated reload of vector-agent")
                        }
                        Err(err) => {
                            error!("Error occured, {}", err)
                        }
                    }
                }
                _ => (),
            },
            Err(e) => println!("Watch error : {:?}", e),
        }
    }
}
