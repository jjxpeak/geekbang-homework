use clap::Parser;
use rcli::{cli::Rcli, Actuator};

fn main() -> anyhow::Result<()> {
    let rcli = Rcli::parse();

    match rcli.command {
        rcli::Commands::Text(text_opt) => text_opt.execute(),
        rcli::Commands::Jwt(_jwt_opt) => todo!(),
        rcli::Commands::Ftp(_ftp_opt) => todo!(),
    }
}
