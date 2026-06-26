use clap::{CommandFactory, Parser, error::ErrorKind};

mod args;
mod launcher;
mod token;

fn main() {
    let args = args::Args::parse();

    let token = args
        .token
        .clone()
        .unwrap_or_else(|| match token::get_token(&args.ticket_dir) {
            Ok(t) => t,
            Err(e) => args::Args::command()
                .error(
                    ErrorKind::InvalidValue,
                    format!("failed to get NBL token: {}", e),
                )
                .exit(),
        });

    let cfg = launcher::Config {
        monitoring_bin_path: args.lm_bin_path,
        monitoring_port: args.lm_port,
        monitoring_host: args.lm_host,
        monigoring_db_dir: args.db_dir,
        nbl_grpc_port: args.grpc_port,
        prometheus_port: args.prometheus_port,
        gprc_token: token,
    };

    if let Err(e) = launcher::launch(&cfg) {
        args::Args::command()
            .error(ErrorKind::Io, format!("failed to launch: {}", e))
            .exit();
    }
}
