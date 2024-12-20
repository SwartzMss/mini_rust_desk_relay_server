use clap::App;
mod relay_server;
use flexi_logger::*;
use mini_rust_desk_common::ResultType;
use relay_server::*;

fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("debug")?
        .log_to_stdout()
        .format(opt_format)
        .write_mode(WriteMode::Async)
        .start()?;
    let args = format!(
        "-p, --port=[NUMBER(default=21117)] 'Sets the listening port'
        -k, --key=[KEY] 'Only allow the client with the same key'
        ",
    );
    let matches = App::new("mini_rust_desk_relay_server")
        .args_from_usage(&args)
        .get_matches();
    if let Ok(v) = ini::Ini::load_from_file(".env") {
        if let Some(section) = v.section(None::<String>) {
            section.iter().for_each(|(k, v)| std::env::set_var(k, v));
        }
    }
    let mut port = 21117;
    if let Ok(v) = std::env::var("PORT") {
        let v: i32 = v.parse().unwrap_or_default();
        if v > 0 {
            port = v + 1;
        }
    }
    start(
        matches.value_of("port").unwrap_or(&port.to_string()),
        matches
            .value_of("key")
            .unwrap_or(&std::env::var("KEY").unwrap_or_default()),
    )?;
    Ok(())
}
