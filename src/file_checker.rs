use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const EXPORT_BRANCH_FILES_METADATA: &str = "export_branch_files_metadata.txt";

pub enum FileStatus {
    UpToDate,
    Modified(SystemTime),
}

pub struct FileChecker {
    directory: PathBuf,
    files: HashMap<String, String>,
}

impl FileChecker {
    pub fn new(directory: PathBuf) -> FileChecker {
        match FileChecker::read_file(&directory) {
            Ok(contents) => FileChecker::build(directory, contents),
            _ => FileChecker::default(directory),
        }
    }

    pub fn check(&mut self, file: &PathBuf) -> FileStatus {
        match FileChecker::get_modified(file) {
            Ok(system_time) => match self.files.get(file.to_str().unwrap()) {
                Some(file_modified) => {
                    if *file_modified == format!("{:?}", system_time) {
                        FileStatus::UpToDate
                    } else {
                        FileStatus::Modified(system_time)
                    }
                }

                _ => FileStatus::Modified(system_time),
            },

            _ => FileStatus::Modified(SystemTime::now()),
        }
    }

    pub fn save(&self) -> Result<()> {
        let mut file = FileChecker::get_file(&self.directory)?;
        let mut contents = String::new();

        for (file_name, file_metadata) in &self.files {
            contents.push_str(&format!("{};{}\n", file_name, file_metadata));
        }
        file.write_all(contents.as_bytes())
    }

    pub fn add_file(&mut self, file: &PathBuf, system_time: SystemTime) {
        self.files.insert(
            file.to_str().unwrap().to_string(),
            format!("{:?}", system_time),
        );
    }

    pub fn remove_file(&mut self, file: &PathBuf) {
        self.files.remove(&file.to_str().unwrap().to_string());
    }

    pub fn force_update(&mut self, file: &PathBuf) -> FileStatus {
        match FileChecker::get_modified(file) {
            Ok(system_time) => FileStatus::Modified(system_time),
            _ => FileStatus::Modified(SystemTime::now()),
        }
    }
}

impl FileChecker {
    fn default(directory: PathBuf) -> FileChecker {
        FileChecker {
            directory: directory,
            files: HashMap::new(),
        }
    }

    fn read_file(directory: &PathBuf) -> Result<String> {
        let mut file = FileChecker::get_file(directory)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn get_file(directory: &PathBuf) -> Result<File> {
        let file = Path::new(directory).join(EXPORT_BRANCH_FILES_METADATA);

        if !file.exists() {
            return File::create(file);
        }

        File::options().read(true).write(true).open(file)
    }

    fn build(directory: PathBuf, contents: String) -> FileChecker {
        let mut files = HashMap::new();

        for line in contents.lines() {
            let mut parts = line.split(";");
            if let Some(file_name) = parts.next() {
                if let Some(file_metadata) = parts.next() {
                    files.insert(file_name.to_string(), file_metadata.to_string());
                }
            };
        }
        FileChecker { directory, files }
    }

    fn get_modified(file: &PathBuf) -> Result<SystemTime> {
        let metadata = file.metadata()?;
        let modified = metadata.modified()?;
        Ok(modified)
    }
}
