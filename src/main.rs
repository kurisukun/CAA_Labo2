mod client;
mod server;
mod process;
mod elements;

use crate::process::{challenge_response, two_factors};

fn main() {
    
    challenge_response();
    two_factors();
}