use std::fmt::{Debug, Display, Formatter};

fn main() {
    let tom = update_user(User {
        username: String::from("tom"),
        email: String::from("example@test.com"),
        active: true,
        sign_in_count: 10,
    });

    print!("user info: \n\
    name: {}\n\
    email: {}\n\
    active: {}\n\
    sign_in_count: {}", tom.username, tom.email, tom.active, tom.sign_in_count)
}

fn update_user(tom: User) -> User {
    User {
        username: String::from("Tim"),
        sign_in_count: 0,
        ..tom
    }
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {}, active: {}, sign_in_count: {} }}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}
