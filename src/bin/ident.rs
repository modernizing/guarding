use std::fs;
use std::path::PathBuf;

use clap::{AppSettings, Clap};

use guarding_ident::ModelBuilder;

#[derive(Clap)]
#[clap(version = "1.0", author = "Inherd Group <group@inherd.org>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = ".")]
    path: String,

    #[clap(short, long, default_value = "guard-ident.json")]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let code_dir = PathBuf::from(opts.path);

    let models = ModelBuilder::build_models_by_dir(code_dir);
    let content = serde_json::to_string_pretty(&models).unwrap();
    let _ = fs::write(opts.output, content);
}
