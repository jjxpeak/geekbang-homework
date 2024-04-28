use crate::cli::JwtOpts;
use crate::Actuator;
use chrono::DateTime;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha512;
use std::collections::BTreeMap;

impl Actuator for JwtOpts {
    fn execute(self) -> anyhow::Result<()> {
        match self.action {
            crate::JwtAction::Sign {
                subject,
                issuer,
                expiration_time,
                audience,
                nbf,
                iat,
                jti,
                data,
            } => {
                let key = Hmac::<Sha512>::new_from_slice(self.key.as_bytes())?;

                let header = Header {
                    algorithm: AlgorithmType::Hs512,
                    ..Default::default()
                };

                let mut payload: BTreeMap<String, String> = BTreeMap::new();
                subject.map(|x| payload.insert("sub".to_string(), x));

                issuer.map(|x| payload.insert("iss".to_string(), x));

                expiration_time
                    .map(|x| payload.insert("exp".to_string(), x.timestamp().to_string()));

                audience.map(|x| payload.insert("aud".to_string(), x));

                nbf.map(|x| payload.insert("nbf".to_string(), x.timestamp().to_string()));

                iat.map(|x| payload.insert("iat".to_string(), x.timestamp().to_string()));

                jti.map(|x| payload.insert("jti".to_string(), x));

                payload.insert("data".to_owned(), data);
                let jwt = Token::new(header, payload).sign_with_key(&key)?;
                print!("{}", jwt.as_str());
                Ok(())
            }
            crate::JwtAction::Verify { token } => {
                let key = Hmac::<Sha512>::new_from_slice(self.key.as_bytes())?;
                let token: Result<Token<Header, BTreeMap<String, String>, _>, _> =
                    token.verify_with_key(&key);
                match token {
                    Ok(token) => {
                        let payload = token.claims();
                        let exp = payload
                            .get("exp")
                            .map(|e| DateTime::from_timestamp(e.parse().unwrap(), 0))
                            .unwrap()
                            .expect("Invalid JWT");
                        let nbf = payload
                            .get("nbf")
                            .map(|n| DateTime::from_timestamp(n.parse().unwrap(), 0))
                            .unwrap()
                            .expect("Invalid JWT");

                        let iat = payload
                            .get("iat")
                            .map(|i| DateTime::from_timestamp(i.parse().unwrap(), 0))
                            .unwrap()
                            .expect("Invalid JWT");

                        let now = chrono::Utc::now();
                        if !(now < exp && now > nbf && now > iat) {
                            println!("JWT expired");
                            return Ok(());
                        }
                        println!("JWT is valid");
                    }
                    Err(_) => {
                        println!("Invalid JWT");
                    }
                }
                Ok(())
            }
        }
    }
}
