use core::str;

use chrono::{DateTime, FixedOffset};
use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
pub struct JwtOpts {
    #[command(subcommand)]
    pub action: JwtAction,

    #[arg(long, name = "key", help = "Key", default_value = "some-secret")]
    pub key: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum JwtAction {
    #[command(name = "sign", about = "Sign JWT")]
    Sign {
        #[arg(long = "sub", help = "Subject")]
        subject: Option<String>,

        #[arg(long = "iss", help = "Issuer")]
        issuer: Option<String>,

        #[arg(long = "exp", default_value= "1days" ,help = "Expiration time in seconds. \n[<+/-><number><until>]+ examples '-1days+2hours-3minutes+4seconds',\nuntil keyword is 'years|months|fortnights|weeks|days|hours|h|minutes|mins|m|seconds|secs|s|yesterday|tomorrow|now|today'",value_parser = parse_datetime)]
        expiration_time: Option<DateTime<FixedOffset>>,

        #[arg(long = "aud", help = "Audience")]
        audience: Option<String>,

        #[arg(long = "nbf", default_value= "now" , help = "Not Before time in seconds.[<+/-><number><until>]+\n examples '-1days+2hours-3minutes+4seconds',\nuntil keyword is 'years|months|fortnights|weeks|days|hours|h|minutes|mins|m|seconds|secs|s|yesterday|tomorrow|now|today'",value_parser = parse_datetime)]
        nbf: Option<DateTime<FixedOffset>>,

        #[arg(long = "iat", default_value= "now" , help = "Issued At time in seconds.\n[<+/-><number><until>]+ examples '-1days+2hours-3minutes+4seconds',\nuntil keyword is 'years|months|fortnights|weeks|days|hours|h|minutes|mins|m|seconds|secs|s|yesterday|tomorrow|now|today'",value_parser = parse_datetime)]
        iat: Option<DateTime<FixedOffset>>,

        #[arg(long = "jti", help = "JWT ID")]
        jti: Option<String>,
        #[arg(
            name = "data",
            help = "Write the data in jwt. If it is a file, read the file content and write it. The supported file type is Json/Yaml/toml."
        )]
        data: String,
    },
    #[command(name = "verify", about = "Verify JWT")]
    Verify {
        #[arg(name = "token", help = "Verify Jwt. ")]
        token: String,
    },
}

fn parse_datetime(str: &str) -> anyhow::Result<DateTime<FixedOffset>> {
    parse_datetime::parse_datetime(str).map_err(|e| anyhow::anyhow!(e))
}

#[cfg(test)]
mod test {

    use parse_datetime::parse_datetime;
    #[test]
    fn test_parse_datetime() {
        let date = parse_datetime("-2days-3hours-4minutes-5seconds");
        println!("{:?}", date);
        assert!(date.is_ok());
    }
}
