use {
    crate::*,
    lazy_regex::*,
    std::{
        fs,
        io::Read,
        path::{
            Path,
            PathBuf,
        },
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    File,
    Dir,
}

pub fn list_paths_in(
    pattern: &str,
    parent: &Path,
    file_kind: Option<FileKind>,
) -> anyhow::Result<Vec<PathBuf>> {
    if !pattern.starts_with('/') {
        anyhow::bail!("pattern must start with /");
    }
    let absolute_pattern = format!("{}{}", parent.display(), pattern);
    let mut paths = Vec::new();
    for entry in glob::glob(&absolute_pattern)? {
        let entry = entry?;
        if let Some(file_kind) = file_kind {
            match file_kind {
                FileKind::File => {
                    if !entry.is_file() {
                        continue;
                    }
                }
                FileKind::Dir => {
                    if !entry.is_dir() {
                        continue;
                    }
                }
            }
        }
        paths.push(entry);
    }
    Ok(paths)
}

pub fn stem_of(path: &Path) -> anyhow::Result<String> {
    let file_name = path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("file_name not found"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("invalid file name"))?;
    let (_, stem) = regex_captures!(r"^(.*)(?:\.[^\.]+)?$", file_name).unwrap();
    let stem = path
        .file_stem()
        .and_then(|os_str| os_str.to_str())
        .ok_or_else(|| anyhow::anyhow!("file_stem not found"))?
        .to_string();
    Ok(stem)
}

pub fn name_of(path: &Path) -> anyhow::Result<String> {
    let file_name = path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("file_name not found"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("invalid file name"))?;
    Ok(file_name.to_string())
}

pub fn read_files_to_string(
    pattern: &str,
    parent: &Path,
    content: &mut String,
) -> anyhow::Result<()> {
    for path in list_paths_in(pattern, parent, Some(FileKind::File))? {
        read_file_to_string(&path, content)?;
    }
    Ok(())
}
pub fn read_file_to_string(
    path: &Path,
    content: &mut String,
) -> anyhow::Result<()> {
    let mut file = fs::File::open(&path)?;
    file.read_to_string(content)?;
    Ok(())
}

/// Produce a destination path by replacing the extension of the source path
/// and moving it to a new parent directory.
///
/// Replace only the "last" extension, ie "rooms.mob.scss"
///  becomes "rooms.mob.css" when new_extension is "css".
pub fn dest_path<P1, P2>(
    src_path: P1,
    parent: P2,
    new_extension: &str,
) -> anyhow::Result<PathBuf>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let file_name = src_path
        .as_ref()
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("file_name not found"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("invalid file name"))?;
    let file_name = regex_replace!(r"[^\.]+$", file_name, new_extension).to_string();
    let dest_path = parent.as_ref().join(file_name);
    Ok(dest_path)
}

pub fn write<P: AsRef<Path>>(
    path: P,
    content: &str,
) -> anyhow::Result<()> {
    fs::create_dir_all(path.as_ref().parent().unwrap())?;
    fs::write(path, content)?;
    Ok(())
}

#[test]
fn test_stem() {
    let path = PathBuf::from("/a/b/rooms.mob.scss");
    assert_eq!(stem_of(&path).unwrap(), "rooms.mob");
}
