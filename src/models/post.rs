use crate::models::{FileManager, Template};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct PostMeta {
    pub title: String,
    pub author: String,
    pub date: String,
    pub intro: String,
    pub slug: String
}

#[derive(Debug, Clone)]
pub struct Post {
    pub meta: PostMeta,
    content: String
}


impl Post {

    fn get_meta(metaline:String) -> PostMeta {
        let lines = metaline.as_str().split('\n');

        let mut title = String::from("");
        let mut author = String::from("");
        let mut date = String::from("");
        let mut intro = String::from("");
        let mut slug = String::from("");

        for line in lines {
            let pairs = line.split("title:");
            if pairs.clone().count() > 1 {
                title = pairs.last().unwrap_or("").to_string();
            }

            let pairs = line.split("author:");
            if pairs.clone().count() > 1 {
                author = pairs.last().unwrap_or("").to_string();
            }

            let pairs = line.split("date:");
            if pairs.clone().count() > 1 {
                date = pairs.last().unwrap_or("").to_string();
            }

            let pairs = line.split("intro:");
            if pairs.clone().count() > 1 {
                intro = pairs.last().unwrap_or("").to_string();
            }

            let pairs = line.split("slug:");
            if pairs.clone().count() > 1 {
                slug = pairs.last().unwrap_or("").to_string().trim().replace(" ","-");
            }

        }
        PostMeta{title,author,date,intro,slug}
    }

    pub fn get_posts(template_location: String) -> Vec<Post> {

        let post_template = Template::content(FileManager::new(format!("{}/{}/{}.{}", template_location.clone(), "templates", "post", "html")));

        FileManager::list_files(format!("{}/{}", template_location, "posts"), "md").iter().map(|post| {
            let post_content = post.read().unwrap_or(String::from(""));

            let regex = Regex::new(r"---((.|\n)*?)---").unwrap();

            let mut content: String = String::from("");
            let parsed_content = regex.replace(post_content.as_str(), "");
            let parser = pulldown_cmark::Parser::new(&parsed_content);

            pulldown_cmark::html::push_html(&mut content, parser);
                    
            let metadata = regex.captures(post_content.as_str()).unwrap();
            let metaline = metadata.get(1).unwrap().as_str();
            let meta = Post::get_meta(metaline.to_string());
            let template_content = post_template
                .replace("{content}", content.as_str())
                .replace("{title}", meta.title.clone().as_str())
                .replace("{author}", meta.author.clone().as_str())
                .replace("{date}", meta.date.clone().as_str())
                .replace("{slug}", meta.slug.clone().as_str())
                .replace("{intro}", meta.intro.clone().as_str());

            Post{meta,content:template_content}

        }).collect()
    }

    pub fn save(&self) {
        let slug = self.meta.slug
            .trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii()).collect::<String>()
            .replace(['!','@','#','$','%','^','&','*','(',')','+','?','>','<','±',',',';','"','\'','|','/',' '], "-");

            FileManager::create(format!("{}/{}.{}","./public", slug, "html"), self.content.clone());
    }
}
