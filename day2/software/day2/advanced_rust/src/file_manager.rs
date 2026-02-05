use std::fmt;
use std::fs;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum Errors {
    FileNotFound,
    PermissionDenied,
    InvalidPath,
    GeneralErrors(String),
}

pub struct FileManager;

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::FileNotFound => write!(f, "File not found"),
            Errors::PermissionDenied => write!(f, "Permission denied"),
            Errors::InvalidPath => write!(f, "Invalid path"),
            Errors::GeneralErrors(err)=> write!(f, "Error ! {}", err),
        }
    }
}


impl FileManager {
    pub fn list_files(path: &str) -> Result<Vec<String>, Errors>{

        let listing = fs::read_dir(path);

        match listing {
            Ok(list) =>{
                let mut files= Vec::new();
                for elements in list {
                    if let Ok(element) = elements{
                        let is_file = element.file_type().map_err(|e| Errors::GeneralErrors(e.to_string()))?;
                        if is_file.is_file(){
                            let files_name = element.file_name().to_string_lossy().into_owned();
                            files.push(files_name);
                        }
                    }
                }
                Ok(files)
            }
            Err(e) => {
                let err = match e.kind() {
                    ErrorKind::NotFound => Errors::FileNotFound,
                    ErrorKind::PermissionDenied => Errors::PermissionDenied,
                    ErrorKind::InvalidInput => Errors::InvalidPath,
                    _ => Errors::GeneralErrors(e.to_string()),
                };
            Err(err)
            }
        }
    }


    pub fn read_file(path: &str)-> Result<String, Errors>{

        let content = fs::read_to_string(path);

        match content {
            Ok(content) => Ok(content),
            Err(e) => {let err= match e.kind() {
                ErrorKind::NotFound => Errors::FileNotFound,
                ErrorKind::PermissionDenied => Errors::PermissionDenied,
                ErrorKind::InvalidInput => Errors::InvalidPath,
                _ => Errors::GeneralErrors(e.to_string()),
                };
                Err(err)
            }
        }
    }


    pub fn write_file(path: &str, content: &str) -> Result<(), Errors> {

        let writing = fs::write(path, content);

        match writing {
            Ok(_) => Ok(()),
            Err(e) => { let err = match e.kind() {
                ErrorKind::NotFound => Errors::FileNotFound,
                ErrorKind::PermissionDenied => Errors::PermissionDenied,
                ErrorKind::InvalidInput => Errors::InvalidPath,
                _ => Errors::GeneralErrors(e.to_string()),
            };
            Err(err)
            }
        }
    }

    pub fn copy_file(src: &str, dest: &str) -> Result<(), Errors>{

        let copy = fs::copy(src, dest);

        match copy {
            Ok(_) => Ok(()),
            Err(e) => { let err = match e.kind() {
                ErrorKind::NotFound => Errors::FileNotFound,
                ErrorKind::PermissionDenied => Errors::PermissionDenied,
                ErrorKind::InvalidInput => Errors::InvalidPath,
                _ => Errors::GeneralErrors(e.to_string()),
            };
            Err(err)
            }
        }
    }
}