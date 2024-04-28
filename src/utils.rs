use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::Base64Charset;
use anyhow::Result;
use base64::prelude::*;
use stringreader::StringReader;

pub fn base64_encode(charset: &Base64Charset, content: &[u8]) -> Result<String> {
    match charset {
        Base64Charset::Standard => Ok(BASE64_STANDARD.encode(content)),
        Base64Charset::StandardNoPad => Ok(BASE64_STANDARD_NO_PAD.encode(content)),
        Base64Charset::UrlSaff => Ok(BASE64_URL_SAFE.encode(content)),
        Base64Charset::UrlSafeNoPad => Ok(BASE64_URL_SAFE_NO_PAD.encode(content)),
        Base64Charset::None => Ok(String::from_utf8(content.to_vec())?),
    }
}

pub fn base64_decode(charset: &Base64Charset, content: &[u8]) -> Result<Vec<u8>> {
    match charset {
        Base64Charset::Standard => Ok(BASE64_STANDARD.decode(content)?),
        Base64Charset::StandardNoPad => Ok(BASE64_STANDARD_NO_PAD.decode(content)?),
        Base64Charset::UrlSaff => Ok(BASE64_URL_SAFE.decode(content)?),
        Base64Charset::UrlSafeNoPad => Ok(BASE64_URL_SAFE_NO_PAD.decode(content)?),
        Base64Charset::None => Ok(content.to_vec()),
    }
}

/// If the input is a file, return the file content, otherwise return the current parameters.
///
/// # Examples
///
/// ```no_run
/// echo "123321" > exist.file
///
///
/// use rcli::utils::read_file_content;
/// let path = "none.file";
/// assert_eq!(read_file_content(path), path );
///
///
/// let path = "exist.file";
/// assert_eq!(read_file_content(path), "123321" );
/// ```
pub fn get_reader<'a>(path: &'a str) -> Result<Box<dyn 'a + Read>> {
    let path = path.trim_end();
    let path_buf = PathBuf::from(path);
    if path_buf.exists() && path_buf.is_file() {
        let reader: Box<dyn Read> = match OpenOptions::new().read(true).open(path) {
            Ok(f) => Box::new(f),
            Err(_) => Box::new(BufReader::new(StringReader::new(path))),
        };
        return Ok(reader);
    }
    Ok(Box::new(BufReader::new(StringReader::new(path))))
}

pub fn reader_content(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(32);
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn reader_content_str(reader: &mut dyn Read) -> Result<String> {
    let buf = reader_content(reader)?;
    Ok(String::from_utf8(buf)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils_base64_standard_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = super::Base64Charset::Standard;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }

    #[test]
    fn utils_base64_standard_no_pad_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = super::Base64Charset::StandardNoPad;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }
    #[test]
    fn utils_base64_url_saff_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = super::Base64Charset::UrlSaff;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }
    #[test]
    fn utils_base64_url_saff_no_pad_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = super::Base64Charset::UrlSafeNoPad;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }

    #[test]
    fn utils_check_path_is_dir() -> Result<()> {
        let path = String::from("123");
        let mut text = get_reader(&path)?;
        let text: String = reader_content_str(&mut text)?;
        println!("text: {}", text);
        assert_eq!(path.as_bytes(), text.as_bytes());
        Ok(())
    }

    #[test]
    fn utils_check_path_is_file() -> Result<()> {
        let path = String::from("fixtures/check_path.txt");
        let mut text = get_reader(&path)?;
        let text = reader_content_str(&mut text)?;
        assert_eq!("is file".as_bytes(), text.as_bytes());
        Ok(())
    }
}
