pub mod dir;

use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::path::{PathBuf};
use crate::file_system::dir::{Dir, Node};
use crate::file_system::dir::file::{CustomError, File, FileType};
use crate::file_system::dir::file::CustomError::{FileOrDirNameNotFound, InvalidQuery};

#[derive(Default)]
pub struct MatchResult<'a> {
    queries: Vec<&'a str>, // query matchate
    nodes: Vec<&'a mut Node>
}
impl Display for MatchResult<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("Matched queries: ");
        for query in self.queries.iter() {
            result.push_str(query);
            result.push_str(", ");
        }
        result.pop();
        result.pop();
        result.push_str("\nNodes found: ");
        for node in self.nodes.iter() {
            result.push_str(&format!("\n\t{}", node));
        }
        f.write_str(&result)
    }
}

pub enum Queries<'a>{
    Name(&'a str, &'a str),
    Content(&'a str, &'a str),
    Larger(&'a str, usize),
    Smaller(&'a str, usize),
    Newer(&'a str, u64),
    Older(&'a str, u64)
}
impl<'a> Queries<'a>{
    fn to_str(&self) -> &'a str {
        match self {
            Self::Name(string, _) => string,
            Self::Content(string, _) => string,
            Self::Larger(string, _) => string,
            Self::Smaller(string, _) => string,
            Self::Newer(string, _) => string,
            Self::Older(string, _) => string,
        }
    }
    pub fn matches(&self, node: &Node) -> bool {
        match node {
            Node::File(file) => self.match_for_file(file),
            Node::Dir(dir) => self.match_for_dir(&dir.borrow()),
        }
    }
    fn match_for_file(&self, file: &File) -> bool {
        match self {
            Queries::Name(_, name) => file.get_name().contains(name),
            Queries::Content(_, content) => {
                if *file.get_filetype() == FileType::Text {
                    let file_contents = match std::str::from_utf8(file.get_content()) {
                        Ok(content) => content,
                        Err(_) => return false,
                    };
                    file_contents.contains(content)
                } else {
                    false
                }
            }
            Queries::Larger(_, size) => file.get_content().len() > *size,
            Queries::Smaller(_, size) => file.get_content().len() < *size,
            Queries::Newer(_, time) => file.get_creation_time() > *time,
            Queries::Older(_, time) => file.get_creation_time() < *time,
        }
    }
    fn match_for_dir(&self, dir: &Dir) -> bool {
        match self {
            Queries::Name(_, name) => dir.get_name().contains(name),
            Queries::Content(_, _) => false,
            Queries::Larger(_, _) => false,
            Queries::Smaller(_, _) => false,
            Queries::Newer(_, time) => dir.get_creation_time() > *time,
            Queries::Older(_, time) => dir.get_creation_time() < *time,
        }
    }
}

pub struct FileSystem {
    root: Dir
}
impl Display for FileSystem{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileSystem: \n{}", self.root)
    }
}
#[cfg(target_os = "windows")]
impl<'b> FileSystem{
    pub fn new() -> Self{
        FileSystem{
            root: Dir::default()
        }
    }
    pub fn from_dir(path: &str) -> Result<FileSystem, CustomError>{
        let mut fs = FileSystem::new();
        fs.root = Dir::new(path)?;
        Ok(fs)
    }
    pub fn get_root(&self) -> &Dir {&self.root}
    pub fn mk_dir(&mut self, path: &str) -> Result<(), CustomError>{
        if self.root.is_empty() {
            self.root = Dir::new(path)?;
        }else{
            self.root.mk_dir(&PathBuf::from(path))?;
        }
        Ok(())
    }
    pub fn rm_dir(&mut self, path: &str) -> Result<(), CustomError>{
        if self.root.is_empty() {
            return Err(FileOrDirNameNotFound);
        }
        self.root.rm_dir(&PathBuf::from(path))?;
        Ok(())
    }
    pub fn new_file(&mut self, path: &str, file: File) -> Result<(), CustomError>{
        self.root.new_file(&PathBuf::from(&path), &file)?;
        Ok(())
    }
    pub fn rm_file(&mut self, path: &str) -> Result<(), CustomError>{
        if self.root.is_empty() {
            return Err(FileOrDirNameNotFound);
        }
        self.root.rm_file(&PathBuf::from(&path))?;
        Ok(())
    }
    pub fn get_file(&mut self, path: &str) -> Option<&mut File>{
        if self.root.is_empty() {
            return None;
        }
        self.root.get_file(&PathBuf::from(&path))
    }
    pub fn search<'a>(&'b mut self, queries: &[&'a str]) -> MatchResult<'a> where 'b: 'a, {
        let queries: Vec<Queries> = queries
            .iter()
            .map(|query_string| {
                let mut query = query_string.split(':');
                let query_type = query.next().ok_or(InvalidQuery)?;
                let query_value = query.next().ok_or(InvalidQuery)?;
                let enum_type_query = match query_type {
                    "name" => Queries::Name(query_string, query_value),
                    "content" => Queries::Content(query_string, query_value),
                    "larger" => {
                        let size = query_value
                            .parse::<usize>()
                            .map_err(|_| InvalidQuery)?;
                        Queries::Larger(query_string, size)
                    }
                    "smaller" => {
                        let size = query_value
                            .parse::<usize>()
                            .map_err(|_| InvalidQuery)?;
                        Queries::Smaller(query_string, size)
                    }
                    "newer" => {
                        let time = query_value
                            .parse::<u64>()
                            .map_err(|_| InvalidQuery)?;
                        Queries::Newer(query_string, time)
                    }
                    "older" => {
                        let time = query_value
                            .parse::<u64>()
                            .map_err(|_| InvalidQuery)?;
                        Queries::Older(query_string, time)
                    }
                    &_ => {
                        return Err(InvalidQuery);
                    }
                };
                Ok(enum_type_query)
            })
            .filter_map(|x| match x {
                Ok(q) => Some(q),
                Err(_) => None,
            })
            .collect();
        self.root.search(&queries, MatchResult::default())
    }
}
