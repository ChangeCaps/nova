use std::path::PathBuf;

#[derive(Default)]
pub struct Scenes {
    pub loaded: Vec<PathBuf>,
    pub open: Option<PathBuf>,
}
