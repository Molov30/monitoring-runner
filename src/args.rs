use std::path::PathBuf;

use clap::{
    Parser,
    builder::{Styles, styling::AnsiColor},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None,
    styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Cyan.on_default()))]
pub struct Args {
    #[arg(long, default_value_t = 20109)]
    pub grpc_port: u64,

    #[arg(long, default_value_t = 20400)]
    pub prometheus_port: u64,

    #[arg(long, default_value = default_tickets_dir(), value_parser = existing_dir)]
    pub ticket_dir: PathBuf,

    #[arg(long, default_value = ".", value_parser = existing_dir)]
    pub db_dir: PathBuf,

    #[arg( long, value_parser = existing_file)]
    pub lm_bin_path: PathBuf,

    #[arg(long, default_value = "localhost")]
    pub lm_host: String,

    #[arg(long, default_value_t = 9999)]
    pub lm_port: u64,

    #[arg(long)]
    pub token: Option<String>,
}

fn existing_dir(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    match p.exists() && p.is_dir() {
        true => Ok(p),
        false => Err(format!("path does not exist or not dir: {}", s)),
    }
}

fn existing_file(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    match p.exists() && p.is_file() {
        true => Ok(p),
        false => Err(format!("path does not exist or not file: {}", s)),
    }
}

#[cfg(target_os = "windows")]
fn default_tickets_dir() -> &'static str {
    r"C:\ProgramData\AxxonSoft\AxxonNext\Tickets"
}

#[cfg(not(target_os = "windows"))]
fn default_tickets_dir() -> &'static str {
    "." // FIXME
}
