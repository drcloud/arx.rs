use std::error::Error;
use std::fmt;

use yaml_rust::{ScanError, Yaml, YamlLoader, YamlEmitter};


pub fn load(manifest: &str) -> Result<Yaml, Box<Error>> {
    let mut docs = try!(YamlLoader::load_from_str(manifest));
    let doc = docs.pop().unwrap_or(Yaml::BadValue);
    match doc {
        Yaml::Hash(_) => Ok(doc),
        _ => Err(Box::new(NotADict)),
    }
}


#[derive(Debug)]
pub enum ParseError {
    NotADict,
}

use self::ParseError::*;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.description())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            NotADict => "Root YAML was not a dictionary.",
        }
    }
}
