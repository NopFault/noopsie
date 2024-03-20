use crate::models::{Config, AttributeType};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::io::Write;


#[derive(Debug, Clone)]
pub struct FileManager {
    pub path: String,
}

impl FileManager {
    pub fn new(path: String) -> FileManager {
        FileManager { path }
    }

    pub fn read(&self) -> Result<String, io::Error> {
        let content = fs::read_to_string(&self.path)?;
        Ok(content)
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

    pub fn list_files(path: String, tipas: &str) -> Vec<FileManager> {
        let entries = fs::read_dir(path.clone()).unwrap();

        let files: Vec<FileManager> = entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let file_name = e.file_name();
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(tipas) {
                            Some(file_name_str.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
        .map(|file_name| {
            let file_path = PathBuf::from(path.clone()).join(&file_name);
            FileManager::new(file_path.to_string_lossy().into_owned())
        }).collect();

        files
    }

    pub fn create(location: String, content: String) {
        let mut file = fs::File::create(location).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    pub fn build_dirs() {
        if Path::new("./public").is_dir() {
            fs::remove_dir_all("./public").unwrap();
        }
        fs::create_dir("./public").unwrap();
    }
}
