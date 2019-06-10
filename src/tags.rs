use git2::{Direction, RemoteCallbacks, Repository, RemoteConnection};
use tempdir::TempDir;

use super::pkg::Pkg;
use super::err::TaggerError;


static ORIGIN: &str = "origin";
static TAG_REF_PREFIX: &str = "refs/tags/";


type TaggerResult<T> = std::result::Result<T,TaggerError>;

pub struct Tags<'a> {
  pkg: &'a Pkg,
}



impl<'a> Tags<'a> {

  fn get_tmp_dir(&self) -> TaggerResult<TempDir> {
    let tmp_dir_name = format!("tagger__{}", self.pkg.name);
    TempDir::new(&tmp_dir_name)
      .or( Err(TaggerError::new( "cannot create temp directory" ) ) )
  }

  fn get_repo(&self, dir: &TempDir) -> TaggerResult<Repository> {
    Repository::open_bare(dir.path())
      .or_else(|_| Repository::init_bare(dir.path())
      .or_else( |_| Err(TaggerError::new( "cannot create temp directory" ) ))
    )
  }

  fn get_remote_callbacks(&self) -> RemoteCallbacks {
    let mut cbs = RemoteCallbacks::default();

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

    cbs.credentials(&git_credentials_callback);
    cbs
  }


  fn connect_remote(&self, repo: &'static Repository) -> TaggerResult<RemoteConnection> {
    let mut remote = repo
      .find_remote(ORIGIN)
      .or( repo.remote(ORIGIN, &self.pkg.url)
      .or( Err(TaggerError::new( "cannot find or create remote with name origin" ) )
    ))?;

    let conn = (&mut remote)
      .connect_auth(Direction::Fetch, Some(self.get_remote_callbacks()), None)
      // .as_ref()
      .or_else(|e| Err(TaggerError::new(
        &format!("cannot create temp directory\n{}", e.message() )
      ) )
    );

    conn

  }


  pub fn fetch_all<'c>(&self) -> TaggerResult<Vec<String>> {
    let tmp_dir = self.get_tmp_dir()?;

    let repo:  &'static Repository = &self.get_repo(&tmp_dir)?;
    let connection = self.connect_remote(repo)?;

    let ls = connection.list()
      .or( Err(TaggerError::new( "cannot get remote repository referenses list" ) ) )?;

    let mut tags = Vec::new();

    for l in ls {
      let name = l.name();
      if name.starts_with(TAG_REF_PREFIX) {
        tags.push(name.replace(TAG_REF_PREFIX, ""));
      }
    }

    Ok(tags)
  }
}



pub fn fetch_all_tags(url: &str) -> Vec<String> {
  let mut tags = Vec::new();

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

