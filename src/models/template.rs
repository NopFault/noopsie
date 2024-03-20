use crate::models::FileManager;
use regex::Regex;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Template;

impl Template {

    fn content(template_file_name: FileManager) -> String {
        let contents = template_file_name.read();
        let reg_parts = Regex::new(r"\#\[template\:\ ?(.*?)\]").unwrap();
        let cont = contents.unwrap_or_else(|_| String::from(""));
        let parts = reg_parts.captures_iter(cont.as_str());

        let pt = template_file_name.path.clone();
        let path = Path::new(pt.as_str());
        for part in parts {
            if let (Some(loc), Some(pth)) = (part.get(1), path.parent()) {
                    let partfile = FileManager::new(format!("{}/{}", pth.display(), loc.as_str())).read().unwrap_or_else(|_| String::from(""));
                     return Regex::new(format!(r"\#\[template\:\ ?{}\]", loc.as_str()).as_str())
                                .unwrap()
                                .replace_all(cont.as_str(), partfile)
                                .into_owned();
            }
        }
        String::from("")
    }
    
    pub fn generate(template_location: String) {
        let templates = FileManager::list_files(format!("{}/{}/", template_location, "templates"), "html");
        templates.iter().for_each(|template| {
            let content = Template::content(template.clone());
            let file_name = Path::new(&template.path).file_name().unwrap().to_str().unwrap().to_string();
            FileManager::create(format!("{}/{}","./public",file_name), content);
            println!("    * Template: {:?} created!", template);
        });
    }
}

