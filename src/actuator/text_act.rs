use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};

use crate::{
    utils::{base64_decode, base64_encode, get_reader, reader_content, reader_content_str},
    Actuator, Base64Charset, TextAction, TextOpts,
};

impl Actuator for TextOpts {
    fn execute(self) -> anyhow::Result<()> {
        match self.action {
            TextAction::Encrypt(opt) => {
                let content = read_content(opt.content)?;
                let mut key = get_reader(&opt.key)?;

                // 自定义Key的使用chacha20poly1305对text进行加密
                let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(
                    reader_content(&mut key)?.as_slice(),
                ));

                let nonce = ChaCha20Poly1305::generate_nonce(OsRng);
                match cipher.encrypt(&nonce, content.as_ref()) {
                    Ok(c) => {
                        output_text(c, opt.output, opt.format)?;
                        Ok(())
                    }
                    Err(e) => Err(anyhow::Error::msg(e.to_string())),
                }
            }
            TextAction::Decrypt(opt) => {
                let content = read_content(opt.content)?;

                let content = base64_decode(&opt.format, content.as_bytes())?;

                let mut key = get_reader(&opt.key)?;
                let key = reader_content(&mut key)?;
                let key = GenericArray::clone_from_slice(key.as_slice());
                // 自定义Key的使用chacha20poly1305对text进行加密
                let cipher = ChaCha20Poly1305::new(&key);
                let nonce: GenericArray<u8, _> = ChaCha20Poly1305::generate_nonce(OsRng);
                match cipher.decrypt(&nonce, content.as_ref()) {
                    Ok(c) => {
                        output_text(c, opt.output, opt.format)?;
                        Ok(())
                    }
                    Err(e) => Err(anyhow::Error::msg(e.to_string())),
                }
            }
            TextAction::GenerateKey(opt) => {
                let key = ChaCha20Poly1305::generate_key(OsRng);
                let mut f = OpenOptions::new()
                    .truncate(true)
                    .create(true)
                    .write(true)
                    .open(opt.key)?;
                f.write_all(key.as_slice())?;
                println!("key: {:?}", key);
                f.flush()?;
                Ok(())
            }
        }
    }
}

fn read_content(content: Option<String>) -> anyhow::Result<String, anyhow::Error> {
    match content {
        Some(c) => {
            let mut content = get_reader(&c)?;
            Ok(reader_content_str(&mut content)?)
        }
        None => {
            let mut buf = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn output_text(
    content: Vec<u8>,
    path: Option<PathBuf>,
    format: Base64Charset,
) -> anyhow::Result<()> {
    let content = base64_encode(&format, &content)?;
    println!("{}", content);
    match path {
        Some(path) => fs::write(path, content)?,
        None => println!("{}", content),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_read_content_some() {
        let content = read_content(Some("fixtures/text_read_content.txt\n".to_string()));
        assert!(content.is_ok());
        assert_eq!("123123", content.unwrap());
    }

    #[test]
    fn text_read_content_none() {
        let content = read_content(None);
        assert!(content.is_ok());
        assert_eq!("", content.unwrap());
    }

    #[test]
    #[ignore = "test"]
    fn text_decrypt_encrypt() {
        let path: String = String::from("fixtures/key.pem");
        let mut key = get_reader(&path).unwrap();
        let key = reader_content(&mut key).unwrap();
        let key = GenericArray::clone_from_slice(key.as_slice());
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(OsRng); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(&nonce, b"helloworld".as_ref()).unwrap();
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
        assert_eq!(&plaintext, b"helloworld");
    }
}
