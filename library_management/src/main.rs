mod library_user;
mod user_enum;
mod login_credentials;
mod user_permission;
use library_user::LibraryUser;

fn main() {
    let user = LibraryUser::new(0, "elliot");
    let admin = LibraryUser::create_admin(0, "elliot");
    println!("User info is {}", user);
    println!("First admin is {}", admin)
}
