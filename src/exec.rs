
use super::pkg::Pkg;
use std::process::Command;

pub fn run(pkg: &Pkg) -> std::process::Output {

  let mut pkg_str = String::new();
  pkg_str.push_str(&pkg.url);

  if let Some(version) = &pkg.version {
    pkg_str.push_str(&format!("#{}", version));
  }

  let cmd = format!("yarn add {}", pkg_str);

  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", &cmd ] )
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .args(&["-c", &cmd ])
      .output()
      .expect("failed to execute process")
  };
  output
}
