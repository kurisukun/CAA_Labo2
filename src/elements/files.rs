use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::pwhash::argon2id13::Salt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Files{
    name: String,
    salt: Salt,
    nonce_enc: [u8; 12],
    nonce_enc_filename: [u8; 12],
}

impl Files{

    pub fn new(name: String, salt: Salt, nonce_enc: [u8; 12], nonce_enc_filename: [u8; 12]) -> Files{
        Files{name, salt, nonce_enc, nonce_enc_filename}
    } 

    pub fn get_name(&self) -> &String{
        &self.name
    }


    pub fn get_salt(&self) -> Salt{
        self.salt
    }


    pub fn get_nonce_enc(&self) -> [u8;12]{
        self.nonce_enc
    }

    pub fn get_nonce_enc_filename(&self) -> [u8;12]{
        self.nonce_enc_filename
    }
}