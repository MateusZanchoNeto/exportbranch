use regex::Regex;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::path::PathBuf;

pub fn check_configuration_file(
    directory: &PathBuf,
    file_filters: Vec<Regex>,
    only_copy_files: Vec<Regex>,
) -> (Vec<Regex>, Vec<Regex>) {
    let config_only_copy = read_config_file(directory, "extecoesapenascopiar.exb");
    let config_do_not_convert = read_config_file(directory, "naoconverteacentos.exb");

    (
        check_filters(&config_only_copy, file_filters),
        check_only_copy(config_only_copy, config_do_not_convert, only_copy_files),
    )
}

pub fn checked_to_regex(checked: Vec<String>) -> Vec<Regex> {
    let mut regex: Vec<Regex> = vec![];

    for file in checked {
        let file = file.replace(".", "\\.").replace("*", ".*");
        regex.push(Regex::new(&file).unwrap());
    }

    regex
}

fn check_filters(
    config_only_copy: &Result<Vec<String>>,
    only_copy_files: Vec<Regex>,
) -> Vec<Regex> {
    match config_only_copy {
        Ok(checked) => checked_to_regex(checked.clone()),
        Err(_) => only_copy_files,
    }
}

fn check_only_copy(
    config_only_copy: Result<Vec<String>>,
    config_do_not_convert: Result<Vec<String>>,
    only_copy_files: Vec<Regex>,
) -> Vec<Regex> {
    if config_only_copy.is_err() && config_do_not_convert.is_err() {
        return only_copy_files;
    }

    let mut checked: Vec<String> = vec![];

    if let Ok(mut files) = config_only_copy {
        checked.append(&mut files);
    }

    if let Ok(mut files) = config_do_not_convert {
        checked.append(&mut files);
    }

    checked_to_regex(checked)
}

fn read_config_file(directory: &PathBuf, config_file: &str) -> Result<Vec<String>> {
    let file_name: PathBuf = directory.join(config_file);

    if !file_name.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("File not found: {:?}", file_name),
        ));
    }

    let mut file: fs::File = fs::File::open(file_name)?;
    let mut config_file_buffer: String = String::new();

    file.read_to_string(&mut config_file_buffer)?;

    let config_file_vec: Vec<&str> = config_file_buffer.split(";").collect::<Vec<&str>>();
    let mut config: Vec<String> = vec![];

    for file in config_file_vec {
        let file_filter = file.replace(char::from(10), "").replace(char::from(13), "");

        if !file_filter.is_empty() {
            config.push(file_filter);
        }
    }

    Ok(config)
}
