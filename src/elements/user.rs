use sodiumoxide::crypto::pwhash::argon2id13::Salt;


pub struct User{
    password: [u8; 32],
    salt: Salt,
}

impl User{
    pub fn new(password: [u8; 32], salt: Salt) -> User{
        User{password, salt}
    }

    pub fn get_password(&self) -> [u8;32]{
        self.password
    }

    pub fn get_salt(&self) -> &Salt{
        &self.salt
    }

}