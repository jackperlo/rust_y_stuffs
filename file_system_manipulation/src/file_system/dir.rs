pub mod file;

use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use crate::file_system::dir::file::{CustomError, File, timestamp_to_u64};
use std::time::UNIX_EPOCH;
use crate::file_system::{MatchResult, Queries};

pub enum Node {
    File(File),
    Dir(Dir),
}
impl Display for Node{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Node::Dir(dir) => write!(f, "{}", dir),
            Node::File(file) => write!(f, "{}", file)
        }
    }
}
impl PartialEq<Path> for Node {
    fn eq(&self, other: &Path) -> bool {
        match self {
            Node::Dir(dir) => Path::new(&dir.name) == other,
            Node::File(file) => Path::new(file.get_name()) == other
        }
    }
}
impl<'b> Node{
    pub fn search<'a>(&'b mut self, queries: &[Queries<'a>], mut result: MatchResult<'a>) -> MatchResult<'a> where 'b: 'a, {
        match self {
            Self::File(_) => {
                if let Some(q) = queries.iter().find(|q| q.matches(self)) {
                    result.queries.push(q.to_str());
                    result.nodes.push(self);
                }
            },
            Self::Dir(dir) => result = dir.search(queries, result)
        }
        result
    }
}


#[derive(Default)]
pub struct Dir {
    name: String,
    creation_time: u64,
    children: Vec<Node>,
}
impl Display for Dir{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res: std::fmt::Result;
        res = write!(f, "Dir: name={}, creation_time={}\n", self.name, self.creation_time);
        for child in self.children.iter() {
            res = write!(f, "{}", child);
        }

        res
    }
}
impl PartialEq<Path> for Dir {
    fn eq(&self, other: &Path) -> bool {
        Path::new(&self.name) == other
    }
}
impl<'b> Dir{
    pub fn new(path: &str) -> Result<Dir, CustomError>{
        let mut dir = Dir{
            name: path.to_string(),
            creation_time: 0,
            children: vec![]
        };
        let file_info = fs::metadata(path)?;
        dir.creation_time = file_info.created()?.duration_since(UNIX_EPOCH)?.as_secs();
        let dir_children = fs::read_dir(path)?;
        for child in dir_children {
            let child = child?;
            let child_metadata = child.metadata()?;
            if child_metadata.is_dir() {
                dir.children.push(Node::Dir(Dir::new(child.path().to_str().ok_or(CustomError::FileOrDirNameNotFound)?)?));
            } else if child_metadata.is_file() {
                dir.children.push(Node::File(File::new( child.path().to_str().ok_or(CustomError::FileOrDirNameNotFound)?.to_string(), child_metadata)?));
            } else {
                println!("Content not recognized: {:?}, type is {:?}", child.path(), child_metadata.file_type());
            }
        }
        Ok(dir)
    }
    pub fn new_from_dir(path: &Path, creation_time: u64) -> Result<Dir, CustomError>{
        let name = path.to_str().ok_or(CustomError::FileOrDirNameNotFound)?.to_string();
        Ok(Dir{
            name,
            creation_time,
            children: vec![]
        })
    }
    pub fn is_empty(&self) -> bool {self.children.len() == 0}
    pub fn mk_dir(&mut self, path: &Path) -> Result<(), CustomError>{
        if path.parent().unwrap().to_str().ok_or(CustomError::FileOrDirNameNotFound)? == &self.name{
            if self.children.iter().any(|child| child == path) {
                return Err(CustomError::DirOrFileAlreadyExists);
            }
            self.children.push( Node::Dir(Dir::new_from_dir(path, timestamp_to_u64(std::time::SystemTime::now())?)?));
        }else{
            for child in self.children.iter_mut() {
                match child {
                    Node::Dir(dir) => dir.mk_dir(&path)?,
                    Node::File(_file) => continue
                }
            }
        }
        Ok(())
    }
    pub fn rm_dir(&mut self, path: &Path) -> Result<(), CustomError>{
        if path.parent().unwrap().to_str().ok_or(CustomError::FileOrDirNameNotFound)? == &self.name{
            let mut index_to_remove: usize = 0;
            if !self.children
                .iter()
                .enumerate()
                .any(|(i, child)|
                    match child {
                        Node::Dir(dir) =>  {
                            let res = &dir.name == path.to_str().unwrap() && dir.is_empty();
                            if res {index_to_remove = i;}
                            res
                        },
                        Node::File(_file) => false
                    }
                ) { return Err(CustomError::FileOrDirNameNotFound); }
            self.children.remove(index_to_remove);
        }else{
            for child in self.children.iter_mut() {
                match child {
                    Node::Dir(dir) => dir.rm_dir(&path)?,
                    Node::File(_file) => continue
                }
            }
        }
        Ok(())
    }
    pub fn new_file(&mut self, path: &Path, file: &File) -> Result<(), CustomError>{
        if path.parent().unwrap().to_str().ok_or(CustomError::FileOrDirNameNotFound)? == &self.name{
            if self.children.iter().any(|child| child == path) {
                return Err(CustomError::DirOrFileAlreadyExists);
            }
            self.children.push( Node::File(File::new_from_file(path, file)?));
        }else{
            for child in self.children.iter_mut() {
                match child {
                    Node::Dir(dir) => dir.new_file(path, file)?,
                    Node::File(_file) => continue
                }
            }
        }
        Ok(())
    }
    pub fn rm_file(&mut self, path: &Path) -> Result<(), CustomError>{
        if path.parent().unwrap().to_str().ok_or(CustomError::FileOrDirNameNotFound)? == &self.name{
            if self.is_empty() {
                return Err(CustomError::FileNotFound);
            }
            let mut index_to_remove: usize = 0;
            if !self.children
                .iter()
                .enumerate()
                .any(|(i, child)|
                    match child {
                        Node::Dir(_dir) =>  false,
                        Node::File(file) => {
                            let res = file.get_name() == path.to_str().unwrap();
                            if res {index_to_remove = i;}
                            res
                        }
                    }
                ) { return Err(CustomError::FileOrDirNameNotFound); }
            self.children.remove(index_to_remove);
        }else{
            for child in self.children.iter_mut() {
                match child {
                    Node::Dir(dir) => dir.rm_file(path)?,
                    Node::File(_file) => continue
                }
            }
        }
        Ok(())
    }
    pub fn get_file(&mut self, path: &Path) -> Option<&mut File>{
        if path.parent().unwrap().to_str().ok_or(CustomError::FileOrDirNameNotFound).ok()? == &self.name{
            for child in self.children.iter_mut() {
                match child {
                    Node::Dir(_dir) =>  continue,
                    Node::File(file) => if file.get_name() == path.to_str().unwrap() { return Some(file); }
                }
            }
        }else{
            for child in self.children.iter_mut() {
                match child {
                    Node::Dir(dir) =>  return dir.get_file(path),
                    Node::File(_file) => continue
                }
            }
        }
        None
    }
    pub fn search<'a>(&'b mut self, queries: &[Queries<'a>], mut result: MatchResult<'a>) -> MatchResult<'a> where 'b: 'a, {
        for child in self.children.iter_mut() {
            result = child.search(queries, result)
        }
        result.queries.sort_unstable();
        result.queries.dedup();
        result
    }

    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_creation_time(&self) -> u64 { self.creation_time }
}
