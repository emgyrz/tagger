use git2::{Direction, RemoteCallbacks, RemoteHead, Repository};
use tempdir::TempDir;


use super::err::TaggerError;
use super::pkg::Pkg;

static ORIGIN: &str = "origin";
static TAG_REF_PREFIX: &str = "refs/tags/";

type TaggerResult<T> = std::result::Result<T, TaggerError>;

pub struct Tags<'a> {
  pkg: &'a Pkg,
  tags: Option<Vec<String>>,
  is_fetched: bool,
}


impl<'a> Tags<'a> {
  pub fn new(pkg: &Pkg) -> Tags {
    Tags {
      pkg,
      tags: None,
      is_fetched: false,
    }
  }

  fn get_tmp_dir(&self) -> TaggerResult<TempDir> {
    let tmp_dir_name = format!("tagger__{}", self.pkg.name);
    TempDir::new(&tmp_dir_name).or_else(|_| Err(TaggerError::new("cannot create temp directory")))
  }

  fn get_repo(&self, dir: &TempDir) -> TaggerResult<Repository> {
    Repository::open_bare(dir.path()).or_else(|_| {
      Repository::init_bare(dir.path())
        .or_else(|_| Err(TaggerError::new("cannot create temp directory")))
    })
  }

  fn get_remote_callbacks<'b>() -> RemoteCallbacks<'b> {
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


  fn filter_tags(&self, heads: &[RemoteHead]) -> TaggerResult<Vec<String>> {
    let mut tags = Vec::new();

    for ref_name in heads {
      let name = ref_name.name();
      if name.starts_with(TAG_REF_PREFIX) {
        let tag = name.replace(TAG_REF_PREFIX, "");
        if semver::Version::parse(&tag).is_ok() {
          tags.push(tag);
        }
      }
    }

    if tags.is_empty() {
      return Err(TaggerError::new(&format!(
        "repo {} does not have any valid tags",
        &self.pkg.name
      )));
    }

    Ok(tags)
  }

  pub fn fetch_all(&mut self) -> TaggerResult<()> {
    let tmp_dir = self.get_tmp_dir()?;

    let repo = self.get_repo(&tmp_dir)?;

    let mut remote = repo.find_remote(ORIGIN).or_else(|_| {
      repo.remote(ORIGIN, &self.pkg.url).or_else(|_| {
        Err(TaggerError::new(
          "cannot find or create remote with name origin",
        ))
      })
    })?;

    let conn = (&mut remote)
      .connect_auth(Direction::Fetch, Some(Tags::get_remote_callbacks()), None)
      .or_else(|e| {
        Err(TaggerError::new(&format!(
          "cannot create temp directory\n{}",
          e.message()
        )))
      })?;

    let ls = conn.list().or_else(|_| {
      Err(TaggerError::new(
        "cannot get remote repository referenses list",
      ))
    })?;

    let tags = self.filter_tags(ls)?;
    self.tags = Some(tags);
    self.is_fetched = true;

    Ok(())
  }

  pub fn print_all(&mut self) -> TaggerResult<()> {
    if !self.is_fetched {
      self.fetch_all()?;
    }

    let tags = self.tags.as_ref().unwrap();

    let mut s = format!("\nValid tags available in repo {} :\n[\n", self.pkg.name);
    let tags_str = tags
      .iter()
      .map(|tag| format!("    {}\n", tag))
      .collect::<Vec<String>>()
      .join("");

    s.push_str(&tags_str);
    s.push(']');
    println!("{}", s);

    Ok(())
  }


  pub fn get_latest(&mut self) -> TaggerResult<String> {
    if !self.is_fetched {
      self.fetch_all()?;
    }

    let tags = self.tags.as_ref().unwrap();

    let latest_version = tags
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
      Ok(ver.0.to_owned())
    } else {
      Err(TaggerError::new(""))
    }
  }

  pub fn print_latest(&mut self) -> TaggerResult<()> {
    let latest = self.get_latest()?;
    println!(
      "\nLatest valid tag for repo `{}`: {}",
      &self.pkg.name, &latest
    );
    Ok(())
  }

}

