use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::Base64Charset;

#[derive(Debug, Clone, Parser)]
pub struct TextOpts {
    #[command(subcommand, help = "Encrypt / Decrypt the input text")]
    pub action: TextAction,
}

#[derive(Debug, Clone, Subcommand)]
pub enum TextAction {
    #[command(name = "encrypt", about = "Encrypt input")]
    Encrypt(SubCommandOpt),
    #[command(name = "decrypt", about = "Decrypt input")]
    Decrypt(SubCommandOpt),
    #[command(name = "generate-key", about = "Generate a random key")]
    GenerateKey(SubCommandOpt),
}

#[derive(Debug, Clone, Parser)]
pub struct SubCommandOpt {
    #[arg(
        long,
        default_value = "standard-nopad",
        help = "input text base64 encode format Base64 Encode Charsets values [standard,standard-nopad,urlsafe,urlsafe-nopad,none], the none is not base64 encoded"
    )]
    pub in_format: Base64Charset,

    #[arg(
        long,
        default_value = "none",
        help = "output text base64 encode format  charsets values [standard,standard-nopad,urlsafe,urlsafe-nopad,none],the none is not base64 encoded"
    )]
    pub out_format: Base64Charset,

    #[arg(
        long,
        help = "encrypted/decrypted save file, if empty the output in stdout"
    )]
    pub output: Option<PathBuf>,

    #[arg(
        long,
        help = "If the input is a file, the file content will be used as the key to encrypt the text."
    )]
    pub key: String,
    #[arg(
        help = "The input to encrypt. If it is a file directory, the file content will be encrypted/decrypted. If it is text, the text will be encrypted/decrypted. If it is empty, the input in stdin will be obtained."
    )]
    pub content: Option<String>,
}
