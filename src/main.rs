
use std::error::Error;
use std::io::{self, Write};

static VERSION: &str = "0.1.3";


mod args;
pub mod cfg;
mod exec;
pub mod pkg;
mod tags;


fn main() -> Result<(), Box<Error>> {

  let args = args::parse_args()?;
  // println!("args: {:?}", args);

  let cfg = cfg::read_cfg(args.flag_cfg)?;
  // println!("cfg: {:?}", cfg);

  if args.flag_version {
    println!(
      "`tagger`, an update utility for Addreality, version {}",
      VERSION
    );
    return Ok(());
  }

  let mut package_args = pkg::get_package_args(args.arg_PACKAGES, &cfg.repos);
  // println!("package_args: {:?}", package_args);

  if package_args.is_empty() {
    println!("No valid package passed. Exiting");
    std::process::exit(1);
  }


  if args.flag_show_latest {
    for pkg in &package_args {
      tags::get_and_print_latest_valid_tag(&pkg);
    }
    return Ok(());
  }

  if args.flag_list_all {
    for pkg in &package_args {
      tags::fetch_and_list_valid_tags(&pkg);
    }
    return Ok(());
  }


  for pkg in &mut package_args {
    if pkg.version.is_none() {
      pkg.version = tags::get_latest_valid_tag(&pkg.url);
    }
    let out = exec::run(&pkg);
    io::stdout().write_all(&out.stdout).unwrap();
  }


  Ok(())

}
