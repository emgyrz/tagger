use git2::{Direction, RemoteCallbacks, Repository};
use tempdir::TempDir;

use super::pkg::Pkg;


fn git_credentials_callback(
  _url: &str,
  username: Option<&str>,
  cred_type: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
  let user = username.unwrap_or("git");
  if cred_type.contains(git2::CredentialType::USERNAME) {
    return git2::Cred::username(user);
  }

  let home_dir = dirs::home_dir().unwrap_or_default();
  let key_path = std::path::Path::new(&home_dir).join(".ssh/id_rsa");
  git2::Cred::ssh_key(user, None, &key_path, None)
}


pub fn fetch_all_tags(url: &str) -> Vec<String> {

  // TODO:
  let tmp_dir = TempDir::new("tagger").unwrap();

  let repo = Repository::open_bare(tmp_dir.path())
    .unwrap_or_else(|_| Repository::init_bare(tmp_dir.path()).expect("cannot open or init repo"));

  let mut remote = repo
    .find_remote("origin")
    .unwrap_or_else(|_| repo.remote("origin", url).unwrap());

  let mut cbs = RemoteCallbacks::default();
  cbs.credentials(&git_credentials_callback);

  let connection = remote
    .connect_auth(Direction::Fetch, Some(cbs), None)
    .unwrap();

  let ls = connection.list().unwrap();

  let mut tags = Vec::new();

  for l in ls {
    let name = l.name();
    if name.starts_with("refs/tags/") {
      tags.push(name.replace("refs/tags/", ""));
    }
  }

  tags

}

fn fetch_valid_tags(url: &str) -> Vec<String> {
  fetch_all_tags(url)
    .iter()
    .filter_map(|tag| {
      let parsed = semver::Version::parse(tag);

      if parsed.is_ok() {
        Some(tag.to_owned())
      } else {
        None
      }
    })
    .collect()
}

pub fn fetch_and_list_valid_tags(pkg: &Pkg) {
  let tags = fetch_valid_tags(&pkg.url);
  let mut s = format!("\nValid tags available in repo {} :\n[\n", pkg.name);
  let tags_str = tags
    .iter()
    .map(|tag| format!("    {}\n", tag))
    .collect::<Vec<String>>()
    .join("");

  s.push_str(&tags_str);
  s.push(']');
  println!("{}", s);
}

pub fn get_latest_valid_tag(url: &str) -> Option<String> {
  let tags = fetch_valid_tags(url);

  let latest_version: Option<(&String, semver::Version)> = tags
    .iter()
    .filter_map(|tag| {
      let parsed = semver::Version::parse(tag);
      if let Ok(ver) = parsed {
        Some((tag, ver))
      } else {
        None
      }
    })
    .max_by(|(_, ver1), (_, ver2)| ver1.cmp(ver2));

  if let Some(ver) = latest_version {
    Some(ver.0.to_owned())
  } else {
    None
  }
}

pub fn get_and_print_latest_valid_tag(pkg: &Pkg) {
  let tag = get_latest_valid_tag(&pkg.url);
  if let Some(t) = tag {
    println!("\nLatest valid tag for repo `{}`: {}", pkg.name, t);
  } else {
    println!("\nRepo `{}` haven't valid tags", pkg.name);
  }
}

