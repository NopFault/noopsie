use crate::file::{Post, TFile, Template};
use clinop::CliNop;
use std::env;

mod file;

fn main() {
    let arguments: CliNop = CliNop::new(env::args().collect());

    let template_base = arguments.get::<String>("template");
}
