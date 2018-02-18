use std::path::PathBuf;
use std;

pub enum MediaResource {
    File(PathBuf)
}

impl MediaResource {
    pub fn size(&self) -> std::io::Result<u64> {
        match self {
            &MediaResource::File(ref path) => {
                let file = try!(std::fs::File::open(path));
                let metadata = try!(file.metadata());
                return Ok(metadata.len());
            }
        }
    }

    pub fn reference(&self) -> String {
        match self {
            &MediaResource::File(ref path) => {
                match path.to_str() {
                    Some(p) => p.to_owned(),
                    None => String::new() // should never happen
                }
            }
        }
    }
}
