use crate::models::user::{User, UserRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedUserCookie {
    pub login: String,
    pub role: UserRole,
}

impl From<User> for LoggedUserCookie {
    fn from(user: User) -> Self {
        Self {
            login: user.login,
            role: user.role,
        }
    }
}
