use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct TaggerError {
  msg: String,
}

impl fmt::Display for TaggerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", &self.msg)
  }
}

impl StdError for TaggerError {
  fn description(&self) -> &str {
    &self.msg
  }
}


impl TaggerError {
  pub fn new(msg: &str) -> TaggerError {
    TaggerError { msg: msg.to_owned() }
  }

  pub fn exit(self) -> ! {
    println!("{}", self.msg);
    std::process::exit(1);
  }
}
