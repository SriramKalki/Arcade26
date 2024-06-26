use clap::{Arg, App};
use std::fs;
use std::io::{self, Read, Write};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use rand::rngs::OsRng;
use rand::RngCore;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

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

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

fn decrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(data).unwrap()
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

        let key = hex!("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f");
        let mut iv = [0u8; 16];
        OsRng.fill_bytes(&mut iv);

    if let Some(file) = matches.value_of("encrypt") {
        println!("Encrypting file: {}", file);
        let contents = read_file(file).expect("Failed to read file");
        let encrypted_data = encrypt(&contents, &key, &iv);
        let mut encrypted_file = Vec::new();
        encrypted_file.extend_from_slice(&iv);
        encrypted_file.extend_from_slice(&encrypted_data);
        write_file("encrypted.bin", &encrypted_file).expect("Failed to write file");
    } else if let Some(file) = matches.value_of("decrypt") {
        println!("Decrypting file: {}", file);
        let contents = read_file(file).expect("Failed to read file");
        let (iv, data) = contents.split_at(16);
        let decrypted_data = decrypt(data, &key, iv);
        write_file("decrypted.txt", &decrypted_data).expect("Failed to write file");
    } else {
        eprintln!("Invalid... Use --help for more information.");
    }
}
