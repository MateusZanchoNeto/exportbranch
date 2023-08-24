mod configuration;
mod convert_file;
mod convertions;
mod export;
mod export_branch;
mod export_branch_files;
mod file_checker;
mod help;

use configuration::Configuration;
use export_branch::ExportBranch;
use file_checker::FileChecker;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let configuration = build_configuration();

    configuration.print();

    for source in configuration.source() {
        for destination in configuration.destination() {
            export(source, destination, &configuration);
        }
    }

    print_time_elapsed(timer);
}

fn build_configuration() -> Configuration {
    Configuration::build(&mut env::args()).unwrap_or_else(|err: String| {
        eprintln!("{}", err);
        std::process::exit(1);
    })
}

fn export(source: &String, destination: &String, configuration: &Configuration) {
    let source_path_buffer = source_path(&source);
    let destination_path_buffer = destination_path(&source_path_buffer, &destination);
    let mut file_checker = FileChecker::new(Path::new(&destination).to_path_buf());
    let mut export = ExportBranch::build(
        source_path_buffer,
        destination_path_buffer,
        &configuration,
        &mut file_checker,
    );

    export.perform_exporting();
}

fn source_path(source: &String) -> PathBuf {
    Path::new(&source).canonicalize().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    })
}

fn destination_path(source_path: &PathBuf, destination: &String) -> PathBuf {
    let mut ancestors = source_path.ancestors();

    let first = ancestors
        .next()
        .unwrap_or_else(|| {
            eprintln!("Not found source path ancestor");
            std::process::exit(1);
        })
        .file_name()
        .unwrap_or_else(|| {
            eprintln!("Not found source path ancestor");
            std::process::exit(1);
        });

    let second = ancestors
        .next()
        .unwrap_or_else(|| {
            eprintln!("Not found source path ancestor");
            std::process::exit(1);
        })
        .file_name()
        .unwrap_or_else(|| {
            eprintln!("Not found source path ancestor");
            std::process::exit(1);
        });

    Path::new(&destination).join(second).join(first)
}

fn print_time_elapsed(timer: Instant) {
    println!(
        "\r\n--------------------------\r\nTime elapsed: {:?} secs",
        timer.elapsed().as_secs()
    );
}
