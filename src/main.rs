use crate::file::{Post, TFile, Template};
use clinop::CliNop;
use std::env;

mod file;

fn main() {
    let arguments: CliNop = CliNop::new(env::args().collect());

    let template_base = arguments.get::<String>("template").unwrap();

    // Config
    let config_file = TFile::new(format!("{}/{}.{}", template_base, "config", "cfg"));
    let config = config_file.to_config();
    println!("Config: {:?}", config);

    // Template
    let template_file: TFile = TFile::new(format!(
        "{}/{}/{}.{}",
        template_base, "templates", "index", "html"
    ));
    let template = template_file.to_template(String::from("index.html"));
    println!("Template: {:?}", template);

    let template_content = template.contents();
    println!("TC: {:?}", template_content);

    // Post
}
