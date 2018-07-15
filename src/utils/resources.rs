use std::env;
use std::path::PathBuf;

pub fn path() -> PathBuf {
  let mut path = env::current_exe().unwrap();
  path.pop();
  path.push("resources");

  path
}
