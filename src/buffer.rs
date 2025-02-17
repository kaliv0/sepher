use crate::line::Line;
use std::fmt;
use std::fmt::Display;
use std::fs::read_to_string;
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Buffer {
    lines: Vec<Line>,
    file_info: FileInfo, //TODO: simplify to underlying path property? -> as string
    dirty: bool,         //TODO: probably should rename it to 'modified'
}

impl Buffer {
    //TODO: is this java and we cannot do without getters?
    pub const fn get_file_info(&self) -> &FileInfo {
        &self.file_info
    }

    //TODO: rename to 'is_modified'
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    // TODO: override ::from for Line and FileInfo
    pub fn load(file_name: &str) -> Result<Self, Error> {
        // TODO: re-write using 'streams'
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self {
            lines,
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
    }
}

/////////////
#[derive(Default, Debug)]
pub struct FileInfo {
    path: Option<PathBuf>,
    //TODO: we ditched file_type_info
}

impl FileInfo {
    fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .get_path()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(formatter, "{name}")
    }
}
