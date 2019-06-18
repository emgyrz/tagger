
use super::err::TaggerError;
use super::pkg::Pkg;

use std::error::Error;
use std::process::{Child, Command, Stdio};

static NAME: &str = "{NAME}";
static URL: &str = "{URL}";
static VERSION: &str = "{VERSION}";

fn generate_cmd_str(pkg: &Pkg, cmd_str: Option<&String>) -> Result<String, TaggerError> {
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

  Ok(cmd_str)
}


pub fn run(pkg: &Pkg, cmd_str: Option<&String>) -> Result<Child, TaggerError> {
  let is_win32 = cfg!(target_os = "windows");

  let exec_bin_str = if is_win32 { "cmd" } else { "sh" };
  let exec_bin_arg_str = if is_win32 { "/C" } else { "-c" };
  let cmd = generate_cmd_str(pkg, cmd_str)?;
  let args = [exec_bin_arg_str, &cmd];


  let cmd_child = Command::new(exec_bin_str)
    .args(&args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn();

  match cmd_child {
    Ok(child) => Ok(child),
    Err(e) => Err(TaggerError::new(e.description())),
  }

}
