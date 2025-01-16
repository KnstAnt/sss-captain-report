use db::api_server::ApiServer;
use error::Error;
use log::info;
use parser::Report;
use args::get_args;

mod args;
mod content;
mod db;
mod error;
mod formatter;
mod parser;
mod converter;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");
    let reply = if let Err(error) = execute() {
        let str1 = r#"{"status":"failed","message":""#;
        let str2 = r#""}"#;
        format!("{str1}{}{str2}", error)
    } else {
        r#"{"status":"ok","message":null}"#.to_owned()
    };
    info!("reply: {reply}");
    let _ = std::io::Write::write_all(&mut std::io::stdout().lock(), reply.as_bytes());
}
#[allow(unused)]
fn execute() -> Result<(), Error> {
    let message = get_args()?;
    log::info!(
        "args path:{} name:{}",
        message.path,
        message.name
    );
    let mut report = Report::new(2, ApiServer::new("sss-computing".to_owned()));
    if let Err(error) = report.get_from_db() {
        return Err(Error::FromString(format!("Execute report.get_from_db error: {}", error)));
    }
    if let Err(error) = report.write(&message.path, &message.name) {
        return Err(Error::FromString(format!("Execute report.write error: {}", error)));
    }
    Ok(())    
}
