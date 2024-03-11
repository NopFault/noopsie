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
        Template::new(self.path, name)
    }
}

trait Templatable {
    fn to_tfile(path:String) -> TFile {
        TFile::new(path)
    }
}

#[derive(Debug)]
pub struct Template {
    pub path: String,
    name: String,
}

impl Template {
    pub fn new(path: String, name: String) -> Template {
        Template { path, name }
    }
}

impl Templatable for Template {}


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

impl Templatable for Post {}



