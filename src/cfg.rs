use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path,PathBuf};
use serde::Deserialize;

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

pub fn read_cfg(cfg_path: Option<String>) -> Result<Cfg, Box<Error>> {
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
      println!("There is no tagger config in path {}", p_str);
      std::process::exit(1);
    }
  } else {
    let def = default_paths.iter().find(|p| p.exists() );

    if let Some(p) = def {
      p.clone()
    } else {
      println!("Cannot find tagger cfg in default pathes {:?}", default_paths);
      println!("Please specify path to config, for example:");
      println!(r"tagger --cfg C:\Users\Alice\.cfg.json --show-latest my_pkg");
      std::process::exit(1);
    }
  };

  // let path = cfg_path.unwrap_or_else(|| Path::new(".tagger.cfg.json"));

  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let cfg: Cfg = serde_json::from_reader(reader)?;

  Ok(cfg)
}
