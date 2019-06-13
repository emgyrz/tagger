
use super::err::TaggerError;
use super::pkg::Pkg;
use std::process::{Command, Output};

static NAME: &str = "{NAME}";
static URL: &str = "{URL}";
static VERSION: &str = "{VERSION}";


pub fn run(pkg: &Pkg, cmd_str: Option<&String>) -> Result<Output, TaggerError> {

  let mut cmd_str = if let Some(cs) = cmd_str {
    cs.to_owned()
  } else {
    return Err(TaggerError::new(""));
  };

  if cmd_str.find(&VERSION).is_some() {
    let pkg_ver = if let Some(version) = &pkg.version {
      version
    } else {
      return Err(TaggerError::new(
        "VERSION is using in command, but couldn't be resolved",
      ));
    };

    cmd_str = cmd_str.replace(&VERSION, &pkg_ver);
  }

  if cmd_str.find(&NAME).is_some() {
    cmd_str = cmd_str.replace(&NAME, &pkg.name);
  }

  if cmd_str.find(&URL).is_some() {
    cmd_str = cmd_str.replace(&URL, &pkg.url);
  }

  let cmd_output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", &cmd_str])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .args(&["-c", &cmd_str])
      .output()
      .expect("failed to execute process")
  };

  Ok(cmd_output)
}
