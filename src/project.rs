use {
    crate::*,
    std::{
        fs,
        io::Read,
        path::PathBuf,
    },
};

fn env_dir(name: &str) -> anyhow::Result<PathBuf> {
    let path = std::env::var(name).map_err(|_| anyhow::anyhow!("{} env var not set", name))?;
    let dir = PathBuf::from(path);
    if !dir.exists() || !dir.is_dir() {
        anyhow::bail!("{} is not a directory", dir.display());
    }
    Ok(dir)
}

#[derive(Debug)]
pub struct Project {
    /// The "miaou" root directory.
    pub root_dir: PathBuf,
    /// Where miaou si built. It's normally the same as root_dir but
    /// can be changed.
    pub build_dir: PathBuf,
}

impl Project {
    pub fn new() -> anyhow::Result<Self> {
        let root_dir = env_dir("MIAOU_ROOT")?;
        let build_dir = root_dir.clone();
        Ok(Self {
            root_dir,
            build_dir,
        })
    }
    pub fn list_dirs(
        &self,
        pattern: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
        files::list_paths_in(pattern, &self.root_dir, Some(files::FileKind::Dir))
    }
    pub fn list_files(
        &self,
        pattern: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
        files::list_paths_in(pattern, &self.root_dir, Some(files::FileKind::File))
    }
    pub fn list_paths(
        &self,
        pattern: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
        files::list_paths_in(pattern, &self.root_dir, None)
    }
    pub fn read_files_to_string(
        &self,
        pattern: &str,
        content: &mut String,
    ) -> anyhow::Result<()> {
        for path in self.list_files(pattern)? {
            let mut file = fs::File::open(&path)?;
            file.read_to_string(content)?;
        }
        Ok(())
    }
}
