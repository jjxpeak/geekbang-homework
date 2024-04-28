use clap::{Parser, Subcommand};

mod ftp;
mod jwt;
mod text;

pub use ftp::*;
pub use jwt::*;
pub use text::*;

#[derive(Debug, Clone, Parser)]
#[command(version)]
pub struct Rcli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[command(name = "text")]
    Text(TextOpts),
    #[command(name = "jwt")]
    Jwt(JwtOpts),
    #[command(name = "ftp")]
    Ftp(FtpOpts),
}
