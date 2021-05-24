use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use lazy_static::lazy_static;
use rand::Rng;
use sodiumoxide::crypto::auth::hmacsha256::authenticate;
use sodiumoxide::crypto::auth::hmacsha256::Key;
use sodiumoxide::crypto::auth::hmacsha256::Tag;
use sodiumoxide::crypto::pwhash::argon2id13;
use sodiumoxide::crypto::pwhash::argon2id13::Salt;
use sodiumoxide::crypto::secretbox;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::elements::files::Files;
use crate::elements::{challenge::Challenge, google_secret::GoogleSecret, user::User};

const TOKEN_PATH: &str = "./src/server/two_factors.json";
const FILES_PATH: &str = "./src/server/files/";
const VAULT_PATH: &str = "./src/server/vault/";

lazy_static! {
    static ref DB: HashMap<String, User> = {
        const USERNAME: &str = "kurisukun";
        const PASSWORD: &str = "MyPassword";
        let salt = argon2id13::gen_salt();
        let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
        let secretbox::Key(ref mut kb) = k;
        argon2id13::derive_key(
            kb,
            PASSWORD.as_bytes(),
            &salt,
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        )
        .unwrap();

        let mut map = HashMap::new();
        map.insert(USERNAME.to_string(), User::new(*kb, salt));

        map
    };
}

pub fn check_user(username: &str) -> Result<(Salt, u64, Challenge), ()> {
    match DB.get::<str>(username) {
        Some(user) => {
            let mut rng = rand::thread_rng();
            let password = user.get_password();
            let salt = user.get_salt();
            let challenge: u64 = rng.gen();

            let computed_challenge = compute_challenge(challenge, password);

            //Sending challenge for user
            Ok((*salt, challenge, computed_challenge))
        }
        None => Err(()),
    }
}

fn compute_challenge(challenge: u64, kb: [u8; 32]) -> Challenge {
    let tag = authenticate(&challenge.to_be_bytes(), &Key::from_slice(&kb).unwrap());

    Challenge::new(tag)
}

pub fn check_response(user_tag: Tag, challenge_tag: Tag) -> bool {
    user_tag == challenge_tag
}

pub fn verify_secret(input_token: &str) -> bool {
    let secret = get_secret();
    let google_auth = GoogleAuthenticator::new();
    google_auth.verify_code(secret.as_str(), input_token, 0, 0)
}

fn get_secret() -> String {
    let secret = fs::read_to_string(TOKEN_PATH).expect("Unable to read file");

    let deserialized: GoogleSecret = serde_json::from_str(&secret).unwrap();
    deserialized.get_secret()
}

fn gen_secret(auth: &GoogleAuthenticator) -> String {
    auth.create_secret(32)
}

pub fn begin_two_factors() {
    println!("\n### Authentication with two factors ###");

    if !Path::new(TOKEN_PATH).exists() {
        let google_auth = GoogleAuthenticator::new();
        let secret = gen_secret(&google_auth);

        let google_secret = GoogleSecret::new(&secret);

        let serialized = serde_json::to_string(&google_secret).unwrap();
        fs::write(TOKEN_PATH, serialized).expect("Unable to write file");

        let qr_code = google_auth.qr_code_url(
            secret.as_str(),
            "qr_code",
            "caa_labo2_account",
            400,
            400,
            ErrorCorrectionLevel::High,
        );

        println!("Or go to this link and scan the QR Code: {}", qr_code);
    }
}

pub fn list_files() {
    let paths = fs::read_dir(VAULT_PATH).unwrap();

    println!("List of the files:");
    for path in paths {
        let p = path.unwrap().path().display().to_string();
        let pos = p.rfind('/').unwrap() + 1;
        let filename = p[pos..p.len()].to_string();
        println!("{}", filename);
    }
    print!("\n");
}

pub fn upload_file(
    file_name: &str,
    file: &str,
    salt: Salt,
    nonce_enc: [u8; 12],
    nonce_enc_filename: [u8; 12],
) {
    println!("Uploading file.....");
    let new_path = VAULT_PATH.to_owned() + file_name;
    fs::rename(file, new_path).unwrap();

    let f = Files::new(file_name.to_string(), salt, nonce_enc, nonce_enc_filename);
    let db_file = FILES_PATH.to_owned() + file_name + ".json";
    println!("File {} created", db_file);
    let serialized = serde_json::to_string(&f).unwrap();
    fs::write(db_file, serialized).expect("Unable to write file");
}

pub fn send_file_infos(filename: &str) -> Result<(String, Files), std::io::Error> {
    let path = VAULT_PATH.to_owned() + filename;
    let content = fs::read_to_string(&path)?;
    let info_path = FILES_PATH.to_owned() + filename + ".json";
    let f = fs::read_to_string(info_path).expect("Unable to read file");
    let file: Files = serde_json::from_str(&f).unwrap();

    Ok((content, file))
}
