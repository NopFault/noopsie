use std::fs;
use std::io;
use regex::Regex;
use std::path::Path;

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

trait Fileable {
    fn to_tfile(&self) -> TFile;
}

// Template file
//
#[derive(Debug, Clone)]
pub struct Template {
    pub path: String,
    name: String,
}

impl Fileable for Template {
    fn to_tfile(&self) -> TFile {
        TFile::new(self.path.clone())
    }
}
impl Template {
    pub fn new(path: String, name: String) -> Template {
        Template { path, name }
    }

    fn parts(&self) -> String {
        let contents = self.to_tfile().read();
        let reg_parts = Regex::new(r"\#\[template\:\ ?(.*?)\]").unwrap();
        let cont = contents.unwrap_or_else(|_| String::from(""));
        let parts = reg_parts.captures_iter(cont.as_str());

        let pt = self.path.clone();
        let path = Path::new(pt.as_str());
        for part in parts {
            if let (Some(loc), Some(pth)) = (part.get(1), path.parent()) {
                    let partfile = TFile::new(format!("{}/{}", pth.display(), loc.as_str())).read().unwrap_or_else(|_| String::from(""));
                     return Regex::new(format!(r"\#\[template\:\ ?{}\]", loc.as_str()).as_str())
                                .unwrap()
                                .replace_all(cont.as_str(), partfile)
                                .into_owned();
            }
        }
        String::from("")
    }

    pub fn contents(&self) -> String {
        self.parts()
    }
}

// Post file
//
#[derive(Debug, Clone)]
pub struct Post {
    pub path: String,
    title: String,
    author: String,
    date: String,
    intro: String,
    content: String,
    file_name: String,
}

impl Post {
    pub fn new(
        path: String,
        title: String,
        author: String,
        date: String,
        intro: String,
        content: String,
        file_name: String,
    ) -> Post {
        Post {
            path,
            title,
            author,
            date,
            intro,
            content,
            file_name,
        }
    }
}

impl Fileable for Post {
    fn to_tfile(&self) -> TFile {
        TFile::new(self.path.clone())
    }
}

// Config file
//
#[derive(Debug, Clone)]
enum AttributeType {
    Uint(u32),
    Int(i32),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    key: String,
    value: AttributeType,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub attributes: Vec<Attribute>
}

impl Config {
    pub const PER_PAGE: &str = "per_page";

    pub fn new() -> Self {
        Config {
            attributes: Vec::new(),
        }
    }

    pub fn add(&mut self, key: String, val: AttributeType) -> &mut Self {
        self.attributes.push(Attribute{ key, value: val});
        return self;
    }

    pub fn get(self, key:String) -> Attribute {
        self.attributes.iter().find(|attr| attr.key == key).cloned().unwrap()
    }


}
