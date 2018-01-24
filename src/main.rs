use std::env;
use std::io;
use std::io::ErrorKind::*;
use std::path::{Path,PathBuf};

extern crate rusty_leveldb;
use rusty_leveldb::{DB,Options,Status};
use rusty_leveldb as ldb;

fn main() {
}

fn gcache() -> ldb::Result<()> {
    let d = Dirs::new()?;

    let dirs = d.as_str()?;

    let mut db = DB::open(
        dirs.gcache_leveldb_dir,
        Options::default(),
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Dirs<P: AsRef<Path>> {
    home_dir: P,
    gcache_file_dir: P,
    gcache_leveldb_dir: P,
}


impl Dirs<PathBuf> {
    fn new() -> io::Result<Dirs<PathBuf>> {
        let home_dir = env::home_dir().ok_or(
            io::Error::new(NotFound, "Can’t find home directory.")
        )?;

        let mut gcache_file_dir = (&home_dir).clone();
        gcache_file_dir.push("GCache/v1/files");

        let mut gcache_leveldb_dir = (&home_dir).clone();
        gcache_leveldb_dir.push("GCache/v1/meta/resource_metadata_resource_map.db");

        //let tmp_leveldb_dir = PathBuf::from("/tmp/resource_metadata_resource_map.db")
        //let link_dir = (&home_dir).clone().push("gdrive-files");

        let dirs = Dirs{
            home_dir,
            gcache_file_dir,
            gcache_leveldb_dir,
        };

        Ok(dirs)
    }
}

impl<P> Dirs<P>
    where P: AsRef<Path>
{
    fn as_str<'a>(&'a self) -> io::Result<Dirs<&'a str>> {
        let home_dir = self.home_dir.as_ref().to_str().ok_or(
            io::Error::new(NotFound, "Can’t find home directory.")
        )?;

        let gcache_file_dir = self.gcache_file_dir.as_ref().to_str().ok_or(
            io::Error::new(NotFound, "Can’t find home directory.")
        )?;

        let gcache_leveldb_dir = self.gcache_leveldb_dir.as_ref().to_str().ok_or(
            io::Error::new(NotFound, "Can’t find home directory.")
        )?;

        let dirs = Dirs{
            home_dir,
            gcache_file_dir,
            gcache_leveldb_dir,
        };

        Ok(dirs)
    }
}
