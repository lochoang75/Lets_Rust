use std::fmt;
use crate::user_enum:: UserAccountLevel;
use crate::login_credentials::UserLoginCredential;
use crate::user_permission::{UserManagePermission, BookManagePermission, BookServicePermission};

pub struct LibraryUser {
    id: u32,
    name: String,
    account_status: u8,
    account_level: UserAccountLevel,
    user_manage_perm: UserManagePermission,
    book_manage_perm: BookManagePermission,
    book_service_perm: BookServicePermission,
    user_login: UserLoginCredential
}

impl fmt::Display for LibraryUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "{}, {}, {}\n", self.id, self.name, self.account_status);
        let _ = write!(f, "user_manage_perm: {}\n", self.user_manage_perm);
        let _ = write!(f, "book_manage_perm: {}\n", self.book_manage_perm);
        write!(f, "book_service_perm: {}\n", self.book_service_perm)
    }
}

impl LibraryUser {
    pub fn new(user_id: u32, name: &str) -> Self {
        Self {id: user_id,
            name: name.to_string(),
            account_status: 0,
            account_level: UserAccountLevel::LevelNormal,
            user_manage_perm: UserManagePermission::new(false, false, false),
            book_manage_perm: BookManagePermission::new(false, false, false),
            book_service_perm: BookServicePermission::new(false, false, true),
            user_login: UserLoginCredential::new("abc".to_string(), "abc".to_string())
        }
    }

    pub fn create_admin(user_id:  u32, name: &str) -> Self {
        Self {id: user_id,
            name: name.to_string(),
            account_status: 0,
            account_level: UserAccountLevel::LevelAdmin,
            user_manage_perm: UserManagePermission::new(true,true, true),
            book_manage_perm: BookManagePermission::new(true, true, true),
            book_service_perm: BookServicePermission::new(true, true, false),
            user_login: UserLoginCredential::new("abc".to_string(), "abc".to_string())
        }
    }
}
