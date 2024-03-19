use crate::models::{TFile};
use regex::Regex;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Template {
    pub path: String,
    name: String,
}

impl Template {
    pub fn new(path: String, name: String) -> Template {
        Template { path, name }
    }

    fn to_tfile(&self) -> TFile {
        TFile::new(self.path.clone())
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

