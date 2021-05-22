mod client;
mod server;
mod process;
mod elements;
mod validation;

use crate::process::{challenge_response, two_factors, ask_command};
use crate::process::Menu;
use crate::server::server::list_files;

fn main() {
    
    //challenge_response();
    //two_factors();


    loop {
        match ask_command() {
            Menu::ListFiles => {
                list_files()
            }
            Menu::Upload => {    
                
            }
            Menu::Download => {
                
            },
            Menu::Quit => return,
        }
    }
    
}