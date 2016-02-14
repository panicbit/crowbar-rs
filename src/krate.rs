use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use serde_json as json;
use error::ParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Crate {
    name: String,
    versions: Vec<Version>
}

impl Crate {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Crate, ParseError> {
        let path = path.as_ref();
        let file = try!(File::open(&path));

        println!("{:?}", path);

        let versions: Result<Vec<Version>, ParseError> = BufReader::new(file)
            .lines()
            .map(|line| {
                let line = try!(line);
                let json = try!(json::from_str(line.trim()));
                Ok(json)
            })
            .collect();

        Ok(Crate {
            name: path.file_name().unwrap().to_str().expect("file_name").into(),
            versions: try!(versions)
        })
    }

    pub fn latest(&self) -> &Version {
        self.versions.iter().max_by_key(|version| version.semver()).expect("some version")
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Version {
    name: String,
    vers: String,
    deps: Vec<Dependency>,
    cksum: String,
    features: HashMap<String, Vec<String>>,
    yanked: bool,
}

impl Version {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.vers
    }

    pub fn semver(&self) -> ::semver::Version {
        ::semver::Version::parse(self.version()).expect("semver")
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Dependency {
    name: String,
    req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: Option<String>
}
