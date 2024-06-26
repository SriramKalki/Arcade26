use clap::{Arg, App};
use std::fs;
use std::io::{self, Read, Write};

fn read_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}

fn write_file(file_path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(data)?;

    Ok(())
}

fn main() {
    let matches = App::new("File Encryptor")
        .version("1.0")
        .author("Sriram Kalki")
        .about("Encryption/Decryption of Files")
        .arg(Arg::new("encrypt")
            .short('e')
            .long("encrypt")
            .value_name("FILE")
            .about("Encrypts")
            .takes_value(true))
        .arg(Arg::new("decrypt")
            .short('d')
            .long("decrypt")
            .value_name("FILE")
            .about("Decrypts")
            .takes_value(true))
        .get_matches();

    if let Some(file) = matches.value_of("encrypt") {
        println!("Encrypting file: {}", file);
    } else if let Some(file) = matches.value_of("decrypt") {
        println!("Decrypting file: {}", file);
    } else {
        eprintln!("Invalid... Use --help for more information.");
    }
}
