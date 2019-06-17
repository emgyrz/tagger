use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path,PathBuf};
use serde::Deserialize;

use super::err::TaggerError;

#[derive(Deserialize, Debug)]
pub struct Repo {
  pub name: String,
  pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Cfg {
  pub repos: Vec<Repo>,
  pub command: Option<String>,
}

pub fn read_cfg(cfg_path: Option<String>) -> Result<Cfg, TaggerError> {
  let home_dir = dirs::home_dir().unwrap_or_default();
  let cwd_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".") );
  let cfg_default_name = ".tagger.cfg.json";

  let default_paths = vec![
    Path::new(&cwd_dir).join( &cfg_default_name ),
    Path::new(&home_dir).join( &cfg_default_name ),
  ];


  let path = if let Some(p_str) = cfg_path {
    let p = Path::new(&p_str);
    if p.exists() {
      p.to_path_buf()
    } else {
      return Err(TaggerError::new(
        &format!("There is no tagger config in path {}", p_str)
       ))
    }
  } else {
    let def = default_paths.iter().find(|p| p.exists() );

    if let Some(p) = def {
      p.clone()
    } else {
      let mut err_str = String::from(
        format!("Cannot find tagger cfg in default pathes {:?}\n", default_paths)
      );

      err_str.push_str("Please specify path to config, for example: \n");
      err_str.push_str(r"tagger --cfg C:\Users\Alice\.cfg.json --show-latest my_pkg");
      return Err( TaggerError::new(&err_str));
    }
  };

  // let path = cfg_path.unwrap_or_else(|| Path::new(".tagger.cfg.json"));

  let file = match File::open(path) {
    Ok(f) => f,
    Err(e) => {
      return Err(TaggerError::new(e.description()));
    }
  };

  let reader = BufReader::new(file);
  let cfg: Cfg = match serde_json::from_reader(reader) {
    Ok(c) => c,
    Err(e) => {
      return Err(TaggerError::new(e.description()));
    }
  };

  Ok(cfg)
}
