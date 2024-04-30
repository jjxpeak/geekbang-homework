use crate::cli::JwtOpts;
use crate::utils::{get_reader, reader_content, toml_to_json, yml_to_json};
use crate::{Actuator, DataFormat, JwtSignOpts};
use anyhow::anyhow;
use anyhow::Result;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha512;
use std::collections::BTreeMap;

impl Actuator for JwtOpts {
    fn execute(self) -> anyhow::Result<()> {
        match self.action {
            crate::JwtAction::Sign(ops) => {
                let payload = sigin_opt_to_btree_map(ops)?;
                let jwt = jwt_sign(self.key.as_str(), payload)?;
                println!("{}", jwt);
                Ok(())
            }
            crate::JwtAction::Verify(ops) => {
                jwt_verify(&self.key, &ops.token)?;
                println!("verify success");
                Ok(())
            }
        }
    }
}

fn sigin_opt_to_btree_map(ops: JwtSignOpts) -> anyhow::Result<BTreeMap<String, String>> {
    let mut payload: BTreeMap<String, String> = BTreeMap::new();
    ops.subject.map(|x| payload.insert("sub".to_string(), x));

    ops.issuer.map(|x| payload.insert("iss".to_string(), x));

    ops.expiration_time
        .map(|x| payload.insert("exp".to_string(), x.timestamp().to_string()));

    ops.audience.map(|x| payload.insert("aud".to_string(), x));

    ops.nbf
        .map(|x| payload.insert("nbf".to_string(), x.timestamp().to_string()));

    ops.iat
        .map(|x| payload.insert("iat".to_string(), x.timestamp().to_string()));

    ops.jti.map(|x| payload.insert("jti".to_string(), x));

    let data = parse_data_by_data_format(ops.data.as_str(), &ops.data_format)?;
    payload.insert("data".to_owned(), data);
    Ok(payload)
}

fn parse_data_by_data_format(data: &str, data_format: &DataFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(data)?;
    let content = match data_format {
        DataFormat::Json => reader_content(&mut reader)?,
        DataFormat::Yaml => yml_to_json(&mut reader)?,
        DataFormat::Toml => toml_to_json(&mut reader)?,
        DataFormat::Text => reader_content(&mut reader)?,
    };
    Ok(String::from_utf8(content)?)
}

fn jwt_sign(key: &str, payload: BTreeMap<String, String>) -> anyhow::Result<String> {
    let key = Hmac::<Sha512>::new_from_slice(key.as_bytes())?;

    let header = Header {
        algorithm: AlgorithmType::Hs512,
        ..Default::default()
    };
    let jwt = Token::new(header, payload).sign_with_key(&key)?;
    Ok(jwt.as_str().to_string())
}

fn jwt_verify(key: &str, token: &str) -> anyhow::Result<bool> {
    let key = Hmac::<Sha512>::new_from_slice(key.as_bytes())?;
    let token: Result<Token<Header, BTreeMap<String, String>, _>, _> = token.verify_with_key(&key);
    match token {
        Ok(token) => {
            let payload: &BTreeMap<String, String> = token.claims();
            let exp = parse_timestamp_to_datetime(payload.get("exp"))?;
            let nbf = parse_timestamp_to_datetime(payload.get("nbf"))?;
            let iat = parse_timestamp_to_datetime(payload.get("iat"))?;

            let now = chrono::Utc::now();
            if !(now < exp && now > nbf && now > iat) {
                return Err(anyhow!("JWT expired"));
            }
            Ok(true)
        }
        Err(_) => Err(anyhow!("Invalid JWT")),
    }
}

fn parse_timestamp_to_datetime(timestamp: Option<&String>) -> anyhow::Result<DateTime<Utc>> {
    let timestamp = timestamp.unwrap();
    let timestamp = timestamp.parse::<i64>()?;
    match DateTime::from_timestamp(timestamp, 0) {
        Some(date) => Ok(date),
        None => Err(anyhow!("Timestamp out of range")),
    }
}
