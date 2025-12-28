pub struct UserLoginCredential {
    user_name: String,
    password: String
}

impl UserLoginCredential {
    pub fn new(user_name: String, password: String) -> Self {
        Self {
            user_name,
            password
        }
    }

    pub fn verify_credentials(&self, user_name: String, password: String) -> Result<String, String>{
        if self.user_name == user_name {
            if self.password == password {
                Ok("Authentication successfully".to_string())
            } else {
                Err("Authentication failed, invalid password".to_string())
            }
        } else {
            Err("Authentication failed, incorrect  user name".to_string())
        }
    }
}