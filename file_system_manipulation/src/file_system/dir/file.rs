use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Debug)]
pub enum CustomError {
    FileOrDirNameNotFound,
    DirOrFileAlreadyExists,
    FileNotFound,
    InvalidQuery,
    IoError(std::io::Error),
    SystemTimeError(std::time::SystemTimeError),
}
impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::FileOrDirNameNotFound => write!(f, "Content name not found"),
            CustomError::DirOrFileAlreadyExists => write!(f, "Content already exists. Cannot replicate it."),
            CustomError::FileNotFound => write!(f, "Directory empty. Cannot remove file which does not exist"),
            CustomError::InvalidQuery => write!(f, "Invalid query. Cannot understand which query to select"),
            CustomError::IoError(e) => write!(f, "I/O error: {}", e),
            CustomError::SystemTimeError(e) => write!(f, "System time error: {}", e),
        }
    }
}
impl std::error::Error for CustomError {}
impl From<std::io::Error> for CustomError {
    fn from(e: std::io::Error) -> Self {
        CustomError::IoError(e)
    }
}
impl From<std::time::SystemTimeError> for CustomError {
    fn from(e: std::time::SystemTimeError) -> Self {
        CustomError::SystemTimeError(e)
    }
}

pub fn timestamp_to_u64(time: std::time::SystemTime) -> Result<u64, std::time::SystemTimeError> {
    Ok(time.duration_since(UNIX_EPOCH)?.as_secs())
}

#[derive(PartialEq)]
pub enum FileType {
    Text, Binary
}
impl Default for FileType {
    fn default() -> Self { FileType::Text }
}

#[derive(Default)]
pub struct File {
    name: String,
    content: Vec<u8>, // max 1000 bytes, rest of the file truncated
    creation_time: u64,
    type_: FileType,
}
impl Display for File{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.type_ {
            FileType::Binary =>  write!(f, "File: name={}, content={:?}, creation_time={}, type=.bin\n", self.name, self.content, self.creation_time),
            FileType::Text => write!(f, "File: name={}, content={:?}, creation_time={}, type=.txt\n", self.name, self.content, self.creation_time)
        }
    }
}
impl PartialEq<Path> for File{
    fn eq(&self, other: &Path) -> bool {
        Path::new(&self.name) == other
    }
}
impl File {
    pub fn new(name: String, metadata: fs::Metadata) -> Result<File, CustomError>{
        let mut content = vec![];
        let file = OpenOptions::new().read(true).open(&name)?;
        let mut reader = BufReader::new(file.take(1000));
        let path = Path::new(&name);
        let extension = match path.extension() {
            Some(ext) => ext.to_str().ok_or(CustomError::FileOrDirNameNotFound)?,
            None => "",
        };
        let type_ = match extension {
            "txt" | "md" | "rs" | "py" | "js" | "html" | "css" | "json" | "toml" | "yaml" | "yml" => FileType::Text,
            _ => FileType::Binary,
        };
        reader.read_to_end(&mut content)?;
        Ok(File {
            name,
            content,
            creation_time: timestamp_to_u64(metadata.created()?)?,
            type_,
        })
    }
    pub fn new_from_file(path: &Path, file: &File) -> Result<File, CustomError> {
        let name = path
            .to_str()
            .ok_or(CustomError::FileOrDirNameNotFound)?
            .to_string();
        Ok(File {
            name,
            content: file.content.clone(),
            creation_time: file.creation_time,
            type_: {
                match file.type_ {
                    FileType::Binary => FileType::Binary,
                    FileType::Text => FileType::Text
                }
            },
        })
    }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_filetype(&self) -> &FileType { &self.type_ }
    pub fn get_content(&self) -> &Vec<u8> { &self.content }
    pub fn get_creation_time(&self) -> u64 { self.creation_time }
    pub fn set_name(&mut self, name: String) { self.name = name; }
    pub fn set_content(&mut self, content: Vec<u8>) { self.content = content; }
    pub fn set_creation_time(&mut self, creation_time: u64) { self.creation_time = creation_time; }
    pub fn set_type_(&mut self, type_: FileType) { self.type_ = type_; }
}
