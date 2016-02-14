use std::path::{PathBuf, Path};
use git2::{Repository, ErrorCode, ResetType};
use walkdir::{WalkDir, DirEntry, WalkDirIterator};
use krate::Crate;
use serde_json as json;
use error::ParseError;
use std::fs::File;

pub const CRATES_IO: &'static str = "https://github.com/rust-lang/crates.io-index";

pub struct Index {
    path: PathBuf
}

impl Index {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Index, ::git2::Error> {
        let index = Index {
            path: path.into()
        };
        
        try!(index.update());

        Ok(index)
    }

    pub fn update(&self) -> Result<(), ::git2::Error> {
        Repository::clone(CRATES_IO, &self.path).map(|_| ()).or_else(|e| {
            if e.code() == ErrorCode::Exists {
                let repo = try!(Repository::open(&self.path));
                let mut remote = try!(repo.find_remote("origin"));
                let result = remote.fetch(&["origin/master"], None, None);
                // let origin_master = repo.revparse_single("origin/master").unwrap();
                // repo.reset(&origin_master, ResetType::Hard, None).unwrap();
                result
            } else {
                Err(e)
            }  
        })
    }

    pub fn crates(&self) -> Box<Iterator<Item=Result<Crate, ParseError>>> {
        Box::new(
            WalkDir::new(&self.path)
            .into_iter()
            .filter_entry(|entry| !is_ignored(entry))
            .flat_map(|entry| entry)
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| Crate::from_path(entry.path()))
        )
    }

    pub fn config(&self) -> Result<Config, ParseError> {
        let file = try!(File::open(self.path.join("config.json")));
        let config = try!(json::from_reader(file));
        Ok(config)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn set_config(&mut self, config: &Config) -> Result<(), ParseError> {
        let ref mut file = try!(File::create(self.path.join("config.json")));
        try!(json::to_writer(file, config));
        Ok(())
    }
}

fn is_ignored(entry: &DirEntry) -> bool {
    let ignored = ["config.json", ".git"];
    entry
        .file_name()
        .to_str()
        .map(|name| ignored.contains(&name))
        .unwrap_or(false)
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    dl: String,
    api: String
}

impl Config {
    pub fn set_dl<S: Into<String>>(&mut self, url: S) -> &mut Self  {
        self.dl = url.into();
        self
    }

    pub fn set_api<S: Into<String>>(&mut self, url: S) -> &mut Self  {
        self.api = url.into();
        self
    }
}