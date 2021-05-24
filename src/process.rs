use read_input::prelude::*;
use std::str::FromStr;
use strum_macros::EnumString;

use crate::client::client::{ask_user, send_response};
use crate::server::server::{begin_two_factors, check_response, check_user, verify_secret};
use crate::validation::validation::check_cmd_syntax;

#[derive(PartialEq, Debug, EnumString)]
pub enum Menu {
    #[strum(serialize = "List", serialize = "list", serialize = "1")]
    ListFiles,
    #[strum(serialize = "Upload", serialize = "upload", serialize = "2")]
    Upload,
    #[strum(serialize = "Download", serialize = "download", serialize = "3")]
    Download,
    #[strum(serialize = "Quit", serialize = "quit", serialize = "4")]
    Quit,
}

fn menu() {
    println!("1. List the files in the vault ({:?})", Menu::ListFiles);
    println!("2. Upload a file into the vault ({:?})", Menu::Upload);
    println!(
        "3. Read content of a file in the vault ({:?})",
        Menu::Download
    );
    println!("4. Quit the program ({:?})", Menu::Quit);
    println!("\n");
}

pub fn ask_command() -> Menu {
    menu();

    loop {
        let input = input()
            .msg("Enter your command: ")
            .add_err_test(
                |command: &String| check_cmd_syntax(command),
                "Invalid command entered!",
            )
            .get();

        match Menu::from_str(&input) {
            Ok(_) => {
                return Menu::from_str(&input).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    }
}

pub fn challenge_response() -> bool {
    let username = ask_user("Enter your username: ");

    match check_user(username.as_str()) {
        Ok(t) => {
            let response = send_response(t.0, t.1);
            let challenge_computed = t.2.get_tag();

            if check_response(response, challenge_computed) {
                println!("*Challenge-response passed*");
                return true;
            } else {
                println!("Error: login failed");
                return false;
            }
        }
        Err(_) => {
            println!("Error: login failed");
            return false;
        }
    }
}

pub fn two_factors() -> bool {
    begin_two_factors();
    let token = ask_user("Enter your token: ");
    if verify_secret(&token) {
        println!("You are connected! You can now look for your files");
        return true;
    }

    println!("Error: the token is incorrect");
    false
}
