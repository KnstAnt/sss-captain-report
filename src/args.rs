use sal_core::error::Error;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{io, thread, time};

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiAddress {
    pub host: String,
    pub port: i32,
    pub database: String,
}
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Params {
    pub path: String,
    pub name: String,
    pub language: Option<String>, // "ru" - russian (default) / "en" - english
    #[serde(alias = "ship-id")]
    pub ship_id: String,
    #[serde(alias = "project-id")]
    pub project_id: String,
}
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    #[serde(alias = "api-address")]
    pub address: ApiAddress,
    pub params: Params,
}
//
pub fn get_args() -> Result<Message, Error> {
    let stdin_channel = spawn_stdin_channel();
    for _ in 0..50 {
        match stdin_channel.try_recv() {
            Ok(input) => {
                log::info!("read from stdin: {input}");
                let message = serde_json::from_str(&input)
                    .map_err(|err| Error::new("Message", "get_args").pass(err.to_string()))?;
                log::info!("io::stdin(): {:?}", message);
                return Ok(message);
            }
            Err(_) => {
                thread::sleep(time::Duration::from_millis(100));
                continue;
            }
        }
    }
    log::error!("error read from stdin!");
    let message = Message {
        address: ApiAddress {
            host: "0.0.0.0".to_owned(),
            port: 8081,
            database: "sss-computing".to_owned(),
        },
        params: Params {
            path: "bin/html".to_owned(),
            name: "report".to_owned(),
            language: Some("en".to_owned()),
            ship_id: "2".to_owned(),
            project_id: "NULL".to_owned(),
        },
    };
    log::info!("set default message:{:?}", message);
    Ok(message)
}
