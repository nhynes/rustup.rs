use crate::dist::manifestation::{CHANNEL_MANIFEST_SUFFIX, DIST_CHANNEL};
use crate::errors::Result;

use std::path::{Path, PathBuf};

const REL_MANIFEST_DIR: &str = "lib/rustlib";

#[derive(Clone, Debug)]
pub struct InstallPrefix {
    path: PathBuf,
}
impl InstallPrefix {
    pub fn from(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn abs_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.path.join(path)
    }
    pub fn manifest_dir(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push(REL_MANIFEST_DIR);
        path
    }
    pub fn manifest_file(&self, name: &str) -> PathBuf {
        let mut path = self.manifest_dir();
        path.push(name);
        path
    }
    pub fn rel_manifest_file(&self, name: &str) -> PathBuf {
        let mut path = PathBuf::from(REL_MANIFEST_DIR);
        path.push(name);
        path
    }
    pub fn extra_manifest_files(&self) -> Result<Vec<PathBuf>> {
        let mut manifest_paths = Vec::new();
        for dir_ent in crate::utils::utils::read_dir("manifest dir", &self.manifest_dir())? {
            let path = dir_ent?.path();
            match path.file_name().and_then(|fname| fname.to_str()) {
                Some(fname)
                    if fname.ends_with(CHANNEL_MANIFEST_SUFFIX)
                        && !fname.starts_with(DIST_CHANNEL) =>
                {
                    manifest_paths.push(path);
                }
                _ => continue,
            }
        }
        Ok(manifest_paths)
    }
}
