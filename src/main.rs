use crate::models::{FileManager, Config, AttributeType, Template};
use clinop::CliNop;
use std::env;

pub mod models;


fn build(template_location: String) {
    FileManager::build_dirs();
    println!("    * Public deleted <-> Created");


    println!("--- [ Template ] ---");
    let _ = Template::generate(template_location);
}

fn main() {
    let arguments: CliNop = CliNop::new(env::args().collect());

    let template_base = arguments.get::<String>("template").unwrap();

    // Config
    let config_file = FileManager::new(format!("{}/{}.{}", template_base, "config", "cfg"));
    let config = config_file.to_config().unwrap();


    let per_page = match config.get(String::from(Config::PER_PAGE)).value {
        AttributeType::Uint(val) => val,
        _ => 5 // default
    };

    println!("DEBUG: ");
    println!("--- [ Config ] ---");
    println!("PER_PAGE: {:?}", per_page);

    println!("--- [ Build ] ---");
    build(template_base.clone());

}
