use aead::{generic_array::GenericArray, Aead, Error, NewAead};
use aes_gcm::Aes256Gcm;
use hmacsha256::authenticate;
use rand::RngCore;
use read_input::prelude::*;
use sodiumoxide::crypto::auth::hmacsha256;
use sodiumoxide::crypto::auth::hmacsha256::{Key, Tag};
use sodiumoxide::crypto::pwhash::argon2id13;
use sodiumoxide::crypto::pwhash::argon2id13::Salt;
use sodiumoxide::crypto::secretbox;
use std::fs;
use std::path::Path;
use std::str;
use std::{thread, time};

use crate::elements::files::Files;

const FILES_PATH: &str = "./src/client/files/";

pub fn ask_user(msg: &str) -> String {
    let input: String = input().repeat_msg(msg).get();

    input
}

pub fn send_response(salt: Salt, challenge: u64) -> Tag {
    let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
    let secretbox::Key(ref mut kb) = k;

    let password = ask_user("Enter your password: ");

    argon2id13::derive_key(
        kb,
        password.as_bytes(),
        &salt,
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    let tag = authenticate(&challenge.to_be_bytes(), &Key::from_slice(kb).unwrap());

    tag
}

fn list_files() {
    let paths = fs::read_dir(FILES_PATH).unwrap();

    println!("List of the files:");
    for path in paths {
        let p = path.unwrap().path().display().to_string();
        let pos = p.rfind('/').unwrap() + 1;
        let filename = p[pos..p.len()].to_string();
        println!("{}", filename);
    }
    print!("\n");
}

fn encrypt(text: &str, k: [u8; 32]) -> (String, [u8; 12]) {
    let aes_k = GenericArray::clone_from_slice(&k);
    let aead = Aes256Gcm::new(aes_k);

    let mut nonce = [0u8; 12];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut nonce);
    let aes_nonce = GenericArray::from_slice(&nonce);
    let ciphertext = aead
        .encrypt(aes_nonce, text.as_bytes())
        .expect("encryption failure!");

    let ciphertext_b64 = base64::encode(&ciphertext);

    (ciphertext_b64, nonce)
}

fn decrypt(ciphertext: &str, k: [u8; 32], nonce: [u8; 12]) -> Result<String, Error> {
    let aes_k = GenericArray::clone_from_slice(&k);
    let aead = Aes256Gcm::new(aes_k);
    let aes_nonce = GenericArray::from_slice(&nonce);

    let ciphertext = base64::decode(ciphertext).unwrap();
    let decryption_result = aead.decrypt(aes_nonce, ciphertext.as_ref());

    if let Err(e) = decryption_result {
        println!("Error: decryption failed!");
        return Err(e);
    }
    let plaintext = decryption_result.unwrap();

    //It's the only way I found to get a String from the input of the encryption and the decryption...
    let plaintext_b64 = base64::encode(&plaintext);
    let plaintext = base64::decode(&plaintext_b64).unwrap();
    Ok(str::from_utf8(&plaintext).unwrap().to_string())
}

pub fn ask_file() -> String {
    let input_file_name: String = input()
        .repeat_msg("Please choose the file you want:  ")
        .get();

    input_file_name
}

pub fn send_file_to_upload() -> (String, String, Salt, [u8; 12], [u8; 12]) {
    list_files();

    let input_file_name = ask_file();

    let full_path = FILES_PATH.to_owned() + &input_file_name.to_owned();

    let salt = argon2id13::gen_salt();

    if !Path::new(&full_path).exists() {
        println!("Error: Given file does not exist!");
        return ("".to_string(), "".to_string(), salt, [0; 12], [0; 12]);
    }

    //The user can choose any password
    let password = ask_user("Enter the password you want to encrypt the file: ");
    let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
    let secretbox::Key(ref mut kb) = k;
    argon2id13::derive_key(
        kb,
        password.as_bytes(),
        &salt,
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    let content = fs::read_to_string(full_path.clone()).unwrap();

    let (enc_content, nonce1) = encrypt(content.as_str(), *kb);

    let (enc_filename, nonce2) = encrypt(input_file_name.as_str(), *kb);
    let new_path = FILES_PATH.to_owned() + &enc_filename.to_owned();

    fs::rename(&full_path, &new_path).unwrap();
    //Ensures the program has the time to rename the file
    thread::sleep(time::Duration::from_secs(2));

    fs::write(&new_path, &enc_content).expect("Unable to write file");
    //Ensures the program has the time to write in the file
    thread::sleep(time::Duration::from_secs(2));

    (enc_filename, new_path, salt, nonce1, nonce2)
}

pub fn read_file(file: Files, content: &str) {
    let filename = file.get_name();
    let salt = file.get_salt();
    let nonce_enc = file.get_nonce_enc();
    let nonce_enc_filename = file.get_nonce_enc_filename();
    let password = ask_user("Enter the password you used to encrypt the file: ");

    let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
    let secretbox::Key(ref mut kb) = k;

    argon2id13::derive_key(
        kb,
        password.as_bytes(),
        &salt,
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    let dec_filename_res = decrypt(filename, *kb, nonce_enc_filename);

    if let Err(_) = dec_filename_res {
        return;
    }
    let dec_filename = dec_filename_res.unwrap();

    let dec_content_res = decrypt(content, *kb, nonce_enc);
    if let Err(_) = dec_content_res {
        return;
    }
    let dec_content = dec_content_res.unwrap();

    println!("File {}: ", dec_filename);
    println!("{}", dec_content);
}
