use std::fmt;

// Creating enum for role
#[derive(Clone,PartialEq)]
pub enum Role{
    User,
    Admin
}

// Convert Input String to Enum
impl Role{
    pub fn from_str(role: &str) -> Role{
        match role{
            "Admin" => Role::Admin,
            _ => Role::User
        }
    }
}

// Convert Enum to String
// Useful for creating a token and such
impl fmt::Display for Role{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Role::Admin => write!(f,"Admin"),
            Role::User => write!(f,"User")
        }
    }
}



