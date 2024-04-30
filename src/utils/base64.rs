use crate::Base64Charset;
use anyhow::Result;
use base64::prelude::*;

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn utils_base64_standard_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = crate::Base64Charset::Standard;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }

    #[test]
    fn utils_base64_standard_no_pad_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = crate::Base64Charset::StandardNoPad;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }
    #[test]
    fn utils_base64_url_saff_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = crate::Base64Charset::UrlSaff;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }
    #[test]
    fn utils_base64_url_saff_no_pad_test() -> Result<()> {
        let s: String = String::from("hello word!");
        let charset = crate::Base64Charset::UrlSafeNoPad;
        let encode = base64_encode(&charset, s.as_bytes())?;
        let decode = base64_decode(&charset, encode.as_bytes())?;
        assert_eq!(s.as_bytes(), decode.as_slice());
        Ok(())
    }
}
