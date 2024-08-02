use crate::convert_file::convert_file;
use crate::export_branch::ExportBranch;
use crate::export_branch_files::check_configuration_file;
use crate::file_checker::FileStatus;
use regex::Regex;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub fn export(
    exportbranch: &mut ExportBranch,
    source: PathBuf,
    destination: PathBuf,
    file_filters: &Vec<Regex>,
    only_copy_files: &Vec<Regex>,
) -> Result<()> {
    let (file_filters_regex, only_copy_files_regex) =
        check_configuration_file(&source, file_filters.to_owned(), only_copy_files.to_owned());

    let destination = format_lower(
        destination,
        &exportbranch.destination,
        exportbranch.configuration.lower(),
    );

    if !destination.exists() {
        fs::create_dir_all(&destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            export_directory(
                exportbranch,
                entry_path,
                &destination,
                &file_filters_regex,
                &only_copy_files_regex,
            )?;
        } else if entry_path.is_file() {
            export_file(
                exportbranch,
                entry_path,
                destination.join(entry.file_name()),
                &file_filters_regex,
                &only_copy_files_regex,
            )?;
        }
    }

    Ok(())
}

fn format_lower(destination: PathBuf, raw_destination: &PathBuf, lower: bool) -> PathBuf {
    if !lower {
        return destination;
    }

    let raw = raw_destination.to_str().unwrap();

    raw_destination
        .join(
            {
                let mut formated = destination.to_str().unwrap().replace(raw, "");

                if formated.starts_with(std::path::MAIN_SEPARATOR) {
                    formated.remove(0);
                }

                formated
            }
            .to_lowercase(),
        )
        .to_path_buf()
}

fn export_directory(
    exportbranch: &mut ExportBranch,
    source: PathBuf,
    destination: &PathBuf,
    file_filters: &Vec<Regex>,
    only_copy_files: &Vec<Regex>,
) -> Result<()> {
    let entry_file_name = source.file_name().unwrap();
    let dest_path = destination.join(entry_file_name);
    export(
        exportbranch,
        source.to_path_buf(),
        dest_path,
        file_filters,
        only_copy_files,
    )
}

fn export_file(
    exportbranch: &mut ExportBranch,
    source_file: PathBuf,
    destination_file: PathBuf,
    file_filters: &Vec<Regex>,
    only_copy_files: &Vec<Regex>,
) -> Result<()> {
    if !file_match(&source_file, file_filters) {
        return Ok(());
    }

    let destination_file = format_lower(
        destination_file,
        &exportbranch.destination,
        exportbranch.configuration.lower(),
    );

    match file_need_update(&source_file, exportbranch, &destination_file) {
        FileStatus::UpToDate => {
            return Ok(());
        }
        FileStatus::Modified(system_time) => {
            exportbranch
                .file_checker
                .add_file(&source_file, system_time);
        }
    }

    let only_copy = file_match(&source_file, only_copy_files);

    print_file(only_copy, &source_file, &destination_file);

    if only_copy {
        match fs::copy(&source_file, destination_file) {
            Err(err) => {
                eprint!("Error copying file: {}", err);
                exportbranch.file_checker.remove_file(&source_file);
            }
            _ => {}
        };
    } else {
        match convert_file(&source_file, destination_file) {
            Err(err) => {
                eprint!("Error copying file: {}", err);
                exportbranch.file_checker.remove_file(&source_file);
            }
            _ => {}
        };
    }

    Ok(())
}

fn print_file(only_copy: bool, entry_path: &PathBuf, dest_path: &PathBuf) {
    println!(
        "{}\r\nsource.....: {}\r\ndestination: {}\r\n",
        {
            if only_copy {
                "copying..."
            } else {
                "converting..."
            }
        },
        source_path_display(&entry_path.to_string_lossy()),
        dest_path.to_string_lossy()
    );
}

#[cfg(target_os = "windows")]
fn source_path_display(entry_path: &str) -> &str {
    &entry_path[4..]
}

#[cfg(target_os = "linux")]
fn source_path_display(entry_path: &str) -> &str {
    entry_path
}

fn file_match(file: &PathBuf, file_filters: &Vec<Regex>) -> bool {
    let file_name = file.file_name().unwrap().to_str().unwrap();

    for file_filter in file_filters {
        if file_filter.is_match(&file_name) {
            return true;
        }
    }

    false
}

fn file_need_update(
    file: &PathBuf,
    exportbranch: &mut ExportBranch,
    destination_file: &PathBuf,
) -> FileStatus {
    let configuration = exportbranch.configuration;

    if configuration.reload() || (configuration.md5() && !destination_file.exists()) {
        return exportbranch.file_checker.force_update(file);
    }

    exportbranch.file_checker.check(file)
}
