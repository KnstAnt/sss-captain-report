
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{io, thread, time};

use crate::error::Error;

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
pub struct Message {
    pub path: String,
    pub name: String,
}
//
pub fn get_args() -> Result<Message, Error> {
    let stdin_channel = spawn_stdin_channel();
    for _ in 0..50 {
        match stdin_channel.try_recv() {
            Ok(input) => {
                log::info!("read from stdin: {input}");
                let message = serde_json::from_str(&input)?;
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
        path: "bin/html".to_owned(),
        name: "report".to_owned(),
    };
    Ok(message)
}
