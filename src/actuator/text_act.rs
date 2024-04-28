use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use aead_io::{ArrayBuffer, DecryptBE32BufReader, EncryptBE32BufWriter};
use chacha20poly1305::{aead::OsRng, ChaCha20Poly1305, KeyInit};

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
                let key = reader_content(&mut key)?;
                let mut ciphertext = Vec::default();
                {
                    let mut writer = EncryptBE32BufWriter::<ChaCha20Poly1305, _, _>::new(
                        key.as_slice().into(),
                        &Default::default(), // please use a better nonce ;)
                        ArrayBuffer::<128>::new(),
                        &mut ciphertext,
                    )?;

                    writer.write_all(content.as_bytes())?;
                    writer.flush()?;
                }
                output_result(ciphertext, opt.output, opt.out_format)?;
                Ok(())
            }
            TextAction::Decrypt(opt) => {
                let content = read_content(opt.content)?;

                let content = base64_decode(&opt.in_format, content.as_bytes())?;
                let mut key = get_reader(&opt.key)?;
                let key = reader_content(&mut key)?;

                let mut decrypted = Vec::new();
                {
                    let mut reader = DecryptBE32BufReader::<ChaCha20Poly1305, _, _>::new(
                        key.as_slice().into(),
                        ArrayBuffer::<256>::new(),
                        content.as_slice(),
                    )?;
                    let _ = reader.read_to_end(&mut decrypted)?;
                };

                output_result(decrypted, opt.output, opt.out_format)?;
                Ok(())
            }
            TextAction::GenerateKey(opt) => {
                let key = ChaCha20Poly1305::generate_key(OsRng);
                let mut f = OpenOptions::new()
                    .truncate(true)
                    .create(true)
                    .write(true)
                    .open(opt.key)?;
                f.write_all(key.as_slice())?;
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

fn output_result(
    content: Vec<u8>,
    path: Option<PathBuf>,
    format: Base64Charset,
) -> anyhow::Result<()> {
    let content = base64_encode(&format, &content)?;
    match path {
        Some(path) => fs::write(path, content)?,
        None => println!("{}", content),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aead_io::{ArrayBuffer, DecryptBE32BufReader, EncryptBE32BufWriter};

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
    fn text_decrypt_encrypt() -> anyhow::Result<()> {
        let key = b"my very super super secret key!!".into();
        println!("key: {:?}", "my very super super secret key!!".len());
        let plaintext = b"hello world!";

        let mut ciphertext = Vec::default();
        {
            let mut writer = EncryptBE32BufWriter::<ChaCha20Poly1305, _, _>::new(
                key,
                &Default::default(), // please use a better nonce ;)
                ArrayBuffer::<128>::new(),
                &mut ciphertext,
            )
            .unwrap();
            writer.write_all(plaintext)?;
            writer.flush()?;
        };

        let mut decrypted = Vec::new();
        {
            let mut reader = DecryptBE32BufReader::<ChaCha20Poly1305, _, _>::new(
                key,
                ArrayBuffer::<256>::new(),
                ciphertext.as_slice(),
            )
            .unwrap();
            let _ = reader.read_to_end(&mut decrypted).unwrap();
        };
        println!("decrypted: {:?}", decrypted);
        println!("plaintext: {:?}", plaintext);
        println!("ciphertext: {:?}", ciphertext);
        assert_eq!(decrypted, plaintext);
        Ok(())
    }
}
