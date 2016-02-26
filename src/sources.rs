use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::vec::Vec;


/// A source can be:
/// * Prepared for extraction (by downloading or cloning, for example)
/// * Extracted to a new file system location,
/// * Associated with other sources (as when it has been inlined, for example)
pub trait Source {
    fn prepare(&self, &Path) -> Result<(), Box<Error>>;
    fn extract(&self, &Path, &Path) -> Result<(), Box<Error>>;
    fn formerly(&self) -> Option<&LinkSource>;
}

/// Some sources have a canonical URL.
pub trait LinkSource : Source {
    fn url(&self) -> &str;
}


pub struct InlineFile<'a> {
    data: Vec<u8>,
    formerly: Option<&'a LinkSource>
}

impl<'a> Source for InlineFile<'a> {
    fn prepare(&self, _cache: &Path) -> Result<(), Box<Error>> {
        Ok(())
    }

    fn extract(&self, _cache: &Path, target: &Path) -> Result<(), Box<Error>> {
        try!(fs::create_dir_all(target.parent().unwrap()));
        let mut f: File = try!(File::create(target));
        let ok = try!(f.write_all(&self.data));
        Ok(ok)
    }

    fn formerly(&self) -> Option<&LinkSource> {
        self.formerly
    }
}
