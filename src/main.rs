mod args;

use args::Args;
use clap::Parser;
use convert_case::Casing;
use log::error;
use postcard;
use std::{
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::PathBuf,
};

fn main() {
    let args = Args::parse();
    let last_used_path = PathBuf::from("/tmp/to_case_last_used.bin");
    let case = args.case.unwrap_or_else(|| {
        if !last_used_path.exists() {
            error!("You don't have a case selected, and no last used case is defined!");
            std::process::exit(1);
        }
        let file = File::open(&last_used_path).unwrap();
        let mut reader = BufReader::new(&file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).unwrap();
        postcard::from_bytes(&bytes).unwrap()
    });

    let word = args.word.unwrap_or_else(|| {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer
    });

    let output = match case {
        args::Case::CamelCase => word.to_case(convert_case::Case::Camel),
        args::Case::SnakeCase => word.to_case(convert_case::Case::Snake),
        args::Case::LowerCase => word.to_case(convert_case::Case::Lower),
        args::Case::UpperCase => word.to_case(convert_case::Case::Upper),
    };
    print!("{}", output);

    let case_bytes = postcard::to_allocvec(&case).unwrap();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&last_used_path)
        .unwrap();
    file.write_all(&case_bytes).unwrap();
}
