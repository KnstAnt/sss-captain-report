use args::get_args;
use debugging::session::debug_session::{DebugSession, LogLevel};
use log::info;
use parser::Report;
use sal_core::{dbg::Dbg, error::Error};

use crate::db::api::ApiClient;

mod args;
mod content;
mod converter;
mod db;
mod parser;

fn main() {
    DebugSession::new()
        .filter(LogLevel::Trace)
        .module("api_tools", LogLevel::Error)
        .init();
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
    let error = Error::new("Main", "execute");
    let message = get_args().map_err(|err| error.pass(err))?;
    let dbg = Dbg::own("main");
    let mut report = Report::new(
        &dbg,
        message.params.ship_id.clone(),
        message.params.project_id.clone(),
        message.params.language.unwrap_or("ru".to_owned()).clone(),
        ApiClient::new(
            &dbg,
            message.address.database.clone(),
            message.address.host.clone(),
            message.address.port.to_string().clone(),
        ),
    );
    if let Err(err) = report.get_from_db() {
        return Err(error.pass(err));
    }
    if let Err(err) = report.write(&message.params.path, &message.params.name) {
        return Err(error.pass(err));
    }
    Ok(())
}
