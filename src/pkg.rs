use super::cfg::Repo;
use semver::{SemVerError, Version};


struct PkgUnchecked {
  name: String,
  raw_version: String,
  version: Option<Result<Version, SemVerError>>,
}
#[derive(Debug)]
pub struct Pkg {
  pub name: String,
  pub version: Option<String>,
  pub url: String,
}

// type Pkg = (String,Result<Version,SemVerError>);

pub fn get_package_args(pkgs: Vec<String>, cfg_repos: &[Repo]) -> Vec<Pkg> {
  let valid_pkg_args = parse_package_args(pkgs);

  valid_pkg_args
    .iter()
    .filter_map(|(name, version)| {
      if let Some(repo) = cfg_repos.iter().find(|r| r.name == *name) {
        Some(Pkg {
          name: name.clone(),
          url: repo.url.clone(),
          version: version.clone(),
        })
      } else {
        println!("Cannot find repo in config for package `{}`", name);
        None
      }
    })
    .collect()
}

fn parse_package_args(pkgs: Vec<String>) -> Vec<(String, Option<String>)> {
  let mut parsed: Vec<(String, Option<String>)> = Vec::with_capacity(pkgs.len());

  for pkg_str in pkgs {
    let pkg = parse_package_arg(&pkg_str);

    if pkg.name.is_empty() {
      continue;
    }

    if let Some(ver) = pkg.version {
      match ver {
        Ok(valid_ver) => {
          if format!("{}", valid_ver) == pkg.raw_version {
            parsed.push((pkg.name, Some(pkg.raw_version)));
          } else {
            println!("package `{}` has invalid version", pkg_str);
          }
        }
        Err(_) => {
          println!("package `{}` has invalid version", pkg_str);
        }
      }
    } else {
      parsed.push((pkg.name, None));
    }
  }

  if parsed.is_empty() {
    println!("Error! No any valid package is specified");
    std::process::exit(1);
  }

  parsed
}


fn parse_package_arg(pkg: &str) -> PkgUnchecked {
  let mut splitted = pkg.splitn(2, '@');
  let name = splitted.next().unwrap_or("");
  let mut raw_version = String::new();
  let version = if let Some(ver) = splitted.next() {
    raw_version = ver.to_owned();
    Some(Version::parse(ver))
  } else {
    None
  };

  PkgUnchecked {
    name: name.to_owned(),
    version,
    raw_version,
  }
}
