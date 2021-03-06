use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use serde_json;

use atomic_file;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) secret_store_type: SecretStoreType,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub(crate) enum SecretStoreType {
    File,
    SecretService,
}

#[derive(Clone)]
pub(crate) struct ConfigFilePath(PathBuf);

impl ConfigFilePath {
    pub fn from_dir(dir: &Path) -> ConfigFilePath {
        ConfigFilePath(dir.join("config.json"))
    }

    pub fn get(&self) -> &Path {
        &self.0
    }
}

pub(crate) struct ConfigFile {
    config: Config,
    path: ConfigFilePath,
}

impl ConfigFile {
    pub fn create(path: ConfigFilePath, config: Config) -> io::Result<ConfigFile> {
        let config_file = ConfigFile { config, path };
        config_file.save()?;
        Ok(config_file)
    }

    pub fn load(path: ConfigFilePath) -> io::Result<Option<ConfigFile>> {
        match File::open(path.get()) {
            Ok(file) => serde_json::from_reader(file)
                .map_err(|e| e.into())
                .map(|config| Some(ConfigFile { config, path })),
            Err(ref err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn save(&self) -> io::Result<()> {
        atomic_file::overwrite(self.path(), move |writer| {
            serde_json::to_writer_pretty(writer, &self.config).map_err(|e| e.into())
        })
    }

    fn path(&self) -> &Path {
        self.path.get()
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;

    use self::tempdir::TempDir;

    #[test]
    fn load_with_not_existing_file_returns_none() {
        let temp_dir = TempDir::new("config_tests").unwrap();
        let file_path = ConfigFilePath::from_dir(temp_dir.path());

        assert!(ConfigFile::load(file_path).unwrap().is_none());
    }
}
