use std::fs;
use std::path::PathBuf;

use clap::{AppSettings, Clap};

use guarding::rule_executor::RuleExecutor;

#[derive(Clap)]
#[clap(version = "1.0", author = "Inherd Group <group@inherd.org>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = "guarding.guarding")]
    config: String,

    #[clap(short, long, default_value = "src")]
    path: String,

    #[clap(short, long, default_value = "guard.json")]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let buf = PathBuf::from(opts.path);
    let guarding = PathBuf::from(opts.config);
    let content = fs::read_to_string(guarding).unwrap();

    let errors = RuleExecutor::execute(content, buf);
    let content = serde_json::to_string_pretty(&errors).unwrap();
    let _ = fs::write(opts.output, content);
}
