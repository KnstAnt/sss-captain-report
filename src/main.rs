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
    let mut report = Report::new(
        message.params.language.clone(),    
        ApiServer::new(
            "sss-computing".to_owned(),
            message.address.host.to_owned(),
            message.address.port.to_string(),
            message.params.ship_id, 
            message.params.project_id,
            message.params.language.clone(),      
        )
    );
    if let Err(error) = report.get_from_db() {
        return Err(Error::FromString(format!("Execute report.get_from_db error: {}", error)));
    }
    if let Err(error) = report.write(&message.params.path, &message.params.name) {
        return Err(Error::FromString(format!("Execute report.write error: {}", error)));
    }
    Ok(())    
}
