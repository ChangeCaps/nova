use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub struct ProjectPath(pub PathBuf);

impl ProjectPath {
    #[inline]
    pub fn dir(&self) -> &Path {
        self.0.parent().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildSettings {
    pub manifest_path: PathBuf,
    pub target_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MainScene {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Project {
    #[serde(skip)]
    pub modified: Option<SystemTime>,
    pub package: Package,
    pub build: BuildSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_scene: Option<MainScene>,
}

impl Default for Project {
    #[inline]
    fn default() -> Self {
        Self {
            modified: None,
            package: Package {
                name: String::from("nova-game"),
            },
            build: BuildSettings {
                manifest_path: PathBuf::from("Cargo.toml"),
                target_dir: PathBuf::from("target"),
            },
            main_scene: None,
        }
    }
}

impl Project {
    #[inline]
    pub fn load(path: &Path) -> Result<Option<Self>, toml::de::Error> {
        let project_str = match read_to_string(path).ok() {
            Some(project_str) => project_str,
            None => return Ok(None),
        };

        Ok(Some(toml::from_str(&project_str)?))
    }

    #[inline]
    pub fn update(&mut self, path: &Path) -> bool {
        if let Err(e) = self.try_update(path) {
            log::error!("failed to update project: {}", e);

            false
        } else {
            true
        }
    }

    #[inline]
    pub fn try_update(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let meta = path.metadata()?;

        if let Some(modified) = &self.modified {
            if *modified > meta.modified()? {
                *self = Self::load(path)?.unwrap();
            }
        } else {
            *self = Self::load(path)?.unwrap();
        }

        Ok(())
    }

    #[inline]
    pub fn write(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut project_str = String::new();
        let mut serializer = toml::Serializer::new(&mut project_str);
        self.serialize(&mut serializer)?;
        std::fs::write(path, project_str)?;

        Ok(())
    }
}
