use std::collections::HashMap;
use anyhow::{Context, Result};
use textreader::TextReader;
use toml::Value;


#[derive(Default, Debug)]
pub struct TomlData {
    pub data: HashMap<String, HashMap<String, Value>>,
}


impl TomlData {

    pub fn new() -> TomlData {
        TomlData::default()
    }


    pub fn read(&mut self, file_path:&str) -> Result<&mut Self> {
        let toml_text = TextReader::open(file_path)
            .with_context(|| format!("TomlData: failed to open file: \"{}\"", file_path))?
            .read()
            .with_context(|| format!("TomlData: failed to read from file: \"{}\"", file_path))?;
        
        self.data = toml::from_str(&toml_text)
            .context(format!("TomlData: failed to parse TOML data: \"{}\"", file_path))?;

        Ok(self)
    }

    #[allow(dead_code)]
    pub fn get(&self, table_name:&str, key_name:&str) -> Result<Value> {
        let value = self.data.get(table_name)
            .context(format!("TomlData: not found table name: \"{}\"", table_name))?
            .get(key_name)
            .context(format!("TomlData: not found key name: \"{}\" in table:[{}]", key_name, table_name))?
            .clone();
        Ok(value)
    }


    #[allow(dead_code)]
    pub fn get_string(&self, table_name:&str, key_name:&str) -> Result<String> {
        let binding = self.get(table_name, key_name)?;
        let value = binding.as_str()
            .context(format!("TomlData: value is not a string [{}] \"{}\"", table_name, key_name))?;
        Ok(value.to_string())
    }
}



/*
    let mut t = TomlData::new();
    println!("{:?}", &t.read("test.toml")?.get("table", "key")?);
*/
