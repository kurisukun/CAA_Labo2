mod client;
mod elements;
mod process;
mod server;
mod validation;

use client::client::read_file;

use crate::client::client::{ask_file, send_file_to_upload};
use crate::process::Menu;
use crate::process::{ask_command, challenge_response, two_factors};
use crate::server::server::{list_files, send_file_infos, upload_file};

fn main() {
    if !challenge_response() {
        return;
    }

    if !two_factors() {
        return;
    }

    loop {
        match ask_command() {
            Menu::ListFiles => list_files(),
            Menu::Upload => {
                let (filename, path, salt, nonce_enc, nonce_enc_filename) = send_file_to_upload();
                if filename == "" && path == "" {
                    continue;
                }

                upload_file(
                    filename.as_str(),
                    path.as_str(),
                    salt,
                    nonce_enc,
                    nonce_enc_filename,
                );
            }
            Menu::Read => {
                let filename = ask_file();
                match send_file_infos(filename.as_str()) {
                    Ok(dl) => {
                        let (content, file_info) = dl;
                        read_file(file_info, content.as_str());
                    }
                    Err(_) => {
                        println!("Error: file does not exist!");
                        continue;
                    }
                }
            }
            Menu::Quit => return,
        }
    }
}
