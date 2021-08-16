use std::path::{Path, PathBuf};

use cargo_toml::Manifest;

pub struct Project {
    pub path: PathBuf,
    pub manifest: Manifest,
}

impl Project {
    #[inline]
    pub fn load(path: &Path) -> Result<Self, cargo_toml::Error> {
        let manifest = Manifest::from_path(path.join("Cargo.toml"))?;

        Ok(Self {
            path: path.into(),
            manifest,
        })
    }

    #[inline]
    pub fn verify_crate_type(&self) -> Result<(), ()> {
        let lib = self.manifest.lib.as_ref().ok_or(())?;
        let crate_type = lib.crate_type.as_ref().ok_or(())?;

        if crate_type.iter().find(|ty| *ty == "cdylib").is_some() {
            Ok(())
        } else {
            Err(())
        }
    }
}
