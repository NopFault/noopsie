use crate::models::{FileManager, Post, Config, AttributeType};
use regex::Regex;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Template;

impl Template {

    pub fn content(template_file_name: FileManager) -> String {
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
        let templates = FileManager::list_files(format!("{}/{}/", template_location, "pages"), "html");
        templates.iter().for_each(|template| {
            let mut content = Template::content(template.clone());
            
            let posts = Post::get_posts(template_location.clone());

            if content.split("{posts}").count() > 1 {
                let posts_template = FileManager::new(format!("{}/{}/{}.{}", template_location, "templates", "posts", "html")).read().unwrap_or(String::from(""));
                let mut dynamic_post_content: String = String::from("");

                for post in posts.iter() {
                    dynamic_post_content.push_str(&posts_template
                        .replace("{title}", post.meta.title.as_str())
                        .replace("{date}", post.meta.date.as_str())
                        .replace("{author}", post.meta.author.as_str())
                        .replace("{intro}", post.meta.intro.as_str())
                        .replace("{slug}", format!("{}.{}", post.meta.slug.trim(), "html").as_str())
                    );
                    
                }
                content = content.replace("{posts}", &dynamic_post_content);

                let config_file = FileManager::new(format!("{}/{}.{}", template_location, "config", "cfg"));
                let config = config_file.to_config().unwrap_or(Config::new());
                let per_page = match config.get(String::from(Config::PER_PAGE)).value {
                    AttributeType::Uint(val) => val,
                    _ => 5 // default
                };
                let posts_count = posts.iter().count() as u32;
                if posts_count as u32 > per_page {
                    let talpa = posts_count / per_page;
                    for page_nr in 1..talpa {
                        FileManager::create_dir(format!("{}/{}", "./public", page_nr));
                        let file_name = Path::new(&template.path).file_name().unwrap().to_str().unwrap().to_string();
                        FileManager::create(format!("{}/{}/{}","./public",page_nr,file_name), content.clone());
                    }
                }
            }

            let file_name = Path::new(&template.path).file_name().unwrap().to_str().unwrap().to_string();
            FileManager::create(format!("{}/{}","./public",file_name), content);
        });
    }
}

