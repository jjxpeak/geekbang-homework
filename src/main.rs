use std::env::set_var;

use clap::Parser;
use rcli::{cli::Rcli, Actuator};

fn main() -> anyhow::Result<()> {
    set_var("RUST_LOG", "INFO");
    tracing_subscriber::fmt::init();
    let rcli = Rcli::parse();

    match rcli.command {
        rcli::Commands::Text(text_opt) => text_opt.execute(),
        rcli::Commands::Jwt(jwt_opt) => jwt_opt.execute(),
        rcli::Commands::Ftp(ftp_opt) => ftp_opt.execute(),
    }
}
