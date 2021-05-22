use hmacsha256::authenticate;
use read_input::prelude::*;
use sodiumoxide::crypto::pwhash::argon2id13::Salt;
use sodiumoxide::crypto::auth::hmacsha256;
use sodiumoxide::crypto::pwhash::argon2id13;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::auth::hmacsha256::{Key, Tag};


pub fn send_username() -> String{

    let username: String = input()
        .msg("Please enter your username: ")
        .get();

    username
}

pub fn send_response(salt: Salt, challenge: u64) -> Tag{

    let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
    let secretbox::Key(ref mut kb) = k;

    let password: String = input()
    .msg("Please enter your password: ")
    .get();

    argon2id13::derive_key(kb, password.as_bytes(), &salt, argon2id13::OPSLIMIT_INTERACTIVE, argon2id13::MEMLIMIT_INTERACTIVE).unwrap();


    let tag = authenticate(&challenge.to_be_bytes(), &Key::from_slice(kb).unwrap());

    tag
}

pub fn send_token() -> String{
    let input_token: String = input()
    .repeat_msg("Please enter your two factors authentication token: ")
    /*.add_err_test(
        |m: &String| syntatic_validation_google_token(m),
        "Error: the format is not respected (only 6 numbers) ",
    )*/
    .get();

    input_token
}