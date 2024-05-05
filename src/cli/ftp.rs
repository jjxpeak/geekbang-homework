use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct FtpOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: String,

    #[arg(long, default_value = "9989")]
    pub port: u16,
}
