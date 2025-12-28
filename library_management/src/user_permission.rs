use std::fmt;

pub struct BookManagePermission {
    perm_insert: bool,
    perm_modify: bool,
    perm_delete: bool,
}

pub struct UserManagePermission {
    perm_add_user: bool,
    perm_remove_user: bool,
    perm_modify_user: bool,
}

pub struct BookServicePermission {
    perm_send_notify: bool,
    perm_update_service: bool,
    perm_borrow_book: bool
}

impl fmt::Display for BookManagePermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "insert: {}, modify: {}, delete: {}",
        self.perm_insert,
        self.perm_modify,
        self.perm_delete)
    }
}

impl fmt::Display for UserManagePermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "add_user: {}, remove_user: {}, modify_user: {}",
                self.perm_add_user,
                self.perm_remove_user,
                self.perm_modify_user)
    }
}

impl fmt::Display for BookServicePermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "send_notify: {}, update_service: {}, borrow_book: {}",
        self.perm_send_notify,
        self.perm_update_service,
        self.perm_borrow_book)
    }
}

impl BookManagePermission {
    pub fn new(insert_book: bool, modify_book: bool, delete_book:bool ) -> Self {
        Self {
            perm_insert: insert_book,
            perm_modify: modify_book,
            perm_delete: delete_book
        }
    }

    pub fn set_insert_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_insert = allow;
        self
    }

    pub fn set_modify_perm(&mut self, allow:  bool) -> &mut Self {
        self.perm_modify = allow;
        self
    }

    pub fn set_delete_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_delete = allow;
        self
    }
}

impl UserManagePermission {
    pub fn new(add_user: bool, remove_user: bool, modify_user: bool) -> Self {
        Self {
            perm_add_user: add_user,
            perm_remove_user: remove_user,
            perm_modify_user: modify_user
        }
    }

    pub fn set_add_user_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_add_user = allow;
        self
    }

    pub fn set_remove_user_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_remove_user = allow;
        self
    }

    pub fn set_modify_user_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_modify_user = allow;
        self
    }
}

impl BookServicePermission {
    pub fn new(send_notify: bool, update_ser: bool, borrow_book: bool) -> Self {
        Self {
            perm_send_notify: send_notify,
            perm_update_service: update_ser,
            perm_borrow_book: borrow_book
        }
    }

    pub fn set_send_noitfy_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_send_notify = allow;
        self
    }

    pub fn set_update_service_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_update_service = allow;
        self
    }

    pub fn set_borrow_book_perm(&mut self, allow: bool) -> &mut Self {
        self.perm_borrow_book = allow;
        self
    }
}
