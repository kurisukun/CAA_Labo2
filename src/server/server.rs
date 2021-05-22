use std::collections::HashMap;
use lazy_static::lazy_static;
use rand::Rng;
use sodiumoxide::crypto::pwhash::argon2id13;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::pwhash::argon2id13::Salt;
use sodiumoxide::crypto::auth::hmacsha256::Key;
use sodiumoxide::crypto::auth::hmacsha256::Tag;
use sodiumoxide::crypto::auth::hmacsha256::authenticate;
use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use std::path::Path;
use std::fs;


use crate::elements::{user::User, challenge::Challenge, google_secret::GoogleSecret};

const USERNAME : &str = "kurisukun";
const PASSWORD : &str = "MyPassword";
const TOKEN_PATH: &str = "./src/server/two_factors.json";
const VAULT_PATH: &str = "./src/server/vault";

lazy_static! {

    static ref DB : HashMap<String, User> = {

        let salt = argon2id13::gen_salt();
        let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
        let secretbox::Key(ref mut kb) = k;
        argon2id13::derive_key(kb, PASSWORD.as_bytes(), &salt, argon2id13::OPSLIMIT_INTERACTIVE, argon2id13::MEMLIMIT_INTERACTIVE).unwrap();

        let mut map = HashMap::new();
        map.insert(USERNAME.to_string(), User::new(USERNAME.to_string(), *kb, salt));

        map
    };
}


pub fn check_user(username: &str) -> Result<(Salt, u64, Challenge), ()>{
    
    match DB.get::<str>(username){
        Some(user) => {
            let mut rng = rand::thread_rng();
            let username = user.get_username();
            let password = user.get_password();
            let salt = user.get_salt();
            let challenge: u64 = rng.gen();


            let computed_challenge = compute_challenge(challenge, password);

            //Sending challenge for user
            Ok((*salt, challenge, computed_challenge))
        }
        None => {
            Err(())
        }
    }
}

fn compute_challenge(challenge: u64, kb: [u8; 32]) -> Challenge{
    let tag = authenticate(&challenge.to_be_bytes(), &Key::from_slice(&kb).unwrap());
    
    Challenge::new(tag)
}

pub fn check_response(user_tag: Tag, challenge_tag: Tag) -> bool{
    user_tag == challenge_tag
}

pub fn verify_secret(input_token: &str) -> bool {
    let secret = get_secret();
    let google_auth = GoogleAuthenticator::new();
    google_auth.verify_code(secret.as_str(), input_token, 0, 0)
}

fn get_secret() -> String{
    let secret = fs::read_to_string(TOKEN_PATH).expect("Unable to read file");

    let deserialized: GoogleSecret = serde_json::from_str(&secret).unwrap();
    deserialized.get_secret()
}

fn gen_secret(auth: &GoogleAuthenticator) -> String{
    auth.create_secret(32)
}

pub fn begin_two_factors() {
    println!("\n### Authentication with two factors ###");

    if !Path::new(TOKEN_PATH).exists(){
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

pub fn list_files(){
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