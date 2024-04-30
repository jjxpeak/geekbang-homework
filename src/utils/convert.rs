use anyhow::Result;

use std::io::Read;

type YamlValue = serde_yaml::Value;
type JsonValue = serde_json::Value;
type TomlValue = toml::Value;

pub fn reader_content(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(32);
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn toml_to_json(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let buf = reader_content_str(reader)?;
    let toml: TomlValue = toml::from_str(&buf)?;
    let buf = serde_json::to_value(toml)?.to_string().as_bytes().to_vec();
    Ok(buf)
}

pub fn yml_to_json(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let ymal: YamlValue = serde_yaml::from_reader(reader)?;
    let json = serde_json::to_string(&ymal)?;
    Ok(json.as_bytes().to_vec())
}

pub fn json_to_yml(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let json: JsonValue = serde_json::from_reader(reader)?;
    let yaml = serde_yaml::to_string(&json)?;
    Ok(yaml.as_bytes().to_vec())
}

pub fn reader_content_str(reader: &mut dyn Read) -> Result<String> {
    let buf = reader_content(reader)?;
    Ok(String::from_utf8(buf)?.trim().to_string())
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::utils::get_reader;

    use super::*;

    #[test]
    fn test_yml_to_json() -> Result<()> {
        let path = PathBuf::from("fixtures/convert_yml.yml");
        let mut reader = get_reader(path.to_str().unwrap())?;
        let json = yml_to_json(&mut reader)?;
        println!("json: {:?}", String::from_utf8(json)?);
        Ok(())
    }

    #[test]
    fn test_json_to_yml() -> Result<()> {
        let path = PathBuf::from("fixtures/convert_json.json");
        let mut reader = get_reader(path.to_str().unwrap())?;
        let yml = json_to_yml(&mut reader)?;
        println!("yml: {:#}", String::from_utf8(yml)?);
        Ok(())
    }

    #[test]
    fn test_toml_to_json() -> Result<()> {
        let path = PathBuf::from("fixtures/convert_toml.toml");
        let mut reader = get_reader(path.to_str().unwrap())?;
        let json = toml_to_json(&mut reader)?;
        println!("json: {:?}", String::from_utf8(json)?);
        Ok(())
    }
}
