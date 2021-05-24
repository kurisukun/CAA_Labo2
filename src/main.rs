mod client;
mod server;
mod process;
mod elements;
mod validation;

use client::client::read_file;

use crate::process::{challenge_response, two_factors, ask_command};
use crate::process::Menu;
use crate::server::server::{list_files, upload_file, send_file_infos};
use crate::client::client::{send_file_to_upload, ask_file};

fn main() {
    
    //challenge_response();
    //two_factors();


    loop {
        match ask_command() {
            Menu::ListFiles => {
                list_files()
            }
            Menu::Upload => {    
                let (filename, path, salt, nonce_enc, nonce_enc_filename) = send_file_to_upload();
                if filename == "" && path == ""{
                    continue;
                }
 
                upload_file(filename.as_str(), path.as_str(), salt, nonce_enc, nonce_enc_filename);
            }
            Menu::Download => {
                let filename = ask_file();
                match send_file_infos(filename.as_str()){
                    Ok(dl) => {
                        let (content, file_info) = dl; 
                        read_file(file_info, content.as_str());
                    },
                    Err(_) => {
                        println!("Error: file does not exist!");
                        continue;
                    }
                }

            },
            Menu::Quit => return,
        }
    }
    
}