use crate::configuration::Configuration;
use crate::export::export;
use crate::export_branch_files::checked_to_regex;
use crate::file_checker::FileChecker;
use std::path::PathBuf;

pub struct ExportBranch<'a> {
    pub source: Box<PathBuf>,
    pub destination: Box<PathBuf>,
    pub configuration: &'a Configuration,
    pub file_checker: &'a mut FileChecker,
}

impl<'a> ExportBranch<'a> {
    pub fn build(
        source: PathBuf,
        destination: PathBuf,
        configuration: &'a Configuration,
        file_checker: &'a mut FileChecker,
    ) -> ExportBranch<'a> {
        ExportBranch {
            source: Box::new(source),
            destination: Box::new(destination.clone()),
            configuration,
            file_checker,
        }
    }

    pub fn perform_exporting(&mut self) {
        let file_filters: Vec<String> = self.configuration.file_filters().clone();
        let only_copy_files: Vec<String> = self.configuration.only_copy_files().clone();

        if let Err(err) = export(
            self,
            *(self.source.clone()),
            *(self.destination.clone()),
            &checked_to_regex(file_filters),
            &checked_to_regex(only_copy_files),
        ) {
            eprintln!("{}", err);
            std::process::exit(1);
        }

        if let Err(err) = self.file_checker.save() {
            eprintln!("{}", err);
            std::process::exit(1);
        };
    }
}
