
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GoogleSecret{
    secret: String,
}

impl GoogleSecret{

    pub fn new(secret: &String) -> GoogleSecret{
        GoogleSecret{secret: secret.to_string()}
    } 

    pub fn get_secret(self) -> String{
        self.secret
    }
}