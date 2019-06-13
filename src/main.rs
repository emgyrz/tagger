
use std::error::Error;
use std::io::{self, Write};

static VERSION: &str = env!("CARGO_PKG_VERSION");

mod args;
pub mod cfg;
pub mod err;

mod exec;
pub mod pkg;

mod tags;
use tags::Tags;

fn main() -> Result<(), Box<Error>> {

  let args = args::parse_args()?;


  if args.flag_version {
    println!("`tagger` version is {}", VERSION);
    return Ok(());
  }

  let cfg = cfg::read_cfg(args.flag_cfg)?;


  let mut package_args = pkg::get_package_args(args.arg_PACKAGES, &cfg.repos);

  if package_args.is_empty() {
    println!("No valid package passed. Exiting");
    std::process::exit(1);
  }

  if args.flag_show_latest {
    for pkg in &package_args {
      Tags::new(pkg).print_latest().unwrap_or_else(|e| e.exit());
    }
    return Ok(());
  }

  if args.flag_list_all {
    for pkg in &package_args {
      Tags::new(pkg).print_all().unwrap_or_else(|e| e.exit());
    }
    return Ok(());
  }


  for pkg in &mut package_args {
    if pkg.version.is_none() {
      let latest_version = Tags::new(pkg).get_latest().unwrap_or_else(|e| {
        e.exit();
        String::new()
      });
      pkg.version = Some(latest_version);
    }
    let out = exec::run(&pkg, cfg.command.as_ref())?;
    io::stdout().write_all(&out.stdout).unwrap();
  }


  Ok(())

}
