use std::env;
use std::io;
use std::io::ErrorKind::*;
use std::path::PathBuf;

fn main() {
    let dirs = Dirs::new();
}

#[derive(Debug, Clone)]
struct Dirs {
    home_dir: PathBuf,
    gcache_file_dir: PathBuf,
    gcache_meta_dir: PathBuf,
}

impl Dirs {
    fn new() -> io::Result<Dirs> {
        let home_dir = env::home_dir().ok_or(
            io::Error::new(NotFound, "Can’t find home directory.")
        )?;

        let mut gcache_file_dir = (&home_dir).clone();
        gcache_file_dir.push("GCache/v1/files");

        let mut gcache_meta_dir = (&home_dir).clone();
        gcache_meta_dir.push("GCache/v1/meta/resource_metadata_resource_map.db");

        //let tmp_leveldb_dir = PathBuf::from("/tmp/resource_metadata_resource_map.db")
        //let link_dir = (&home_dir).clone().push("gdrive-files");

        let dirs = Dirs{
            home_dir,
            gcache_file_dir,
            gcache_meta_dir,
        };

        Ok(dirs)
    }
}
