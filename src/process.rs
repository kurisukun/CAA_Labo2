use crate::client::client::{send_username, send_response, send_token};
use crate::server::server::{check_user, check_response, begin_two_factors, verify_secret};

pub fn challenge_response(){

    let username = send_username();
    
    match check_user(username.as_str()){
        Ok(t) => {
            let response = send_response(t.0, t.1);
            let challenge_computed = t.2.get_tag();

            if check_response(response, challenge_computed){
                println!("You are now authentified!");
            }
            else{
                println!("Wrong tag!");
            }
        }
        Err(_) => {
            println!("Error: user not found");
        }
    }
}

pub fn two_factors(){
    begin_two_factors();
    let token = send_token();
    if verify_secret(&token){
        println!("You are connected! You can now look for your files");
        return;
    }

    println!("Error: the token is incorrect");
}
