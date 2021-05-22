use sodiumoxide::crypto::pwhash::argon2id13::Salt;


pub struct User{
    username: String, 
    password: [u8; 32],
    salt: Salt,
}

impl User{
    pub fn new(username: String, password: [u8; 32], salt: Salt) -> User{
        User{username, password, salt}
    }

    pub fn get_username(&self) -> &String{
        &self.username
    }

    pub fn get_password(&self) -> [u8;32]{
        self.password
    }

    pub fn get_salt(&self) -> &Salt{
        &self.salt
    }

}