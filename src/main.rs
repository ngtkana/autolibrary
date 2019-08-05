#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;

use serde_json::map::Map;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

use serde_json::de::from_reader;
use serde_json::ser::to_writer_pretty;
use std::fs::read_dir;

#[derive(Debug)]
enum Error
{
  FFIError(std::ffi::OsString),
  IOError(std::io::Error),
  JsonError(serde_json::Error),
}

impl From<std::ffi::OsString> for Error {
  fn from(e: std::ffi::OsString) -> Self {
    Error::FFIError(e)
  }
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Error::IOError(e)
  }
}

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Error::JsonError(e)
  }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Config {
  output_file: String,
  input_dirs: Vec<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Snippet {
  prefix: String,
  body: Vec<String>,
}

fn read_config (path: &Path) -> Result<Vec<Config>> {
  let file = File::open(&path)?;
  let ret = from_reader(&file)?;
  Ok(ret)
}

fn file_name_from_path(path: &Path) -> Result<String> {
  let ret = path.file_name().unwrap().to_os_string().into_string()?;
  Ok(ret)
}

fn path_name_from_path(path: &Path) -> Result<String> {
  let ret = path.to_str().unwrap().to_string();
  Ok(ret)
}

fn for_each_file(path: &Path, f: &mut FnMut(&Path) -> Result<()>) -> Result<()> {
  if path.is_dir() {
    for child in read_dir(path)? {
      let child = child?;
      let child = child.path();
      for_each_file(&child, f)?;
    }
  } else {
    f(&path)?;
  }
  Ok(())
}

fn make_snippet(path: &Path) -> Result<Snippet> {
  let mut snippet = Snippet{
    prefix: file_name_from_path(&path)?,
    body: vec!(),
  };
  let file = File::open(&path)?;
  let reader = BufReader::new(&file);
  for line in reader.lines() {
    let line = line?;
    snippet.body.push(line);
  }
  Ok(snippet)
}

fn make_and_write_snippet_json(config: &Config) -> Result<()> {
  println!("making a json file for `{}`", config.output_file);
  let mut map = Map::new();

  let mut make_and_insert_snippet = |path: &Path| -> Result<()> {
    let snippet = make_snippet(&path)?;
    let snippet = json!(&snippet);
    map.insert(
      path_name_from_path(&path)?,
      snippet
    );
    Ok(())
  };

  for path in config.input_dirs.iter() {
    let path = Path::new(&path);
    for_each_file(&path, &mut make_and_insert_snippet)?;
  }

  let file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(&config.output_file)?;
  to_writer_pretty(&file, &map)?;
  Ok(())
}

fn main() -> Result<()> {
  let path = Path::new("config.json");
  let configs = read_config(&path)?;
  for config in configs.iter() {
    make_and_write_snippet_json(&config)?;
  }
  Ok(())
}
