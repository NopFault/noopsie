use crate::models::{FileManager, Post, Config, AttributeType, Template};
use clinop::CliNop;
use std::env;

pub mod models;


fn build(template_location: String) {

    FileManager::create_dir(String::from("./public"));

    let _ = Template::generate(template_location.clone());
    let posts = Post::get_posts(template_location.clone());

    for post in posts.clone() {

        post.save();
    }
}

fn main() {
    let arguments: CliNop = CliNop::new(env::args().collect());
    let template_base = arguments.get::<String>("template").unwrap();

    build(template_base.clone());

}
