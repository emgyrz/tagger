use serde::Deserialize;

use docopt::Docopt;

const USAGE: &str = r#"
tagger

Simple utility to do something (e.g. install) packages from git repository by selecting git tag which is presented by semver rules.
By mz <mz@addreality.com>

Usage: tagger (--help | --version)
       tagger [--cfg PATH] (--show-latest | --list-all) PACKAGES...
       tagger [--exec --cfg PATH] PACKAGES...

Options:
    -h, --help              Show this message.
    -v, --version           Show the version of tagger.
    -c PATH, --cfg PATH     Path to config file. If not specified config will be searching in ./.tagger.cfg.json and ~/.tagger.cfg.json"
    -l, --show-latest       Prints latest valid tag.
    -a, --list-all          Prints all valid by semver tags.
    -e, --exec              *optional. Do something with specified or latest package version.

Examples:
    tagger ui
    tagger --show-latest hlp
    tagger --exec -c ../path_to_tagger_config.json hlp@2.1.3
"#;


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Args {
  pub arg_PACKAGES: Vec<String>,
  pub flag_cfg: Option<String>,
  pub flag_help: bool,
  pub flag_version: bool,
  pub flag_show_latest: bool,
  pub flag_list_all: bool,
}


pub fn parse_args() -> Result<Args, Box<std::error::Error>> {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.argv(std::env::args()).deserialize())
    .unwrap_or_else(|e| e.exit());

  Ok(args)
}

