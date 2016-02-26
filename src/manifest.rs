use std::ffi::CString;
use std::path::Path;
use std::vec::Vec;
use std::collections::BTreeMap;

use sources::*;


pub struct Manifest {
    pwd: Option<CString>,
    env: BTreeMap<CString, Option<CString>>,
    code: Vec<Code>,
    data: Vec<Data>,
    label: Option<CString>,
}

pub struct Code {
    cmd: Cmd,
    args: Vec<CString>,
}

pub struct Data {
    destination: Option<CString>,
    source: Box<Source>,
}

/// Kinds of program:
/// * Simple shell word
/// * A reference to a file source.
pub enum Cmd {
    Word(CString),
    Source(Box<Source>),
}
