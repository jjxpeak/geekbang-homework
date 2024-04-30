use anyhow::Result;
use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
};
use stringreader::StringReader;

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

#[cfg(test)]
mod tests {
    use crate::utils::reader_content_str;

    use super::*;

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
