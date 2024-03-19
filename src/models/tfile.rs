use crate::models::{Config, Template, AttributeType};
use std::fs;
use std::io;


#[derive(Debug, Clone)]
pub struct TFile {
    pub path: String,
}

impl TFile {
    pub fn new(path: String) -> TFile {
        TFile { path }
    }

    pub fn read(&self) -> Result<String, io::Error> {
        let content = fs::read_to_string(&self.path)?;
        Ok(content)
    }

    pub fn to_template(&self, name: String) -> Template {
        Template::new(self.path.clone(), name)
    }

    pub fn to_config(&self) -> Option<Config> {
        let mut config: Config = Config::new();

        let contents = self.read().unwrap_or_else(|_| String::from(""));
        let lines: Vec<&str> = contents.trim().split('\n').collect();

        for line in lines {
            if line.chars().nth(0).unwrap_or(';') != ';' {
                let params: Vec<&str> = line.trim().split('=').collect();
                if params[0].trim() == Config::PER_PAGE {
                    config.add(String::from(Config::PER_PAGE), AttributeType::Uint(params[1].parse::<u32>().unwrap()));
                }
                return Some(config);
            }
        }
        None
    }
}
