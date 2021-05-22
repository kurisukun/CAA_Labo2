use sodiumoxide::crypto::auth::hmacsha256::Tag;

pub struct Challenge{
    value: Tag,
}

impl Challenge{
    pub fn get_tag(self) -> Tag{
        self.value
    }

    pub fn new(value: Tag) -> Challenge{
        Challenge{value}
    }
}