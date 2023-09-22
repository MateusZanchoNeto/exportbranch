use crate::help::help;
use std::{
    path::Path,
    result::Result::{Err, Ok},
};

const DEFAULT_ONLY_COPY_FILES: [&str; 5] = ["*.a", "*.so", "*.h", "*.0", "*.18"];

const DEFAULT_FILTERS: [&str; 19] = [
    "*.prg", "*.mke", "*.mkp", "*.mks", "*.mkc", "*.hbp", "*.hbc", "*.hbm", "*.ch", "*.so*",
    "*.cpp", "*.a", "*.c", "*.h", "*.sh", "*.0", "*.18", "*.jar", "*.spec",
];

const DISREGARDED_DIRECTORIES: [&str; 5] = [
    "bin",
    "lib",
    "new/fivewin",
    "programas_externos/conversoes",
    "programas_externos/hbfunctions",
];

pub struct Configuration {
    source: Vec<String>,
    destination: Vec<String>,
    only_copy_files: Vec<String>,
    file_filters: Vec<String>,
    show: bool,
    md5: bool,
    reload: bool,
    lower: bool,
    disregarded_directories: Vec<String>,
}

impl Configuration {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<Configuration, String> {
        let mut destination = Vec::new();
        let mut source = Vec::new();
        let mut md5 = false;
        let mut reload = false;
        let mut lower = false;
        let mut show = false;
        let mut only_copy_files = vec![];
        let mut file_filters = vec![];
        let mut previous_arg = String::new();

        args.next();

        for arg in args {
            if arg == "-d" || arg == "-s" || arg == "-c" || arg == "-f" {
                previous_arg = arg;
                continue;
            } else if previous_arg == "-d" {
                for path in arg.split(";") {
                    destination.push(path.to_string());
                }
            } else if previous_arg == "-s" {
                for path in arg.split(";") {
                    source.push(path.to_string());
                }
            } else if previous_arg == "-c" {
                only_copy_files = Vec::new();
                for file in arg.split(";") {
                    only_copy_files.push(file.to_string());
                }
            } else if previous_arg == "-f" {
                file_filters = Vec::new();
                for file in arg.split(";") {
                    file_filters.push(file.to_string());
                }
            } else if arg == "--md5" {
                md5 = true;
            } else if arg == "--reload" {
                reload = true;
            } else if arg == "--lower" {
                lower = true;
            } else if arg == "--show" {
                show = true;
            } else {
                return Err(help());
            }
            previous_arg = String::new();
        }

        destination = destination.into_iter().filter(|x| x.len() > 0).collect::<Vec<String>>();
        source = source.into_iter().filter(|x| x.len() > 0).collect::<Vec<String>>();

        if destination.len() < 1 || source.len() < 1 {
            return Err(help());
        }

        let mut disregarded_directories = Vec::new();

        for source_directory in &source {
            for disregarded_directory in DISREGARDED_DIRECTORIES {
                let source_path = Path::new(&source_directory).canonicalize().unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
                disregarded_directories
                    .push(Path::new(&source_path).parent().unwrap().join(disregarded_directory).to_str().unwrap().to_string());
            }
        }

        Ok(Configuration {
            source,
            destination,
            only_copy_files: {
                if only_copy_files.len() < 1 {
                    DEFAULT_ONLY_COPY_FILES.map(|x| x.to_string()).to_vec()
                } else {
                    only_copy_files
                }
            },
            file_filters: {
                if file_filters.len() < 1 {
                    DEFAULT_FILTERS.map(|x| x.to_string()).to_vec()
                } else {
                    file_filters
                }
            },
            md5,
            reload,
            lower,
            disregarded_directories,
            show,
        })
    }

    pub fn print(&self) {
        if self.show {
            println!(
                "Export Branch\r\nsource.........: {:?}\r\ndestination....: {:?}\r\nonly_copy_files: {:?}\r\nfile_filters...: {:?}\r\nmd5............: {:?}\r\nreload.........: {:?}\r\nlower..........: {:?}\r\ndisregarded....: {:?}\r\n",
                self.source,
                self.destination,
                self.only_copy_files,
                self.file_filters,
                self.md5,
                self.reload,
                self.lower,
                self.disregarded_directories,
            );
        }
        println!("--------------------------\r\nExporting...\r\n");
    }

    pub fn source(&self) -> &Vec<String> {
        &self.source
    }

    pub fn destination(&self) -> &Vec<String> {
        &self.destination
    }

    pub fn only_copy_files(&self) -> &Vec<String> {
        &self.only_copy_files
    }

    pub fn file_filters(&self) -> &Vec<String> {
        &self.file_filters
    }

    pub fn md5(&self) -> bool {
        self.md5
    }

    pub fn reload(&self) -> bool {
        self.reload
    }

    pub fn lower(&self) -> bool {
        self.lower
    }

    pub fn disregarded_directories(&self) -> &Vec<String> {
        &self.disregarded_directories
    }
}
